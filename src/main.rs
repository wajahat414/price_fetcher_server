use exchange_connectors::adapters::kucoin::{KucoinConnector, KucoinWebSocketConfig};
use exchange_connectors::data_aggregator::DataAggregator;
use std::sync::Arc;
use tokio::time::{self, Duration};

#[tokio::main]

async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let connector = KucoinConnector::new();

    // Fetch WebSocket configuration
    let config: KucoinWebSocketConfig = connector.get_websocket_url().await?;

    let servers = config.servers;
    let token = config.token;

    // Define your custom symbol list
    let symbols = vec!["BTC-USDT".to_string(), "ETH-USDT".to_string()];

    // Connect to WebSocket and subscribe
    connector
        .connect_and_subscribe(servers, &token, symbols)
        .await?;

    Ok(())
}

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
