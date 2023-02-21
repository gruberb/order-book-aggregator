use std::fmt;

/// The supported assets
/// Different exchanges support different currency pairs, so not all combinations
/// work for all exchanges
pub enum Asset {
	/// The Bitcoin currency symbol
	Btc,
	/// The Ethereum currency symbol
	Eth,
}

/// To be able to pass the Asset enum into format! macros and conver them to Strings
impl fmt::Display for Asset {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Asset::Btc => write!(f, "btc"),
			Asset::Eth => write!(f, "eth"),
		}
	}
}
