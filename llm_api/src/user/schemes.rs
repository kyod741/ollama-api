use ::serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct JWTPayload {
    pub iat: u64,
    pub exp: u64,
}
