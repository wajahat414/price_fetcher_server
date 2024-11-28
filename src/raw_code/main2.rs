// use axum::{
//     extract::ws::{Message, WebSocket, WebSocketUpgrade},
//     response::IntoResponse,
//     routing::get,
//     Router,
// };

// use futures_util::{SinkExt, StreamExt};
// use log::{info, warn};
// use serde_json::Value;
// use std::{collections::HashMap, sync::Arc, time::Duration};
// use tokio::{signal, sync::Mutex};
// use tokio_tungstenite::{connect_async, tungstenite::protocol::Message as WsMessage};

// type SharedState = Arc<Mutex<HashMap<String, HashMap<String, String>>>>;

// #[tokio::main]
// async fn main() {
//     env_logger::init();

//     let shared_orderbook = Arc::new(Mutex::new(HashMap::new()));

//     let fetcher_state = shared_orderbook.clone();
//     tokio::spawn(async move {
//         connect_to_binance(fetcher_state).await;
//     });

//     let app = Router::new().route("/", get(index)).route(
//         "/ws",
//         get(move |ws: WebSocketUpgrade| handle_ws(ws, shared_orderbook.clone())),
//     );

//     let addr = "0.0.0.0:3000".parse().unwrap();
//     info!("Starting server on {}", addr);

//     axum::Server::bind(&addr)
//         .serve(app.into_make_service())
//         .with_graceful_shutdown(shutdown_signal())
//         .await
//         .unwrap();
// }

// async fn shutdown_signal() {
//     signal::ctrl_c()
//         .await
//         .expect("Failed to install Ctrl+C handler");
//     info!("Shutdown signal received");
// }

// async fn index() -> &'static str {
//     "Welcome to the Binance order book WebSocket server!"
// }

// async fn handle_ws(ws: WebSocketUpgrade, state: SharedState) -> impl IntoResponse {
//     ws.on_upgrade(move |socket| handle_socket(socket, state.clone()))
// }

// async fn handle_socket(mut socket: WebSocket, state: SharedState) {
//     loop {
//         let orderbook_data = state.lock().await.clone();
//         let orderbook_text = serde_json::to_string(&orderbook_data)
//             .unwrap_or_else(|_| "Error serializing orderbook data".into());

//         if socket.send(Message::Text(orderbook_text)).await.is_err() {
//             warn!("Client disconnected");
//             return;
//         }
//         tokio::time::sleep(Duration::from_secs(1)).await;
//     }
// }

// async fn connect_to_binance(state: SharedState) {
//     let url = "wss://ws-api.binance.com:443/ws-api/v3";
//     let subscribe_message = binance_subscribe_message();

//     if let Err(e) = connect_to_exchange("Binance", url, subscribe_message, state).await {
//         warn!("Failed to connect to Binance: {}", e);
//     }
// }

// async fn connect_to_exchange(
//     exchange_name: &str,
//     url: &str,
//     subscribe_message: WsMessage,
//     state: SharedState,
// ) -> Result<(), Box<dyn std::error::Error>> {
//     let (ws_stream, _) = connect_async(url).await?;
//     info!("Connected to {}", exchange_name);

//     let (mut write, mut read) = ws_stream.split();
//     write.send(subscribe_message).await?;

//     while let Some(Ok(message)) = read.next().await {
//         if let WsMessage::Text(data) = message {
//             if let Some(orderbook) = parse_binance_orderbook(&serde_json::from_str(&data)?) {
//                 let mut state_data = state.lock().await;
//                 state_data.insert(exchange_name.to_string(), orderbook);
//             }
//         }
//     }
//     Ok(())
// }

// fn binance_subscribe_message() -> WsMessage {
//     let symbols: Vec<&str> = vec!["btcusdt", "ethusdt", "bnbusdt"];
//     let params: Vec<String> = symbols
//         .iter()
//         .map(|symbol| format!("{}@bookTicker", symbol))
//         .collect();

//     let subscribe_message = serde_json::json!({
//         "method": "SUBSCRIBE",
//         "params": params,
//         "id": 1,
//     });
//     print!("subscription message{}", subscribe_message.to_string());

//     WsMessage::Text(subscribe_message.to_string())
// }

// fn parse_binance_orderbook(json: &Value) -> Option<HashMap<String, String>> {
//     let bid_price = json.get("b").and_then(Value::as_str)?;
//     let ask_price = json.get("a").and_then(Value::as_str)?;

//     Some(HashMap::from([
//         ("bid_price".to_string(), bid_price.to_string()),
//         ("ask_price".to_string(), ask_price.to_string()),
//     ]))
// }
