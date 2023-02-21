use std::sync::Arc;

use futures_util::{stream::SplitStream, SinkExt, StreamExt};
use serde::Deserialize;
use tokio::{net::TcpStream, sync::Mutex, task::JoinHandle};
use tokio_tungstenite::{connect_async, tungstenite, MaybeTlsStream, WebSocketStream};
use tungstenite::{error::Error, Message};
use url::Url;

use crate::{
	asset::Asset,
	exchange::Exchange,
	order_book::{OrderBook, OrderBookStore},
};

/// A type alias for the stored Stream connection we establish with different exchanges
/// and is hold by the type `Market`
type Connection = SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>;

/// Each added exchange will instantiate a new `market` which holds the request asset pair,
/// the websocket address to the exchange and optionally a connceted stream to the exchange
pub struct Market {
	/// The address in the form of `wss://ws.bitstamp.com` as a `url::Url`
	pub addr: Url,
	/// An enum which indicates which exchange this market is connected to, e.g. `Exchange::Binance`
	pub exchange: Exchange,
	/// The quote of the currency pair, e.g. `eth` for `ethbtc`
	pub quote: Asset,
	/// The base asset of the currency pair, e.g. `btc` for `ethbtc`
	pub base: Asset,
	/// An open web socket connection to an exchange which we can read from (NOT WRITE!)
	/// We would have to store a tuple and store the `write` as well if we want this in the future
	pub read_connection: Option<Connection>,
}

/// A typed response from the Bitstamp Exchange
#[derive(Debug, Deserialize)]
struct BitstampResponse {
	/// Each `bid`, and `ask` is wrapped in a `data` field which we extract here
	data: ResponseData,
}

/// A general order book response from an exchange which holds the data in a format like this:
/// `asks: [ ["3", "1"], ["2.9", "1"],...]`
/// `bids: [ ["3", "1"], ["2.9", "1"],...]`
#[derive(Debug, Deserialize)]
pub struct ResponseData {
	/// Exchange the asks/bids are from
	pub(crate) exchange: Option<Exchange>,
	/// The ask pairs. There are always two Strings in one array
	pub(crate) asks: Vec<Vec<String>>,
	/// The bid pairs. There are always two Strings in one array
	pub(crate) bids: Vec<Vec<String>>,
}

impl Market {
	pub fn new(exchange: Exchange, quote: Asset, base: Asset) -> Market {
		let addr = match exchange {
			Exchange::Binance => Url::parse(
				format!(
					"wss://stream.binance.com:9443/ws/{}{}@depth10@100ms",
					quote, base,
				)
				.as_str(),
			)
			.expect("Cannot be parsed"),
			Exchange::Bitstamp => Url::parse(" wss://ws.bitstamp.net").expect("Cannot be parsed"),
		};

		Market {
			addr,
			exchange,
			quote,
			base,
			read_connection: None,
		}
	}

	pub async fn connect(mut self) -> Result<Market, Error> {
		let (ws_stream, _) = connect_async(&self.addr).await?;
		let (mut write, read) = ws_stream.split();
		self.read_connection = Some(read);

		if let Exchange::Bitstamp = self.exchange {
			let subscribe = format!(
				r#"{{
				   "event": "bts:subscribe",
					  "data": {{
						"channel": "order_book_{}{}"
					  }}
				}}"#,
				self.quote, self.base
			);

			let _ = write.send(Message::Text(subscribe)).await;
		}

		Ok(self)
	}

	pub async fn receive_order_book(mut self, store: Arc<Mutex<OrderBookStore>>) -> JoinHandle<()> {
		tokio::spawn(async move {
			loop {
				let mut store = store.lock().await;
				println!("Current OrderBookStore: {}", store);
				let msg = self
					.read_connection
					.as_mut()
					.unwrap()
					.next()
					.await
					.expect("Error reading message");
				let msg = match msg {
					Ok(tungstenite::Message::Text(s)) => s,
					_ => {
						panic!("Error getting text");
					}
				};

				match self.exchange {
					Exchange::Bitstamp => {
						let response: serde_json::Value =
							serde_json::from_str(&msg).expect("Unable to parse message");
						if response["event"] == *"data" {
							let mut response: BitstampResponse =
								serde_json::from_str(&msg).expect("Unable to parse message");
							response.data.exchange = Some(Exchange::Bitstamp);

							store.update(OrderBook::from(response.data));
						}
					}
					Exchange::Binance => {
						let mut response: ResponseData =
							serde_json::from_str(&msg).expect("Unable to parse message");
						response.exchange = Some(Exchange::Binance);

						store.update(OrderBook::from(response));
					}
				}
			}
		})
	}
}
