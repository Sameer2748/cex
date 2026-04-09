pub mod models;
pub mod engine;
pub mod manager;
pub mod binance;
pub mod api;
use std::sync::{Arc, Mutex};

use axum::{routing::{get, post, delete, put}, Router};
use std::net::SocketAddr;
use std::time::{SystemTime, UNIX_EPOCH};
use models::{Order, Side};
use engine::OrderBook;
use api::{get_candles};
use manager::EngineManager;
use std::time::Instant;
use binance::monitor_binance_trades;

#[tokio::main]
async fn main() {
   let manager = Arc::new(Mutex::new(EngineManager::new()));

   // spawn the binance task in bg 
   let tr_manager = Arc::clone(&manager);
   tokio::spawn(async move {
    monitor_binance_trades(tr_manager).await;
   });

   // now keep main fn alive 
   let diag_manager = Arc::clone(&manager);
   tokio::spawn(async move {
   loop {
    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;

    // we got this lock here becuase we used arc and mutex ar wrapper of this data and we get pointer of it 
     let mg = diag_manager.lock().unwrap();
        if let Some(book) = mg.books.get("BTCUSDT") {
            println!("Current BTC Candles: {}", book.candles.len());
        }
   }});


   let app = Router::new()
   .route("/health", get(healthcheck))
   .route("/get_candles/:book", get(get_candles))
   .with_state(manager);

   let addr = SocketAddr::from(([127,0,0,1], 3000)); 
   let listner = tokio::net::TcpListener::bind(addr).await.unwrap();
   axum::serve(listner, app).await.unwrap();
}


pub async fn healthcheck()-> &'static str{
    "Health check done"
}