use std::{
	net::{IpAddr, Ipv4Addr, SocketAddr},
	pin::Pin,
	sync::Arc,
};

use api::orderbook::{
	orderbook_aggregator_server, orderbook_aggregator_server::OrderbookAggregatorServer, Empty,
	Summary,
};
use futures_core::Stream;
use markets::order_book::OrderBookStore;
use tokio::{
	sync::{mpsc, Mutex},
	task::JoinHandle,
};
use tokio_stream::wrappers::ReceiverStream;
use tonic::{codegen::futures_core, transport::Server, Request, Response, Status};

/// The gRPC server which stores the URL (IP + PORT) and the global in-memory store
pub struct OrderBookServer {
	/// The address to which clients can connect to
	pub(crate) addr: SocketAddr,
	/// The in-memory store for shared access across threads/tasks
	pub(crate) store: Arc<Mutex<OrderBookStore>>,
}

#[async_trait::async_trait]
impl orderbook_aggregator_server::OrderbookAggregator for OrderBookServer {
	/// An Alias for making it easier to work with and read the Result for the stream
	type BookSummaryStream = Pin<Box<dyn Stream<Item = Result<Summary, Status>> + Send>>;

	async fn book_summary(
		&self,
		_: Request<Empty>,
	) -> Result<Response<Self::BookSummaryStream>, Status> {
		let (tx, rx) = mpsc::channel(2048);
		let store = self.store.clone();

		let mut cached_response = Summary::default();

		tokio::spawn(async move {
			loop {
				let order_book_store = store.lock().await;
				if order_book_store.clone().summary != cached_response {
					cached_response = order_book_store.clone().summary;
					if let Err(err) = tx.send(Ok(order_book_store.clone().summary)).await {
						println!("Error: {err:?}");
						return;
					}
				}
			}
		});

		let stream = ReceiverStream::new(rx);
		Ok(Response::new(Box::pin(stream)))
	}
}

impl OrderBookServer {
	/// The IP address for clients to connect to
	pub const DEFAULT_IP: IpAddr = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
	/// Opening this port to be able for clients to connect
	pub const DEFAULT_PORT: u16 = 6669;

	pub fn build(store: Arc<Mutex<OrderBookStore>>) -> OrderBookServer {
		Self {
			addr: SocketAddr::new(OrderBookServer::DEFAULT_IP, OrderBookServer::DEFAULT_PORT),
			store,
		}
	}

	pub async fn start(self) -> JoinHandle<()> {
		let addr = self.addr;
		let service = OrderbookAggregatorServer::new(self);

		tokio::spawn(async move {
			let _ = Server::builder().add_service(service).serve(addr).await;
		})
	}
}
