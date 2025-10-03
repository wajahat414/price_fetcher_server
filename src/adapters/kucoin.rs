use crate::errors::ConnectorError;
use crate::traits::{RestApi, WebSocketApi};
use async_trait::async_trait;
use futures::{SinkExt, StreamExt};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;
use tokio_tungstenite::{connect_async, tungstenite::Message};

#[derive(Debug, Deserialize)]
pub struct KucoinWebSocketResponse {
    code: String,
    data: WebSocketData,
}
#[derive(Debug, Deserialize)]
struct WebSocketData {
    token: String,
    #[serde(rename = "instanceServers")]
    instance_servers: Vec<InstanceServer>,
}

#[derive(Debug, Deserialize)]
pub struct InstanceServer {
    endpoint: String,
    protocol: String,
    encrypt: bool,
    #[serde(rename = "pingInterval")]
    ping_interval: u64,
    #[serde(rename = "pingTimeout")]
    ping_timeout: u64,
}

// Public config returned by get_websocket_url to avoid exposing internal response types
pub struct KucoinWebSocketConfig {
    pub servers: Vec<InstanceServer>,
    pub token: String,
}

pub struct KucoinConnector {
    base_url: String,
    client: Client,
}
impl KucoinConnector {
    pub fn new() -> Self {
        Self {
            base_url: "https://api.kucoin.com".to_string(),
            client: Client::new(),
        }
    }
    pub async fn get_websocket_url(&self) -> Result<KucoinWebSocketConfig, ConnectorError> {
        let url = format!("{}/api/v1/bullet-public", self.base_url);

        let response = self.client.post(&url).send().await?;

        // Print the raw response status and body for debugging
        println!("Response Status: {}", response.status());

        let raw_body = response.text().await?;

        // Deserialize response and return the config
        let parsed: KucoinWebSocketResponse = serde_json::from_str(&raw_body)?;
        println!("Parsed Response: {:?}", parsed);

        Ok(KucoinWebSocketConfig {
            servers: parsed.data.instance_servers,
            token: parsed.data.token,
        })
    }

    pub async fn connect_and_subscribe(
        &self,
        servers: Vec<InstanceServer>,
        token: &str,
        symbols: Vec<String>,
    ) -> Result<(), ConnectorError> {
        if servers.is_empty() {
            return Err(ConnectorError::Other(
                "No WebSocket servers available".into(),
            ));
        }

        let server_url = &servers[0].endpoint;
        let full_url = format!("{}?token={}", server_url, token);
        print!("Connecting to web socket: {}", full_url);

        let (ws_stream, _) = connect_async(&full_url).await?;
        print!("Connected to web scoket");

        let (mut write, mut read) = ws_stream.split();

        let subscribe_message = json!({

            "id" : 1 ,
            "type": "subscribe",
            "topic": "/market/ticker:".to_owned() + &symbols.join(","),
            "privateChannel": false,
            "response": true

        });

        write
            .send(Message::Text(subscribe_message.to_string()))
            .await?;

        println!("subscribe to book ticker for symbols : {:?}", symbols);

        while let Some(msg) = read.next().await {
            match msg {
                Ok(Message::Text(text)) => {
                    let parsed: serde_json::Value = serde_json::from_str(&text)?;

                    if let Some(data) = parsed.get("data") {
                        println!("Received bookTicker update: {:?}", data);
                    }
                }
                Ok(Message::Ping(ping)) => {
                    write.send(Message::Pong(ping)).await?;
                }

                Err(e) => {
                    eprintln!("WebSocket error: {:?}", e);
                    break;
                }

                _ => {}
            }
        }

        return Err(ConnectorError::Other("WebSocket stream ended".into()));
    }
}

#[async_trait]
impl RestApi for KucoinConnector {
    type Error = ConnectorError;

    async fn ticker_price(&self, symbol: &str) -> Result<f64, Self::Error> {
        // KuCoin expects symbols like BTC-USDT
        let url = format!(
            "{}/api/v1/market/orderbook/level1?symbol={}",
            self.base_url, symbol
        );
        #[derive(Deserialize)]
        struct RespData {
            price: String,
        }
        #[derive(Deserialize)]
        struct Resp {
            data: RespData,
        }
        let resp: Resp = self.client.get(url).send().await?.json().await?;
        let price: f64 = resp
            .data
            .price
            .parse::<f64>()
            .map_err(|e| ConnectorError::Other(e.to_string()))?;
        Ok(price)
    }
}

#[async_trait]
impl WebSocketApi for KucoinConnector {
    type Error = ConnectorError;

    async fn subscribe_book_ticker<F>(
        &self,
        symbols: Vec<String>,
        on_update: F,
    ) -> Result<(), Self::Error>
    where
        F: Fn(String, (f64, f64), (f64, f64)) + Send + Sync + 'static,
    {
        let config = self.get_websocket_url().await?;

        // We implement a local loop similar to connect_and_subscribe, but invoking callback
        if config.servers.is_empty() {
            return Err(ConnectorError::Other(
                "No WebSocket servers available".into(),
            ));
        }
        let server_url = &config.servers[0].endpoint;
        let full_url = format!("{}?token={}", server_url, config.token);

        let (ws_stream, _) = connect_async(&full_url).await?;
        let (mut write, mut read) = ws_stream.split();

        let subscribe_message = json!({
            "id": 1,
            "type": "subscribe",
            "topic": format!("/market/ticker:{}", symbols.join(",")),
            "privateChannel": false,
            "response": true
        });
        write
            .send(Message::Text(subscribe_message.to_string()))
            .await?;

        let on_update = std::sync::Arc::new(on_update);

        while let Some(msg) = read.next().await {
            match msg {
                Ok(Message::Text(text)) => {
                    let parsed: serde_json::Value = serde_json::from_str(&text)?;
                    if let Some(data) = parsed.get("data") {
                        // KuCoin book ticker fields: bestBid, bestBidSize, bestAsk, bestAskSize
                        if let (Some(symbol), Some(bid), Some(bid_sz), Some(ask), Some(ask_sz)) = (
                            data.get("symbol").and_then(|v| v.as_str()),
                            data.get("bestBid").and_then(|v| v.as_str()),
                            data.get("bestBidSize").and_then(|v| v.as_str()),
                            data.get("bestAsk").and_then(|v| v.as_str()),
                            data.get("bestAskSize").and_then(|v| v.as_str()),
                        ) {
                            let bid_price = bid.parse::<f64>().unwrap_or_default();
                            let bid_qty = bid_sz.parse::<f64>().unwrap_or_default();
                            let ask_price = ask.parse::<f64>().unwrap_or_default();
                            let ask_qty = ask_sz.parse::<f64>().unwrap_or_default();
                            (on_update)(
                                symbol.to_string(),
                                (bid_price, bid_qty),
                                (ask_price, ask_qty),
                            );
                        }
                    }
                }
                Ok(Message::Ping(ping)) => {
                    write.send(Message::Pong(ping)).await?;
                }
                Err(e) => return Err(ConnectorError::Ws(e)),
                _ => {}
            }
        }
        Err(ConnectorError::Other("WebSocket stream ended".into()))
    }
}
