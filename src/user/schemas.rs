use ::serde::{Deserialize, Serialize};

pub enum format{
    json
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JWTPayload {
    pub iat: u64,
    pub exp: u64,
}

pub struct CompletionRequest {

    pub model: String,
    pub prompt: Option<String>,
    pub suffix: Option<String>,

    //these are advanced parameters

    format: Option<format>,
    options: Option<String>, // todo validate json
    system: Option<String>,
    template: Option<String>,
    context: Option<String>,
    stream: Option<bool>,
    raw: Option<bool>,
    keep_alive: Option<string>,

}

