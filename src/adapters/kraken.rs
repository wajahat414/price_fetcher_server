use crate::errors::ConnectorError;
use crate::traits::{RestApi, WebSocketApi};
use async_trait::async_trait;

#[derive(Clone, Default)]
pub struct KrakenConnector;

impl KrakenConnector {
	pub fn new() -> Self { Self }
}

#[async_trait]
impl RestApi for KrakenConnector {
	type Error = ConnectorError;

	async fn ticker_price(&self, _symbol: &str) -> Result<f64, Self::Error> {
		// TODO: Implement Kraken REST ticker endpoint
		Err(ConnectorError::Other("Kraken REST not implemented".into()))
	}
}

#[async_trait]
impl WebSocketApi for KrakenConnector {
	type Error = ConnectorError;

	async fn subscribe_book_ticker<F>(&self, _symbols: Vec<String>, _on_update: F) -> Result<(), Self::Error>
	where
		F: Fn(String, (f64, f64), (f64, f64)) + Send + Sync + 'static,
	{
		// TODO: Implement Kraken WS ticker
		Err(ConnectorError::Other("Kraken WS not implemented".into()))
	}
}

