use::serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct JWTPayload {
    pub iat: u64,
    pub exp: u64,
}