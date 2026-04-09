use crate::engine::OrderBook;
use crate::models::{Order, Trade, BinanceTrade};
use std::collections::HashMap;


pub struct EngineManager {
    pub books: HashMap<String, OrderBook>
}

// we implement funcations for this struct 
impl EngineManager {
    pub fn new() -> Self{
        Self {
            books: HashMap::new(),
        }
    }

    pub fn process_order(&mut self, symbol:&str, order: Order)-> Result<(Vec<Trade>, Option<Order>), String> {
        let book = self.books.entry(symbol.to_string()).or_insert_with(|| OrderBook::new());
        book.process_order(order)
    }
   pub fn inject_trade(&mut self, symbol: &str, b_trade: BinanceTrade) {
        let book = self.books.entry(symbol.to_string()).or_insert_with(|| OrderBook::new());
        // 1. Create a "Fake" internal trade from the Binance data
        let internal_trade = Trade {
            price: b_trade.get_price_u64(),
            qty: b_trade.get_qty_u64(),
            timestamp: b_trade.timestamp,
            maker_order_id: 0, // We use 0 because it's not our order
            taker_order_id: 0, // We use 0 because it's not our order
        };
        // 2. Now pass the REFERENCE to this trade
        book.update_candles(&internal_trade);
    }
}
