use axum::http::response;
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
    pub async fn get_websocket_url(&self) -> Result<(), Box<dyn std::error::Error>> {
        let url = format!("{}/api/v1/bullet-public", self.base_url);

        let response = self.client.post(&url).send().await?;

        // Print the raw response status and body for debugging
        println!("Response Status: {}", response.status());

        let raw_body = response.text().await?;

        // Attempt to deserialize if the response is successful
        match serde_json::from_str::<KucoinWebSocketResponse>(&raw_body) {
            Ok(parsed) => {
                println!("Parsed Response: {:?}", parsed);
                Ok(())
            }
            Err(e) => {
                eprintln!("Deserialization Error: {}", e);
                Err(Box::new(e))
            }
        }
    }

    pub async fn connect_and_subscribe(
        &self,
        servers: Vec<InstanceServer>,
        token: &str,
        symbols: Vec<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if servers.is_empty() {
            return Err("No WebSocket servers available".into());
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

        return Err("Something went wrong ".into());
    }
}
