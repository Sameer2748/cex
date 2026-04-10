use crate::engine::OrderBook;
use crate::models::{BinanceDepth, BinanceTrade, Order, Side, Trade};
use std::collections::HashMap;
use sqlx::PgPool;


pub struct EngineManager {
    pub books: HashMap<String, OrderBook>,
    pub db: PgPool,
}

// we implement funcations for this struct
impl EngineManager {
    pub fn new(db:PgPool) -> Self {
        Self {
            books: HashMap::new(),
            db,
        }
    }

    pub fn process_order(
        &mut self,
        symbol: &str,
        order: Order,
    ) -> Result<(Vec<Trade>, Option<Order>), String> {
        let book = self
            .books
            .entry(symbol.to_string())
            .or_insert_with(|| OrderBook::new());
        book.process_order(order)
    }
    pub fn inject_trade(&mut self, symbol: &str, b_trade: BinanceTrade) {
        let book = self
            .books
            .entry(symbol.to_string())
            .or_insert_with(|| OrderBook::new());
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

    pub fn sync_depth(&mut self, symbol: &str, depth: BinanceDepth) {
        let book = self.books.entry(symbol.to_string()).or_insert_with(|| OrderBook::new());

            book.bids.clear();
            book.asks.clear();

            for b in depth.bids {
                let price = crate::models::string_to_u64_price(&b[0]);
                let qty = crate::models::string_to_u64_qty(&b[1], "BTCUSDT");

                book.add_order(Order {
                    id: 0,
                    price,
                    qty,
                    side: Side::Buy,
                    timestamp: 0,
                });
            }
            for a in depth.asks {
                let price = crate::models::string_to_u64_price(&a[0]);
                let qty = crate::models::string_to_u64_qty(&a[1], "BTCUSDT");

                book.add_order(Order {
                    id: 0,
                    price,
                    qty,
                    side: Side::Sell,
                    timestamp: 0,
                });
            }

    }
}
