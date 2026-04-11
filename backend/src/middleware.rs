use axum::{extract::{Request, State}, http::StatusCode, middleware::Next, response::Response};
use crate::jwt::verify_token;
use jsonwebtoken::errors::Error;
use std::sync::{Arc, Mutex};
use crate::EngineManager;

// we check the token from request headers and verify it 
pub async fn auth_middleware(State(manager): State<Arc<Mutex<EngineManager>>>, mut request: Request, next: Next)-> Result<Response, StatusCode>{

    let token = request.headers().get("Authorization").map(|value| value.to_str().unwrap().to_string());

    match token {
        Some(token) => {
            match verify_token(token).await {
                Ok(claims) => {
                    // we extract the user id from the token claims and put it in request 
                    request.extensions_mut().insert(claims.sub);
                    Ok(next.run(request).await)
                }
                Err(_)=> Err(StatusCode::UNAUTHORIZED)
            }
        }
        None => Err(StatusCode::UNAUTHORIZED)
    }
}


