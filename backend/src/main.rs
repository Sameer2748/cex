pub mod models;
pub mod engine;
pub mod manager;
pub mod binance;
pub mod api;
pub mod db;


use std::sync::{Arc, Mutex};
use axum::{routing::{get, post, delete, put}, Router};
use std::net::SocketAddr;
use std::time::{SystemTime, UNIX_EPOCH};
use models::{Order, Side};
use engine::OrderBook;
use api::{get_candles, get_orderbook, place_order};
use manager::EngineManager;
use std::time::Instant;
use binance::monitor_binance_trades;
use db::init_db;

#[tokio::main]
async fn main() {
    let pool = init_db().await.unwrap();
   let manager = Arc::new(Mutex::new(EngineManager::new(pool)));

   // spawn the binance task in bg 
   let tr_manager = Arc::clone(&manager);
   tokio::spawn(async move {
    monitor_binance_trades(tr_manager).await;
   });


   let app = Router::new()
   .route("/health", get(healthcheck))
   .route("/get_candles/:book", get(get_candles))
   .route("/orderbook/:book", get(get_orderbook))
   .route("/order", post(place_order))
   .with_state(manager);

   let addr = SocketAddr::from(([127,0,0,1], 3000)); 
   let listner = tokio::net::TcpListener::bind(addr).await.unwrap();
   axum::serve(listner, app).await.unwrap();
}


pub async fn healthcheck()-> &'static str{
    "Health check done"
}