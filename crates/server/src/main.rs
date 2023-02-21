pub(crate) mod grpc;

use std::{error::Error, sync::Arc};

use markets::{asset::Asset, exchange::Exchange, market::Market, order_book::OrderBookStore};
use tokio::sync::Mutex;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
	let store = Arc::new(Mutex::new(OrderBookStore::default()));

	let binance = Market::new(Exchange::Binance, Asset::Eth, Asset::Btc);
	let binance = binance.connect().await.expect("Could not connect");
	let update_binance_handle = binance.receive_order_book(store.clone());

	let bitstamp = Market::new(Exchange::Bitstamp, Asset::Eth, Asset::Btc);
	let bitstamp = bitstamp.connect().await.expect("Could not connect");
	let update_bitstamp_handle = bitstamp.receive_order_book(store.clone());

	let server_handle = grpc::OrderBookServer::build(store).start();

	let _ = tokio::join!(
		update_binance_handle.await,
		update_bitstamp_handle.await,
		server_handle.await,
	);

	Ok(())
}
