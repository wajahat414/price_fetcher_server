use crate::data_aggregator::DataAggregator;
use crate::models::{OrderBook, OrderBookCollection, OrderBookDepth};
use binance::model::BookTickerEvent;
use binance::{model::DepthOrderBookEvent, websockets::*};
use std::sync::{atomic::AtomicBool, Arc};

pub async fn binance_ws(symbols: Vec<String>, aggregator: Arc<DataAggregator>) {
    let mut ws_client = WebSockets::new(move |event: WebsocketEvent| {
        if let WebsocketEvent::DepthOrderBook(ref depth_order_book) = event {
            let aggregator_clone = aggregator.clone();

            // handle_depth_order_book_event(depth_order_book, aggregator_clone);
        }
        if let WebsocketEvent::BookTicker(ref book_ticker) = event {
            let aggregator_clone = aggregator.clone();
            handle_book_ticker_event(book_ticker, aggregator_clone);
        }

        Ok(())
    });

    // Construct stream names for all symbols
    let streams: Vec<String> = symbols
        .iter()
        .map(|s| format!("{}@bookTicker", s.to_lowercase()))
        .collect();
    if let Err(e) = ws_client.connect_multiple_streams(&streams) {
        eprintln!("Failed to connect to streams: {:?}", e);
        return;
    }
    let keep_running = AtomicBool::new(true);
    if let Err(e) = ws_client.event_loop(&keep_running) {
        println!("Error: {:?}", e);
    }
}

fn handle_book_ticker_event(book_ticker: &BookTickerEvent, aggregator: Arc<DataAggregator>) {
    let symbol = book_ticker.symbol.clone();
    let best_bid_price = book_ticker.best_bid.parse::<f64>().unwrap();

    let best_bid_qty = book_ticker.best_bid_qty.parse::<f64>().unwrap();
    let best_ask_price = book_ticker.best_ask.parse::<f64>().unwrap();
    let best_ask_qty = book_ticker.best_ask_qty.parse::<f64>().unwrap();

    let bid = (best_bid_price, best_bid_qty);
    let ask = (best_ask_price, best_ask_qty);

    let order_book = OrderBook { bid, ask };
    tokio::spawn(async move {
        aggregator
            .update_order_book("binance", &symbol, order_book)
            .await;
    });
}

// fn handle_depth_order_book_event(depth: &DepthOrderBookEvent, aggregator: Arc<DataAggregator>) {
//     let bids: Vec<(f64, f64)> = depth
//         .bids
//         .iter()
//         .map(|bids| (bids.price, bids.qty))
//         .collect();

//     let asks: Vec<(f64, f64)> = depth
//         .asks
//         .iter()
//         .map(|asks| (asks.price, asks.qty))
//         .collect();

//     let symbol = depth.symbol.clone();

//     let order_book = OrderBookDepth { bids, asks };
//     tokio::spawn(async move {
//         aggregator
//             .update_order_book_depth("binance", &symbol, order_book)
//             .await;
//     });
// }
