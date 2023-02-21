use std::error::Error;

use api::orderbook::{orderbook_aggregator_client::OrderbookAggregatorClient, Empty, Summary};
use tokio::sync::mpsc;
use tokio_stream::StreamExt;

pub async fn open_client() -> Result<mpsc::Receiver<Summary>, Box<dyn Error>> {
	let (_, receiver) = mpsc::channel(2048);

	let mut client = OrderbookAggregatorClient::connect("http://localhost:6669").await?;

	let mut stream = client.book_summary(Empty {}).await?.into_inner();

	while let Some(res) = stream.next().await {
		match res {
			Ok(summary) => {
				println!("Spread: {}", summary.spread);
				println!("Asks");
				for a in summary.asks {
					println!(
						"Exchange: {}, Price: {}, Amount: {}",
						a.exchange, a.price, a.amount
					);
				}
				println!("Bids");
				for b in summary.bids {
					println!(
						"Exchange: {}, Price: {}, Amount: {}",
						b.exchange, b.price, b.amount
					);
				}
			}
			Err(err) => {
				return Err(err.into());
			}
		};
	}

	Ok(receiver)
}
