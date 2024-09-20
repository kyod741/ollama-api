use::serde::{Serialize, Deserialize};
use jsonwebtoken::{encode, decode, Header, Algorithm, Validation, EncodingKey, DecodingKey};
use std::time::{SystemTime, UNIX_EPOCH};

const EXPIRY_TIME_IN_SECONDS: u64 = 31_540_000; // that is one year
const SECRET: &str = "alko1666witwitwitareczek404zmyslypyslyslawek123";

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    exp: u64,
}

pub fn generate_token() -> Result<String, jsonwebtoken::errors::Error>{
    let header = Header {
        alg: Algorithm::HS256,
        ..Default::default()
    };
    let expiry_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() + EXPIRY_TIME_IN_SECONDS;
    let claims: Claims = Claims {
        exp: expiry_time
    };
    let token = encode(&header, &claims, &EncodingKey::from_secret(SECRET.as_ref()));
    
    token
}

pub fn validate_token(token: &str) -> Result<(), jsonwebtoken::errors::Error>{
    let decoded_token = decode::<Claims>(&token, &DecodingKey::from_secret(SECRET.as_ref()), &Validation::default());
    match decoded_token {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}
