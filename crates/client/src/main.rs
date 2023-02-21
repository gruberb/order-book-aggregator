pub(crate) mod client;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
	let _ = client::open_client().await?;

	Ok(())
}
