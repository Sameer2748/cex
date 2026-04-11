use jsonwebtoken::{encode, Header,EncodingKey, errors::Error };
use crate::models::Claims;
use std::time::{SystemTime, UNIX_EPOCH};
use dotenvy::dotenv;
use std::env;

// get secret 
pub fn get_secret() -> String {
    dotenv().ok();

    let secret_key = env::var("JWT_SECRET").expect("JWT SECRET NOT FOUND IN ENV");

}

pub fn create_jwt(user_id: i32)-> Result<String, Error>{
    
    let secret_key = get_secret();
    let expire = SystemTime::now()
    .duration_since(UNIX_EPOCH).unwrap().as_secs() + 60 * 60 *24 *7;

    let claims = Claims{
        sub: user_id,
        exp: expire as usize // (expected `usize`, found `u64`)
    };


    encode(&Header::default(), &claims, &EncodingKey::from_secret(secret_key.as_ref()))
}

pub async fn verify_token(token: String)-> Result<Claims, Error> {
    let secret_key = get_secret();

    let token_data = decode::<Claims>(
        token, 
        &DecodingKey::from_secret(secret_key.as_ref()), 
        &Validation::default()
    )?;
    Ok(token_data.claims)
}