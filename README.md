# exchange-connectors

[![Crates.io](https://img.shields.io/crates/v/exchange-connectors.svg)](https://crates.io/crates/exchange-connectors)
[![Docs.rs](https://img.shields.io/docsrs/exchange-connectors)](https://docs.rs/exchange-connectors)
[![License: MIT/Apache-2.0](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](#license)

Unified Rust connectors for top crypto exchanges (Binance, KuCoin, Kraken, MEXC) providing simple REST and WebSocket APIs with a consistent interface.

## Why this library?

- One small, consistent API across multiple exchanges
- Minimal dependencies; straightforward to integrate into bots, data collectors, and services
- Start with best-bid/ask (book ticker) and spot ticker; extend as needed

## Features

- Consistent traits across exchanges
  - REST: fetch simple spot ticker price
  - WebSocket: subscribe to book ticker (best bid/ask) updates
- Adapters included
  - Binance: REST + WS (bookTicker)
  - KuCoin: REST + WS (ticker)
  - Kraken: stubs (TODO)
  - MEXC: REST implemented, WS stub (TODO)
- Optional example binary left in the repo (price_fetcher_server)

## Crate status

- Alpha: API surface is small and may change.
- Implemented: Binance (REST/WS), KuCoin (REST/WS), MEXC (REST)
- Planned: Kraken (REST/WS), MEXC (WS)

## Install

In your `Cargo.toml`:

```toml
[dependencies]
exchange-connectors = "0.1"
```

MSRV: Rust 1.70+ recommended.

## Support matrix

- Binance: REST ticker (OK), WS bookTicker (OK)
- KuCoin: REST ticker (OK), WS ticker (OK)
- MEXC: REST ticker (OK), WS (Planned)
- Kraken: REST/WS (Planned)

## Quick start

### REST usage

```rust
use exchange_connectors::adapters::{BinanceConnector, KucoinConnector, MexcConnector};
use exchange_connectors::traits::RestApi;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Binance uses BTCUSDT (no dash); KuCoin uses BTC-USDT (with dash); MEXC uses BTCUSDT.
    let binance = BinanceConnector::new();
    let b_price = binance.ticker_price("BTC-USDT").await?; // internally normalizes to BTCUSDT
    println!("Binance BTC price: {}", b_price);

    let kucoin = KucoinConnector::new();
    let k_price = kucoin.ticker_price("BTC-USDT").await?;
    println!("KuCoin BTC price: {}", k_price);

    let mexc = MexcConnector::new();
    let m_price = mexc.ticker_price("BTC-USDT").await?;
    println!("MEXC BTC price: {}", m_price);

    Ok(())
}
```

### WebSocket usage (book ticker)

```rust
use exchange_connectors::adapters::{BinanceConnector, KucoinConnector};
use exchange_connectors::traits::WebSocketApi;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Binance expects lowercase streams and BTCUSDT; KuCoin expects BTC-USDT.
    let binance = BinanceConnector::new();
    tokio::spawn(async move {
        let _ = binance
            .subscribe_book_ticker(vec!["BTCUSDT".to_string()], |symbol, bid, ask| {
                println!("[Binance] {symbol} bid={:?} ask={:?}", bid, ask);
            })
            .await;
    });

    let kucoin = KucoinConnector::new();
    kucoin
        .subscribe_book_ticker(vec!["BTC-USDT".to_string()], |symbol, bid, ask| {
            println!("[KuCoin] {symbol} bid={:?} ask={:?}", bid, ask);
        })
        .await?;

    Ok(())
}
```

## API surface

- Traits

  - `RestApi`
    - `type Error: std::error::Error + Send + 'static`
    - `async fn ticker_price(&self, symbol: &str) -> Result<f64, Self::Error>`
  - `WebSocketApi`
    - `type Error: std::error::Error + Send + 'static`
    - `async fn subscribe_book_ticker<F>(&self, symbols: Vec<String>, on_update: F) -> Result<(), Self::Error>`
      - `F: Fn(String, (f64, f64), (f64, f64)) + Send + Sync + 'static`

- Errors
  - Unified `ConnectorError` enum for common cases (HTTP, WS, Binance, Serde, Join, Other)

## Symbol conventions

- Binance: `BTCUSDT` (no dash). This crate normalizes `BTC-USDT` -> `BTCUSDT` for REST.
- KuCoin: `BTC-USDT` (dash required).
- MEXC: `BTCUSDT` (no dash). This crate normalizes `BTC-USDT` -> `BTCUSDT` for REST.
- Kraken: varies; mapping TBD (coming soon).

## Examples

This repository ships with a simple binary (`price_fetcher_server`) showing how to call a connector. You can run it with:

```bash
cargo run
```

Or create your own binary with the code examples above.

## Publishing (maintainers)

This crate is intended for crates.io. To publish:

1. Ensure metadata in `Cargo.toml` is correct (name, version, license, repository, readme)
2. Login to crates.io and dry-run:

```bash
cargo login
cargo publish --dry-run
```

3. Publish:

```bash
cargo publish
```

## Development

- Build and check:

```bash
cargo build
cargo clippy --all-targets -- -D warnings
cargo fmt --all
```

- Run the example binary:

```bash
cargo run
```

Notes:

- Public/private endpoints beyond ticker are not yet implemented (no auth flow)
- Handle exchange rate limits yourself; this crate performs raw requests
- Network errors and stream drops are surfaced via errors; consider reconnect/backoff in your app

## Roadmap

- [ ] Implement Kraken REST ticker
- [ ] Implement Kraken WS book ticker
- [ ] Implement MEXC WS book ticker
- [ ] Add feature flags per exchange to make deps optional
- [ ] Add tests and CI
- [ ] Stabilize trait API and version 0.2

## License

Licensed under either of

- Apache License, Version 2.0 (LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license (LICENSE-MIT or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in this crate by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
