use crate::user::utils::validate_token;
use ::serde::{Deserialize, Serialize};
use rocket::outcome::Outcome;
use rocket::data::{self, FromData, ToByteUnit};
use std::env;
use rocket::request::{self, FromRequest};
use rocket::{http::Status, Data, Request};

#[derive(Debug, Serialize, Deserialize)]
pub struct JWTPayload {
    pub iat: u64,
    pub exp: u64,
}

#[derive(Debug)]
pub enum JWTTokenError {
    Missing,
    Invalid,
}

#[derive(Debug)]
pub enum PayloadError {
    Invalid,
    TooLarge,
    Io(std::io::Error),
}

#[derive(Debug)]
pub struct CompletionRequestBody{
    payload: String,
}

#[derive(Debug)]
pub struct CompletionRequestHeaders{
    token: String,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for CompletionRequestHeaders {
    type Error = JWTTokenError;
    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        let token_option = request.headers().get_one("Authorization");
        if let Some(token) = token_option {
            if token.len() > 6 && &token[0..7] == "Bearer " {
                let extracted_token = &token[7..];
                match validate_token(&extracted_token) {
                    Ok(_) => Outcome::Success(CompletionRequestHeaders{token: extracted_token.to_string()}),
                    Err(_) => Outcome::Error((Status::Unauthorized, JWTTokenError::Invalid)),
                }
            } else {
                Outcome::Error((Status::Unauthorized, JWTTokenError::Invalid))
            }
        } else {
            Outcome::Error((Status::Unauthorized, JWTTokenError::Missing))
        }
    }
}

#[rocket::async_trait]
impl<'r> FromData<'r> for CompletionRequestBody{
    type Error = PayloadError;
    async fn from_data(_request: &'r Request<'_>, data: Data<'r>) -> data::Outcome<'r, Self> {
        use PayloadError::*;
        let size_limit: u64 = env::var("STRING_SIZE_LIMIT_IN_BYTES")
            .expect("STRING_SIZE_LIMIT_IN_BYTES must be set")
            .replace("_", "")
            .parse::<u64>()
            .expect("STRING_SIZE_LIMIT_IN_BYTES must be a valid integer");

        match data
            .open(size_limit.bytes())
            .into_string().await {
                Ok(string) if string.is_complete() => Outcome::Success(CompletionRequestBody{payload: string.into_inner()}),
                Ok(_) => Outcome::Error((Status::PayloadTooLarge, TooLarge)),
                Err(e) => Outcome::Error((Status::InternalServerError, Io(e)))
            }
        
    }
}

pub struct ChatCompletionRequest(String);
