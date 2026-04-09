use axum::{extract::{Path, State}, Json, response::IntoResponse};
use std::sync::{Arc, Mutex};
use crate::manager::EngineManager;
use crate::models::Candle;

pub async fn get_candles(Path(book): Path<String>, State(manager): State<Arc<Mutex<EngineManager>>>)-> impl IntoResponse {
    let mg = manager.lock().unwrap();
    if let Some(book) = mg.books.get(&book) {
        let candles = book.candles.values().cloned().collect();
        return Json(candles);
    }

    Json(vec![])
}