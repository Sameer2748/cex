use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy)]
pub struct Order {
    pub id: u64,
    pub price: u64,
    pub qty: u64,
    pub side: Side,
    pub timestamp: u64,
}
#[derive(Debug, Clone, Copy, Serialize)]
pub struct Trade {
    pub price: u64,
    pub qty: u64,
    pub maker_order_id: u64,
    pub taker_order_id: u64,
    pub timestamp: u64,
}
#[derive(Debug, Clone, Copy,Serialize, Deserialize)]
#[serde(rename_all = "lowercase")] 
pub enum Side {
    Buy,
    Sell,
}

#[derive(Debug, Clone, Serialize)]
// candle struct
pub struct Candle {
    pub timestamp: u64,
    pub open: u64,
    pub high: u64,
    pub low: u64,
    pub close: u64,
    pub volume: u64,
    pub trades_count: u64,
    pub first_trade_ts: u64,
    pub last_trade_ts: u64,
}

#[derive(Debug, Deserialize)]
pub struct BinanceTrade {
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "p")]
    pub price: String,
    #[serde(rename = "q")]
    pub qty: String,
    #[serde(rename = "T")]
    pub timestamp: u64,
    #[serde(rename = "m")]
    pub is_buyer_maker: bool,
}

impl BinanceTrade {
    pub fn get_price_u64(&self) -> u64 {
        // parse as float the price
        let value = self.price.parse::<f64>().unwrap();
        // and multiply by 100 to get the pointer values also
        (value * 100.0) as u64
    }

    pub fn get_qty_u64(&self) -> u64 {
        let value = self.qty.parse::<f64>().unwrap_or(0.0);

        // Check the symbol!
        let multiplier = match self.symbol.as_str() {
            "BTCUSDT" => 100_000_000.0,   // 8 decimals
            "SOLUSDT" => 1_000_000_000.0, // 9 decimals
            _ => 100.0,                   // Default to 2 decimals
        };

        (value * multiplier) as u64
    }
}




#[derive(Debug, Deserialize)]
// order route struct 
pub struct PlaceOrderRequest{
    pub symbol: String,
    pub price: String,
    pub qty: String,
    pub side: Side,
}

#[derive(Debug, Deserialize)]
pub struct BinanceDepth {
    // Remove the "rename" lines here!
    pub bids: Vec<Vec<String>>,
    pub asks: Vec<Vec<String>>,
}

pub fn string_to_u64_price(s: &str) -> u64 {
    let value = s.parse::<f64>().unwrap_or(0.0);
    (value * 100.0) as u64
}

pub fn string_to_u64_qty(s: &str, symbol: &str) -> u64 {
    let value = s.parse::<f64>().unwrap_or(0.0);
    let multiplier = match symbol {
        "BTCUSDT" => 100_000_000.0,
        _ => 100.0,
    };
    (value * multiplier) as u64
}

pub fn u64_to_human_price(val: u64) -> String {
    format!("{:.2}", val as f64 / 100.0)
}
pub fn u64_to_human_qty(val: u64, symbol: &str) -> String {
    let divisor = if symbol == "BTCUSDT" { 100_000_000.0 } else { 100.0 };
    format!("{:.8}", val as f64 / divisor)
}



// login routes struct 
#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    pub email: String,
    pub password: String,
}
#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub status: String,
    pub user_id: i32,
    pub token: Option<String>,
}