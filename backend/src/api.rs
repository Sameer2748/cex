use axum::{extract::{Path, State}, Json, response::IntoResponse};
use std::sync::{Arc, Mutex};
use crate::manager::EngineManager;
use crate::models::{Candle};
use crate::engine::OrderBookResponse;
use crate::models::{PlaceOrderRequest, Order, Side};


pub async fn get_candles(Path(book): Path<String>, State(manager): State<Arc<Mutex<EngineManager>>>)-> impl IntoResponse {
    let mg = manager.lock().unwrap();
    if let Some(book) = mg.books.get(&book) {
        let candles = book.candles.values().cloned().collect();
        return Json(candles);
    }

    Json(vec![])
}

pub async fn get_orderbook(Path(book): Path<String>, State(manager): State<Arc<Mutex<EngineManager>>>)-> impl IntoResponse {
    let mg = manager.lock().unwrap();
     if let Some(orderbook) = mg.books.get(&book) {
        return Json(orderbook.get_order_books_table_data(20)); // Show top 20 levels
    }

    Json(OrderBookResponse {bids: vec![], asks: vec![]})

}


pub async fn place_order(
    State(manager): State<Arc<Mutex<EngineManager>>>,
    Json(payload): Json<PlaceOrderRequest>
) -> impl IntoResponse {

    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis() as u64;

    let price_u64 = crate::models::string_to_u64_price(&payload.price);
    let qty_u64 = crate::models::string_to_u64_qty(&payload.qty, &payload.symbol);
    
    let order = Order {
        id: now, 
        price: price_u64,
        qty: qty_u64,
        side: payload.side,
        timestamp: now,
    };

    let mut mg = manager.lock().unwrap();
    match mg.process_order(&payload.symbol, order) {
        Ok((trades, remaining)) => {
            Json(serde_json::json!({
                "status": "success",
                "matched_trades": trades,
                "remaining_qty": remaining.map(|o| o.qty).unwrap_or(0)
            }))
        },
        Err(e) => Json(serde_json::json!({ "status": "error", "message": e }))
    }
}
