use crate::errors::ConnectorError;
use crate::traits::{RestApi, WebSocketApi};
use async_trait::async_trait;
use reqwest::Client;
use serde::Deserialize;

#[derive(Clone)]
pub struct MexcConnector { client: Client }

impl MexcConnector {
	pub fn new() -> Self { Self { client: Client::new() } }
}

#[async_trait]
impl RestApi for MexcConnector {
	type Error = ConnectorError;

	async fn ticker_price(&self, _symbol: &str) -> Result<f64, Self::Error> {
		// MEXC expects symbols like BTCUSDT (no dash)
		let symbol = _symbol.replace('-', "");
		let url = format!("https://api.mexc.com/api/v3/ticker/price?symbol={}", symbol);
		#[derive(Deserialize)]
		struct Resp { price: String }
		let resp: Resp = self.client.get(url).send().await?.json().await?;
		let price: f64 = resp
			.price
			.parse::<f64>()
			.map_err(|e| ConnectorError::Other(e.to_string()))?;
		Ok(price)
	}
}

#[async_trait]
impl WebSocketApi for MexcConnector {
	type Error = ConnectorError;

	async fn subscribe_book_ticker<F>(&self, _symbols: Vec<String>, _on_update: F) -> Result<(), Self::Error>
	where
		F: Fn(String, (f64, f64), (f64, f64)) + Send + Sync + 'static,
	{
		// TODO: Implement MEXC WS ticker
		Err(ConnectorError::Other("MEXC WS not implemented".into()))
	}
}

