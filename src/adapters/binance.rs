use crate::errors::ConnectorError;
use crate::traits::{RestApi, WebSocketApi};
use async_trait::async_trait;
use binance::model::BookTickerEvent;
use binance::websockets::*;
use reqwest::Client;
use std::sync::{atomic::AtomicBool, Arc};

#[derive(Clone, Default)]
pub struct BinanceConnector {
    client: Client,
}

impl BinanceConnector {
    pub fn new() -> Self {
        Self { client: Client::new() }
    }
}

#[async_trait]
impl RestApi for BinanceConnector {
    type Error = ConnectorError;

    async fn ticker_price(&self, symbol: &str) -> Result<f64, Self::Error> {
        // Binance expects symbols like BTCUSDT (no dash)
        let symbol = symbol.replace('-', "");
        let url = format!(
            "https://api.binance.com/api/v3/ticker/price?symbol={}",
            symbol
        );
        #[derive(serde::Deserialize)]
        struct Resp {
            price: String,
        }
        let resp: Resp = self.client.get(url).send().await?.json().await?;
        let price: f64 = resp
            .price
            .parse::<f64>()
            .map_err(|e| ConnectorError::Other(e.to_string()))?;
        Ok(price)
    }
}

#[async_trait]
impl WebSocketApi for BinanceConnector {
    type Error = ConnectorError;

    async fn subscribe_book_ticker<F>(
        &self,
        symbols: Vec<String>,
        on_update: F,
    ) -> Result<(), Self::Error>
    where
        F: Fn(String, (f64, f64), (f64, f64)) + Send + Sync + 'static,
    {
        let cb = Arc::new(on_update);
        let symbols_clone = symbols.clone();

        let join = tokio::task::spawn_blocking(move || -> Result<(), ConnectorError> {
            let cb = cb.clone();
            let mut ws_client = WebSockets::new(move |event: WebsocketEvent| {
                if let WebsocketEvent::BookTicker(ref book_ticker) = event {
                    handle_book_ticker_event(book_ticker, &cb);
                }
                Ok(())
            });

            // Construct stream names for all symbols
            let streams: Vec<String> = symbols_clone
                .iter()
                .map(|s| format!("{}@bookTicker", s.to_lowercase()))
                .collect();
            ws_client.connect_multiple_streams(&streams)?;

            let keep_running = AtomicBool::new(true);
            ws_client.event_loop(&keep_running)?;
            Ok(())
        });

        // Await until the loop finishes or errors
        join.await??;
        Ok(())
    }
}

fn handle_book_ticker_event<F>(
    book_ticker: &BookTickerEvent,
    on_update: &Arc<F>,
) where
    F: Fn(String, (f64, f64), (f64, f64)) + Send + Sync + 'static,
{
    let symbol = book_ticker.symbol.clone();
    let best_bid_price = book_ticker.best_bid.parse::<f64>().unwrap_or_default();
    let best_bid_qty = book_ticker.best_bid_qty.parse::<f64>().unwrap_or_default();
    let best_ask_price = book_ticker.best_ask.parse::<f64>().unwrap_or_default();
    let best_ask_qty = book_ticker.best_ask_qty.parse::<f64>().unwrap_or_default();

    let bid = (best_bid_price, best_bid_qty);
    let ask = (best_ask_price, best_ask_qty);
    (on_update)(symbol, bid, ask);
}
