use crate::models::{OrderBook, OrderBookCollection, OrderBookDepth, OrderBookDepthCollection};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
#[derive(Clone)]
pub struct DataAggregator {
    pub collection: Arc<RwLock<OrderBookCollection>>,
    pub depthcollection: Arc<RwLock<OrderBookDepthCollection>>,
}

impl DataAggregator {
    pub fn new() -> Self {
        Self {
            collection: Arc::new(RwLock::new(HashMap::new())),
            depthcollection: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn update_order_book(&self, exchange: &str, symbol: &str, order_book: OrderBook) {
        let mut collection = self.collection.write().await;
        collection
            .entry(exchange.to_string())
            .or_insert_with(HashMap::new)
            .insert(symbol.to_string(), order_book);
    }

    pub async fn get_order_book(&self, exchange: &str, symbol: &str) -> Option<OrderBook> {
        let collection = self.collection.read().await;

        collection
            .get(exchange)
            .and_then(|symbols| symbols.get(symbol).cloned())
    }

    pub async fn update_order_book_depth(
        &self,
        exchange: &str,
        symbol: &str,
        order_book: OrderBookDepth,
    ) {
        let mut collection = self.depthcollection.write().await;
        collection
            .entry(exchange.to_string())
            .or_insert_with(HashMap::new)
            .insert(symbol.to_string(), order_book);
    }

    // pub async fn get_order_book_depth(
    //     &self,
    //     exchange: &str,
    //     symbol: &str,
    // ) -> Option<OrderBookDepth> {
    //     let collection = self.depthcollection.read().await;

    //     collection
    //         .get(exchange)
    //         .and_then(|symbols| symbols.get(symbol).cloned())
    // }
}
