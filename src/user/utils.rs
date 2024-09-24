use crate::user::schemas;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use std::env;
use std::time::{SystemTime, UNIX_EPOCH};

pub fn generate_token() -> Result<String, jsonwebtoken::errors::Error> {
    let header = Header {
        alg: Algorithm::HS256,
        ..Default::default()
    };
    let expiry_time_var: &str =
        &env::var("EXPIRY_TIME_IN_SECONDS").expect("EXPIRY_TIME_IN_SECONDS must be set");

    let expiry_time_in_seconds: u64 = expiry_time_var
        .replace("_", "")
        .parse::<u64>()
        .expect("Could not parse EXPIRY_TIME_IN_SECONDS");
    let iat: u64 = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let exp: u64 = iat + expiry_time_in_seconds;
    let payload: schemas::JWTPayload = schemas::JWTPayload { iat, exp };
    let token = encode(
        &header,
        &payload,
        &EncodingKey::from_secret(
            env::var("JWT_SECRET")
                .expect("JWT_SECRET must be set")
                .as_ref(),
        ),
    );

    token
}

pub fn validate_token(token: &str) -> Result<(), jsonwebtoken::errors::Error> {
    let decoded_token = decode::<schemas::JWTPayload>(
        token,
        &DecodingKey::from_secret(
            env::var("JWT_SECRET")
                .expect("JWT_SECRET must be set")
                .as_ref(),
        ),
        &Validation::default(),
    );
    match decoded_token {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}
