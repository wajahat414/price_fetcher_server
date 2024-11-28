// // main.rs
// use crate::adapters::binance::binance_ws;
// use crate::data_aggregator::DataAggregator;
// use crate::pub_sub::create_pubsub_channel;
// use crate::websocket_client::fetch_prices;
// use crate::websocket_server::start_server;
// use std::net::SocketAddr;
// use tokio::sync::broadcast;

// mod data_aggregator;

// mod adapters;

// mod models;
// mod pub_sub;
// mod websocket_client;
// mod websocket_server;

// #[tokio::main]
// async fn main() {
//     // Create the pub/sub channel for communication between the WebSocket client and the server
//     let pubsub_channel = create_pubsub_channel();

//     // List of symbols to fetch from Binance (max 50 symbols per connection)
//     let symbols: Vec<String> = vec!["btcusdt", "ethusdt", "bnbusdt"]
//         .into_iter()
//         .map(|s| s.to_string())
//         .collect();
//     // Start WebSocket client (fetch prices from Binance)
//     tokio::spawn(fetch_prices(symbols, pubsub_channel.clone()));

//     // Start WebSocket server (handle client connections)
//     let addr: SocketAddr = "127.0.0.1:8080".parse().unwrap();
//     start_server(addr, pubsub_channel).await;
// }
