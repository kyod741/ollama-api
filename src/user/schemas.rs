use ::serde::{Deserialize, Serialize};

pub enum Format {
    Json,
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
    pub format: Option<Format>,
    pub options: Option<String>, // todo validate json
    pub system: Option<String>,
    pub template: Option<String>,
    pub context: Option<String>,
    pub stream: Option<bool>,
    pub raw: Option<bool>,
    pub keep_alive: Option<String>,
}
