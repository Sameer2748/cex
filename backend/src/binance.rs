use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use futures_util::{StreamExt, SinkExt};
use crate::models::BinanceTrade;
use crate::manager::EngineManager;
use std::sync::{Arc, Mutex};


pub async fn monitor_binance_trades(manager: Arc<Mutex<EngineManager>>) {
    let url = "wss://stream.binance.com:9443/ws/btcusdt@trade";
    
    println!("Connecting to Binance WebSocket...");
    let (ws_stream, _) = connect_async(url).await.expect("Failed to connect");
    println!("Connected to Binance!");
    let (_, mut read) = ws_stream.split();

    while let Some(message) = read.next().await {
        // Check if we actually got a valid message
        if let Ok(msg) = message {
            // now we just use the text msg we got as json 
            match msg {
                Message::Text(text) => {
                    // we turn the json string to our binance trade struct 
                      if let Ok(trade) = serde_json::from_str::<BinanceTrade>(&text) {
                        let mut mg = manager.lock().unwrap();
                        let symbol = trade.symbol.clone();
                        mg.inject_trade(&symbol, trade);
                        // println!("Injected trade for {} into engine!", symbol);

                    } else {
                        println!("Failed to parse JSON: {}", text);
                    }
                }
                Message::Ping(_) => {
                    // Binance sent a 'Ping', tungstenite usually handles the 'Pong' for us.
                }
                _ => {}
            }
        }
    }

} 