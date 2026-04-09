use crate::models::{Candle, Order, Side, Trade};
use std::collections::BTreeMap;
use serde::Serialize;


#[derive(Debug)]
pub struct OrderBook {
    pub bids: BTreeMap<u64, Vec<Order>>,
    pub asks: BTreeMap<u64, Vec<Order>>,
    pub candles: BTreeMap<u64, Candle>,
}


#[derive(Debug, Serialize, Clone)]
pub struct OrderBookResponse{
    // price, total quantiyt 
    pub bids: Vec<(u64, u64)>,
    pub asks: Vec<(u64, u64)>, 
}

impl OrderBook {
    pub fn new() -> Self {
        Self {
            bids: BTreeMap::new(),
            asks: BTreeMap::new(),
            candles: BTreeMap::new(),
        }
    }

    pub fn add_order(&mut self, order: Order) {
        match order.side {
            Side::Buy => {
                self.bids.entry(order.price).or_default().push(order);
            }
            Side::Sell => {
                self.asks.entry(order.price).or_default().push(order);
            }
        }
    }

    pub fn process_order(&mut self, order: Order) -> Result<(Vec<Trade>, Option<Order>), String> {
        let mut trades = Vec::new();

        match order.side {
            Side::Buy => {
                let mut remaining_order_qty = order.qty;

                while remaining_order_qty > 0 {
                    let (&ask_price, _) = match self.asks.first_key_value() {
                        Some(entry) => entry,
                        None => break,
                    };

                    if ask_price > order.price {
                        break;
                    }

                    let (trade, should_remove_level) = {
                        let orders = self.asks.get_mut(&ask_price).unwrap();
                        let first_order = &mut orders[0];
                        let trade_qty = std::cmp::min(remaining_order_qty, first_order.qty);

                        remaining_order_qty -= trade_qty;
                        first_order.qty -= trade_qty;

                        let trade = Trade {
                            price: ask_price,
                            qty: trade_qty,
                            maker_order_id: first_order.id,
                            taker_order_id: order.id,
                            timestamp: order.timestamp,
                        };

                        if first_order.qty == 0 {
                            orders.remove(0);
                        }
                        (trade, orders.is_empty())
                    };

                    self.update_candles(&trade);
                    trades.push(trade);

                    if should_remove_level {
                        self.asks.remove(&ask_price);
                    }
                }

                if remaining_order_qty > 0 {
                    let rest_order = Order {
                        qty: remaining_order_qty,
                        ..order
                    };
                    self.add_order(rest_order);
                    Ok((trades, Some(rest_order)))
                } else {
                    Ok((trades, None))
                }
            }
            Side::Sell => {
                let mut remaining_order_qty = order.qty;

                while remaining_order_qty > 0 {
                    let (&bid_price, _) = match self.bids.last_key_value() {
                        Some(entry) => entry,
                        None => break,
                    };

                    if bid_price < order.price {
                        break;
                    }

                    let (trade, should_remove_level) = {
                        let orders = self.bids.get_mut(&bid_price).unwrap();
                        let first_order = &mut orders[0];
                        let trade_qty = std::cmp::min(remaining_order_qty, first_order.qty);

                        remaining_order_qty -= trade_qty;
                        first_order.qty -= trade_qty;

                        let trade = Trade {
                            price: bid_price,
                            qty: trade_qty,
                            maker_order_id: first_order.id,
                            taker_order_id: order.id,
                            timestamp: order.timestamp,
                        };

                        if first_order.qty == 0 {
                            orders.remove(0);
                        }
                        (trade, orders.is_empty())
                    };

                    self.update_candles(&trade);
                    trades.push(trade);

                    if should_remove_level {
                        self.bids.remove(&bid_price);
                    }
                }

                if remaining_order_qty > 0 {
                    let rest_order = Order {
                        qty: remaining_order_qty,
                        ..order
                    };
                    self.add_order(rest_order);
                    Ok((trades, Some(rest_order)))
                } else {
                    Ok((trades, None))
                }
            }
        }
    }

    pub fn update_candles(&mut self, trade: &Trade) {
        let minute_ts = (trade.timestamp / 60_000) * 60_000;

        let candle = self.candles.entry(minute_ts).or_insert(Candle {
            timestamp: minute_ts,
            open: trade.price,
            high: trade.price,
            low: trade.price,
            close: trade.price,
            volume: 0,
            trades_count: 0,
            first_trade_ts: trade.timestamp,
            last_trade_ts: trade.timestamp,
        });

        if trade.timestamp < candle.first_trade_ts {
            candle.open = trade.price;
            candle.first_trade_ts = trade.timestamp;
        }
        if trade.timestamp > candle.last_trade_ts {
            candle.close = trade.price;
            candle.last_trade_ts = trade.timestamp;
        }
        if trade.price > candle.high {
            candle.high = trade.price;
        }
        if trade.price < candle.low {
            candle.low = trade.price;
        }
        candle.volume += trade.qty;
        candle.trades_count += 1;
    }

    pub fn get_order_books_table_data(&self, limit: usize)-> OrderBookResponse{
        // what we did is first reverse the bid to tkae the highest value of the sellers
        let bids = self.bids.iter().rev().take(limit)
        // tthen we map over the arrays in it price and their order and we get sum of the quantity of those price orders 
        .map(|(price, orders)| (*price, orders.iter().map(|o| o.qty).sum()))
        .collect();

        let asks = self.asks.iter().take(limit)
        .map(|(price, orders)| (*price, orders.iter().map(|o| o.qty).sum()))
        .collect();

        OrderBookResponse {
            bids,
            asks,
        }
    }

}
