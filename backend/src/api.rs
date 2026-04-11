use axum::{extract::{Path, State}, Json, response::IntoResponse};
use std::sync::{Arc, Mutex};
use crate::manager::EngineManager;
use crate::models::{Candle};
use crate::engine::OrderBookResponse;
use crate::models::{PlaceOrderRequest, Order, Side};
use crate::models::{RegisterRequest, AuthResponse};
use bcrypt::{hash, verify, DEFAULT_COST};
use crate::jwt::create_jwt;



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
            let readable_trades: Vec<serde_json::Value> = trades.iter().map(|t| {
                serde_json::json!({
                    "price": crate::models::u64_to_human_price(t.price),
                    "qty": crate::models::u64_to_human_qty(t.qty, &payload.symbol),
                    "maker_order_id": t.maker_order_id,
                    "taker_order_id": t.taker_order_id,
                    "timestamp": t.timestamp,
                })
            }).collect();
            Json(serde_json::json!({
                "status": "success",
                "matched_trades": readable_trades, 
                "remaining_qty": crate::models::u64_to_human_qty(remaining.map(|o| o.qty).unwrap_or(0), &payload.symbol)
            }))
        }, 
        Err(_) => todo!()
    }
}


#[axum::debug_handler]
pub async fn signup(State(manager): State<Arc<Mutex<EngineManager>>>, Json(payload): Json<RegisterRequest>)-> impl IntoResponse {
    // first we will hash the password
    let hashed_pasword = hash(&payload.password, DEFAULT_COST).unwrap();

    // now we access the db from our manager
    let db = {
        let mg = manager.lock().unwrap();
        mg.db.clone() // Clone the pool (it's just an Arc internally)
    };

    // now save user to db 
    let data = sqlx::query!(
        "INSERT INTO users (email, password_hash) VALUES ($1, $2) RETURNING id",
        payload.email,
        hashed_pasword
    ).fetch_one(&db).await;

    match data {
        Ok(data)=> { 
            let _ = sqlx::query!(
                "INSERT INTO balances (user_id, asset , amount) VALUES ($1, $2, $3)",
                data.id,
                "USDT",
                1_000_000
            ).execute(&db).await;

            let token = create_jwt(data.id).ok();
            Json(AuthResponse {
                status: "success".to_string(),
                user_id: data.id,
                token: token
            })
        },
        Err(r) => {
            println!("ERROR IS: {:?}", r.as_database_error().unwrap().code());
            let code = r.as_database_error().unwrap().code();
            if  code == Some("23505".into()) {
                return Json(AuthResponse {
                    status: "user already present".to_string(),
                    user_id: 0,
                    token: None
                })
            }
            Json(AuthResponse {
                status: "user already present".to_string(),
                user_id: 0,
                token: None
            })
        }
    }


}

pub async fn signin (State(manager): State<Arc<Mutex<EngineManager>>>, Json(payload): Json<RegisterRequest>)-> impl IntoResponse {
     let db = {
        let mg = manager.lock().unwrap();
        mg.db.clone() // Clone the pool (it's just an Arc internally)
    };

    // and the we check if user is present or not and if present then match user data and comapre password  return the token with other data like user id 
    let user_data = sqlx::query!{
        "SELECT id, password_hash FROM users WHERE email = $1",
        payload.email
    }.fetch_optional(&db).await;
    
    // println!("user data is : {:?}", user_data);
    // user data is : Ok(Some(Record { id: 8, password_hash: "$2b$12$TpYoapM0stbO3EQpDVSnZ.JyPHebBTkvJsxPczBiCpczSsndB8DmW" }))
    // user data is : Ok(None)

    match user_data {
        // now we check if ok the db connection wroked and we got the data 
        // we check if we have data 
        Ok(Some(data))=> {
            // compare the password 
            let is_valid = verify(&payload.password, &data.password_hash).unwrap();

            if is_valid {
                let token = create_jwt(data.id).ok();
                Json(AuthResponse{
                    status: "success".to_string(),
                    user_id: data.id,
                    token: token
                })
            }else {
                Json(AuthResponse{
                    status: "invalid password".to_string(),
                    user_id: 0,
                    token: None
                })
            }

        },
        // or it is empty 
        Ok(None)=> {
            Json(AuthResponse{
                status: "user not found".to_string(),
                user_id: 0,
                token: None
            })
        },
        Err(_)=> {
            Json(AuthResponse{
                status: "error".to_string(),
                user_id: 0,
                token: None
            })
        }
    }

    
}