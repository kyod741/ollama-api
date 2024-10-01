use crate::user::utils::validate_token;
use ::serde::{Deserialize, Serialize};
use derive_more::From;
use rocket::data::{self, FromData, ToByteUnit};
use rocket::outcome::Outcome;
use rocket::request::{self, FromRequest};
use rocket::http::{Status};
use rocket::{Data, Request};
use std::env;

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

pub trait IsCompletionRequestHeaders {}
pub trait IsCompletionRequestBody {}

#[derive(Debug, From)]
pub struct CompletionRequestBody(pub String);

#[derive(Debug, From)]
pub struct CompletionRequestHeaders {
    pub token: String,
}

#[derive(Debug, From)]
pub struct ChatCompletionRequestBody(pub String);

#[derive(Debug, From)]
pub struct ChatCompletionRequestHeaders {
    pub token: String,
}

impl IsCompletionRequestHeaders for CompletionRequestHeaders {}
impl IsCompletionRequestHeaders for ChatCompletionRequestHeaders {}
impl IsCompletionRequestBody for CompletionRequestBody {}
impl IsCompletionRequestBody for ChatCompletionRequestBody {}

#[derive(Debug)]
pub struct Wrapper<T>(pub T);

#[rocket::async_trait]
impl<'r, T> FromRequest<'r> for Wrapper<T>
where
    T: IsCompletionRequestHeaders + From<String>,
{
    type Error = JWTTokenError;
    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        let token_option = request.headers().get_one("Authorization");
        if let Some(token) = token_option {
            if token.len() > 6 && &token[0..7] == "Bearer " {
                let extracted_token = &token[7..];
                match validate_token(extracted_token) {
                    Ok(_) => Outcome::Success(Wrapper(T::from(extracted_token.to_string()))),
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
impl<'r, T> FromData<'r> for Wrapper<T>
where
    T: IsCompletionRequestBody + From<String>,
{
    type Error = PayloadError;
    async fn from_data(_request: &'r Request<'_>, data: Data<'r>) -> data::Outcome<'r, Self> {
        use PayloadError::*;
        let size_limit: u64 = env::var("STRING_SIZE_LIMIT_IN_BYTES")
            .expect("STRING_SIZE_LIMIT_IN_BYTES must be set")
            .replace("_", "")
            .parse::<u64>()
            .expect("STRING_SIZE_LIMIT_IN_BYTES must be a valid integer");

        match data.open(size_limit.bytes()).into_string().await {
            Ok(string) if string.is_complete() => {
                Outcome::Success(Wrapper(T::from(string.into_inner())))
            }
            Ok(_) => Outcome::Error((Status::PayloadTooLarge, TooLarge)),
            Err(e) => Outcome::Error((Status::InternalServerError, Io(e))),
        }
    }
}
