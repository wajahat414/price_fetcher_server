// use futures::{SinkExt, StreamExt};
// use serde::Deserialize;
// use serde_json::Value;
// use std::error::Error;
// use tokio::net::TcpStream;
// use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};

// #[derive(Deserialize, Debug)]
// struct DepthUpdate {
//     e: String,                // Event type (e.g., depth update)
//     E: u64,                   // Event time
//     s: String,                // Symbol (e.g., BTCUSDT)
//     U: u64,                   // First update time
//     u: u64,                   // Final update time
//     b: Vec<(String, String)>, // Bid orders (price, quantity)
//     a: Vec<(String, String)>, // Ask orders (price, quantity)
// }

// #[tokio::main]
// async fn main() -> Result<(), Box<dyn Error>> {
//     let symbols = vec![
//         "btcusdt", "ethusdt", "bnbusdt", "adausdt", "xrpusdt", /* add more symbols here */
//     ];
//     let url = "wss://stream.binance.com:9443/stream"; // Binance WebSocket endpoint

//     // Build the subscription message for all symbols
//     let mut params = Vec::new();
//     for symbol in symbols {
//         let stream_name = format!("{}@depth@100ms", symbol); // depth at level 1 with 100ms updates
//         params.push(stream_name);
//     }

//     let subscribe_msg = serde_json::json!({
//         "method": "SUBSCRIBE",
//         "params": params,
//         "id": 1
//     })
//     .to_string();

//     // Connect to WebSocket server
//     let (ws_stream, _) = connect_async(url).await?;
//     println!("Connected to Binance WebSocket.");

//     let (mut write, mut read) = ws_stream.split();

//     // Send subscription request
//     write.send(Message::Text(subscribe_msg)).await?;

//     // Listen to incoming messages from WebSocket
//     while let Some(message) = read.next().await {
//         match message? {
//             Message::Text(text) => match serde_json::from_str::<Value>(&text) {
//                 Ok(msg) => {
//                     if let Some(stream_data) = msg.get("data") {
//                         if let Ok(update) =
//                             serde_json::from_value::<DepthUpdate>(stream_data.clone())
//                         {
//                             println!("Received depth update: {:?}", update);
//                         }
//                     }
//                 }
//                 Err(e) => eprintln!("Failed to parse message: {}", e),
//             },
//             Message::Binary(_) => {
//                 println!("Received binary message, ignoring.");
//             }
//             _ => {}
//         }
//     }

//     Ok(())
// }
