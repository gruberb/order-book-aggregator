use std::fmt;

use serde::Deserialize;

/// Supported exchanges in this appliaction
/// Each exchange has to be adapted with a unique way of connecting
/// and subscribing to an order book
#[derive(Debug, Deserialize)]
pub enum Exchange {
	/// The Binance exchange
	Binance,
	/// The Bitstamp exchange
	Bitstamp,
}

/// Implement `Display` so we can pass the Exchange enum as Strings to other functions
impl fmt::Display for Exchange {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Exchange::Binance => write!(f, "Binance"),
			Exchange::Bitstamp => write!(f, "Bitstamp"),
		}
	}
}
