//! Exchange connectors library: Binance, KuCoin, Kraken, MEXC (WS + REST)
//!
//! This crate exposes a small, consistent API to subscribe to best bid/ask (book ticker)
//! streams and query spot tickers via REST across multiple exchanges.

pub mod adapters;
pub mod data_aggregator;
pub mod models;
pub mod traits;
pub mod errors;

pub use adapters::*;
pub use models::*;
pub use traits::*;
pub use errors::*;
