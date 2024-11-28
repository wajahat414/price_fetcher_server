// websocket_client.rs
use futures::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use serde_json::json;
use tokio::sync::broadcast;
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};

#[derive(Serialize, Deserialize, Debug)]
pub struct DepthUpdate {
    e: String,                // Event type (depth update)
    E: u64,                   // Event time
    s: String,                // Symbol
    b: Vec<(String, String)>, // Bids (price, quantity)
    a: Vec<(String, String)>, // Asks (price, quantity)
}

pub async fn fetch_prices(symbols: Vec<String>, sender: broadcast::Sender<String>) {
    let url = "wss://stream.binance.com:9443/stream";
    let mut params = Vec::new();

    for symbol in symbols {
        let stream_name = format!("{}@depth@100ms", symbol);
        params.push(stream_name);
    }

    let subscribe_msg = json!({
        "method": "SUBSCRIBE",
        "params": params,
        "id": 1
    })
    .to_string();

    let (ws_stream, _) = connect_async(url).await.unwrap();
    let (mut write, mut read) = ws_stream.split();

    // Send subscription request
    write.send(Message::Text(subscribe_msg)).await.unwrap();

    // Process messages from WebSocket
    while let Some(message) = read.next().await {
        match message.unwrap() {
            Message::Text(txt) => {
                match serde_json::from_str::<serde_json::Value>(&txt) {
                    Ok(msg) => {
                        if let Some(data) = msg.get("data") {
                            let update: DepthUpdate = serde_json::from_value(data.clone()).unwrap();
                            let update_msg = serde_json::to_string(&update).unwrap();
                            // Broadcast to subscribers
                            if let Err(_) = sender.send(update_msg) {
                                eprintln!("Failed to send data to clients.");
                            }
                        }
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }
}
