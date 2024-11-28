use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct OrderBook {
    pub bid: (f64, f64),
    pub ask: (f64, f64),
}
pub struct OrderBookDepth {
    pub bids: Vec<(f64, f64)>,
    pub asks: Vec<(f64, f64)>,
}

pub type OrderBookCollection = HashMap<String, HashMap<String, OrderBook>>;
pub type OrderBookDepthCollection = HashMap<String, HashMap<String, OrderBookDepth>>;
