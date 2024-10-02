use crate::user::schemas::{ChatCompletionRequestBody, CompletionRequestBody};
use std::env;

pub async fn ollama_completion_request(
    payload: &CompletionRequestBody,
) -> Result<String, reqwest::Error> {
    let client = reqwest::Client::new();
    let response = client
        .post(format!(
            "{}/api/generate",
            env::var("OLLAMA_API_ADDRESS").expect("OLLAMA_API_ADDRESS must be set")
        ))
        .body(payload.0.clone())
        .send()
        .await?
        .error_for_status()?;
    let content = response.text().await?;
    Ok(content)
}

pub async fn ollama_chat_completion_request(
    payload: &ChatCompletionRequestBody,
) -> Result<String, reqwest::Error> {
    let client = reqwest::Client::new();
    let response = client
        .post(format!(
            "{}/api/chat",
            env::var("OLLAMA_API_ADDRESS").expect("OLLAMA_API_ADDRESS must be set")
        ))
        .body(payload.0.clone())
        .send()
        .await?
        .error_for_status()?;
    let content = response.text().await?;
    Ok(content)
}
