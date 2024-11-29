mod adapters;
mod data_aggregator;
mod models;

use crate::adapters::binance::binance_ws;
use crate::data_aggregator::DataAggregator;
use adapters::kucoin::{KucoinConnector, KucoinWebSocketResponse};
use std::collections;
use std::sync::Arc;
use tokio::signal;
use tokio::time::{self, Duration};

#[tokio::main]

async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let connector = KucoinConnector::new();

    // Fetch WebSocket configuration
    let response = connector.get_websocket_url().await?;
    if let KucoinWebSocketResponse { data, .. } = response {
        let servers = data.instance_servers;
        let token = data.token;

        // Define your custom symbol list
        let symbols = vec!["BTC-USDT".to_string(), "ETH-USDT".to_string()];

        // Connect to WebSocket and subscribe
        connector
            .connect_and_subscribe(servers, &token, symbols)
            .await?;
    }

    Ok(())
}

// async fn main() -> Result<(), Box<dyn std::error::Error>> {
//     let aggregator = Arc::new(DataAggregator::new());

//     // Symbols to monitor
//     let binance_symbols = vec!["btcusdt".to_string(), "ethusdt".to_string()];

//     let connector = KucoinConnector::new();

//     let response = connector.get_websocket_url().await?;

//     if let KucoinWebSocketResponse { data, .. } = response {
//         let servers = data.instance_servers;
//         let token = data.token;

//         // Define your custom symbol list
//         let symbols = vec!["BTC-USDT".to_string(), "ETH-USDT".to_string()];

//         // Connect to WebSocket and subscribe
//         connector
//             .connect_and_subscribe(servers, &token, symbols)
//             .await?;
//     }

//     Ok(());

//     // Atomic flag to control the WebSocket lifecycle

//     // Spawn WebSocket client task
//     // let binance_task = tokio::spawn(binance_ws(binance_symbols, aggregator.clone()));

//     // // Graceful shutdown using Ctrl+C signal or other triggers
//     // let shutdown_task = tokio::spawn(async move {
//     //     signal::ctrl_c().await.unwrap();
//     //     println!("Shutdown signal received, stopping WebSocket...");
//     // });

//     // let print_task = tokio::spawn(print_collection_priodically(aggregator.clone()));

//     // Wait for both tasks to complete
//     // let _ = tokio::try_join!(binance_task, print_task, shutdown_task);

//     return Err("Something went wrong ".into());
// }

async fn print_collection_priodically(aggregator: Arc<DataAggregator>) {
    let mut interval = time::interval(Duration::from_secs(1));
    loop {
        interval.tick().await;
        let collection = aggregator.collection.read().await;

        println!("Current orderbook collectiion");
        for (exchange, symbols) in collection.iter() {
            println!("Exchange :{}", exchange);

            for (symbol, order_book) in symbols.iter() {
                println!(
                    "   Symbol: {},  OrderBook Bid {:?}  Ask{:?}",
                    symbol, order_book.bid, order_book.ask
                );
            }
        }
    }
}
