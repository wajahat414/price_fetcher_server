use async_trait::async_trait;

/// Minimal spot REST API for common info; extend as needed
#[async_trait]
pub trait RestApi {
    type Error: std::error::Error + Send + 'static;

    async fn ping(&self) -> Result<(), Self::Error> {
        Ok(())
    }

    /// Get a simple spot ticker for a symbol; price-only for now.
    async fn ticker_price(&self, _symbol: &str) -> Result<f64, Self::Error>;
}

/// WebSocket streaming for book tickers (best bid/ask)
#[async_trait]
pub trait WebSocketApi {
    type Error: std::error::Error + Send + 'static;
    /// Connect and subscribe to book ticker for given symbols; user provides a callback.
    async fn subscribe_book_ticker<F>(&self, symbols: Vec<String>, on_update: F) -> Result<(), Self::Error>
    where
        F: Fn(String, (f64, f64), (f64, f64)) + Send + Sync + 'static;
}
