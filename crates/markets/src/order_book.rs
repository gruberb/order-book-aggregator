use std::fmt::{Display, Formatter};

use api::orderbook::{Level, Summary};
use serde::Deserialize;

use crate::{exchange::Exchange, market::ResponseData};

/// The global in-memory store for this appliaction
///
/// This store is getting passed around wrapped in an `Arc` and guarded with a `Mutex` throughout
/// the application so multiple threads and tasks can update the store.
///
/// Once the store is updated, connceted clients will receive the updated version of it everytime
/// it is getting updated through its own `update` function
#[derive(Clone, Debug, Default)]
pub struct OrderBookStore {
	/// The summary being passed to the client, in the form of
	/// `Summary { spread: f64, bids: Vec<LeveL>, asks: Vec<Level>`
	/// and
	/// `Level { exchange: String, price: f64, amount: f64`
	pub summary: Summary,
}

/// Implement the `Display` trait so we can simply `println!("{}", order_book_store)` to debug
/// and verify its contents throughout the application, and pass it to logging crates
impl Display for OrderBookStore {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		if !self.summary.asks.is_empty() {
			writeln!(f, "Spread: {}", self.summary.spread).expect("Cannot display spread");
			writeln!(f, "Asks:").expect("Cannot display");
			for a in &self.summary.asks {
				writeln!(
					f,
					"Exchange: {}, Amount: {}, Price: {}",
					a.exchange, a.amount, a.price
				)
				.expect("Cannot display");
			}
			writeln!(f, "Bids:").expect("Cannot display");
			for b in &self.summary.bids {
				writeln!(
					f,
					"Exchange: {}, Amount: {}, Price: {}",
					b.exchange, b.amount, b.price
				)
				.expect("Cannot display");
			}
			Ok(())
		} else {
			writeln!(f, "OrderBookStore is empty")
		}
	}
}

impl OrderBookStore {
	pub(crate) fn update(&mut self, order_book: OrderBook) {
		self.summary
			.asks
			.retain(|a| a.exchange != order_book.exchange.to_string());
		self.summary
			.bids
			.retain(|b| b.exchange != order_book.exchange.to_string());

		let mut asks: Vec<Level> = order_book
			.asks
			.into_iter()
			.map(|a| Level {
				exchange: order_book.exchange.to_string(),
				price: a.0,
				amount: a.1,
			})
			.collect();

		let mut bids: Vec<Level> = order_book
			.bids
			.into_iter()
			.map(|a| Level {
				exchange: order_book.exchange.to_string(),
				price: a.0,
				amount: a.1,
			})
			.collect();

		self.summary.asks.append(&mut asks);
		self.summary.bids.append(&mut bids);

		self.summary
			.asks
			.sort_by(|a, b| a.price.partial_cmp(&b.price).unwrap());
		self.summary
			.bids
			.sort_by(|a, b| b.price.partial_cmp(&a.price).unwrap());

		self.summary.asks = self.summary.clone().asks.into_iter().take(10).collect();
		self.summary.bids = self.summary.clone().bids.into_iter().take(10).collect();

		self.summary.spread = self.summary.asks[0].price - self.summary.bids[0].price;
	}
}

/// A Quote for a currency pair with `Amount` and `Price`
#[derive(Debug, Deserialize)]
struct Quote(f64, f64);

/// The OrderBook holds the exchange name and all asks and bids recevied from the exchange through
/// the websocket connection
#[derive(Debug, Deserialize)]
pub struct OrderBook {
	/// The exchange this order book is from, e.g. Exchange::Binance
	exchange: Exchange,
	/// A list of ask quotes for a currency pair
	asks: Vec<Quote>,
	/// A list of bid quotes for a currency pair
	bids: Vec<Quote>,
}

impl From<ResponseData> for OrderBook {
	fn from(r: ResponseData) -> Self {
		OrderBook {
			asks: r
				.asks
				.iter()
				.map(|a| {
					Quote(
						a[0].parse::<f64>().expect("Cannot parse"),
						a[1].parse::<f64>().expect("Cannot parse"),
					)
				})
				.collect(),
			bids: r
				.bids
				.iter()
				.map(|b| {
					Quote(
						b[0].parse::<f64>().expect("Cannot parse"),
						b[1].parse::<f64>().expect("Cannot parse"),
					)
				})
				.collect(),
			exchange: r.exchange.unwrap(),
		}
	}
}
