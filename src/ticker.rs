use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Ticker {
    pub e: String, // Event type
    #[serde(rename = "E")]
    pub event_time: u64,    // Event time
    pub s: String, // Symbol
    pub p: String, // Price change
    #[serde(rename = "P")]
    pub price_change_percent: String, // Price change percent
    pub w: String, // Weighted average price
    pub x: String, // First trade(F)-1 price (first trade before the 24hr rolling window)
    pub c: String, // Last price
    #[serde(rename = "Q")]
    pub last_quantity: String, // Last quantity
    pub b: String, // Best bid price
    #[serde(rename = "B")]
    pub best_bid_quantity: String, // Best bid quantity
    pub a: String, // Best ask price
    #[serde(rename = "A")]
    pub best_ask_quantity: String, // Best ask quantity
    pub o: String, // Open price
    pub h: String, // High price
    pub l: String, // Low price
    pub v: String, // Total traded base asset volume
    pub q: String, // Total traded quote asset volume
    #[serde(rename = "O")]
    pub statistics_open_time: u64,    // Statistics open time
    #[serde(rename = "C")]
    pub statistics_close_time: u64,    // Statistics close time
    #[serde(rename = "F")]
    pub first_trade_id: u64,    // First trade ID
    #[serde(rename = "L")]
    pub last_trade_id: u64,    // Last trade ID
    pub n: u64,     // Total number of trades
}
