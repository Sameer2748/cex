use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use futures_util::{StreamExt, SinkExt};
use crate::models::BinanceTrade;
use crate::manager::EngineManager;
use std::sync::{Arc, Mutex};
use crate::models::BinanceDepth;

#[derive(Debug, serde::Deserialize)]
pub struct CombinedStream {
    pub stream: String,
    pub data: serde_json::Value, // This holds any type of data
}

pub async fn monitor_binance_trades(manager: Arc<Mutex<EngineManager>>) {
    let url = "wss://stream.binance.com:9443/stream?streams=btcusdt@trade/btcusdt@depth20@100ms";
    
    println!("Connecting to Binance WebSocket");
    let (ws_stream, _) = connect_async(url).await.expect("Failed to connect");
    println!("Connected to Binance!");
    let (_, mut read) = ws_stream.split();

    while let Some(message) = read.next().await {
        // Check if we actually got a valid message
        if let Ok(msg) = message {
            // now we just use the text msg we got as json 
            match msg {
                Message::Text(text) => {
                    if let Ok(wrapper) = serde_json::from_str::<CombinedStream>(&text) {
                        let data_str = wrapper.data.to_string();
                        
                        if wrapper.stream.ends_with("@depth20@100ms") {
                            if let Ok(depth) = serde_json::from_str::<BinanceDepth>(&data_str) {
                                manager.lock().unwrap().sync_depth("BTCUSDT", depth);
                            }
                        } else if wrapper.stream.ends_with("@trade") {
                            if let Ok(trade) = serde_json::from_str::<BinanceTrade>(&data_str) {
                                manager.lock().unwrap().inject_trade("BTCUSDT", trade);
                            }
                        }
                    }
                },
                _ => {} 
            }

        }
    }

} 