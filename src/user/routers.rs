use crate::user::proxy::{ollama_chat_completion_request, ollama_completion_request};
use crate::user::schemas::{
    ChatCompletionRequestBody, ChatCompletionRequestHeaders, CompletionRequestBody,
    CompletionRequestHeaders, Wrapper,
};
use rocket::http::Status;

#[post("/generate", data = "<payload>")]
pub async fn completion_request(
    _token: Wrapper<CompletionRequestHeaders>,
    payload: Wrapper<CompletionRequestBody>,
) -> (Status, String) {
    let unwrapped_payload = payload.0;
    let result = ollama_completion_request(&unwrapped_payload).await;
    match result {
        Ok(res) => (Status::Ok, res),
        Err(e) => (Status::BadRequest, e.to_string()),
    }
}

#[post("/chat", data = "<payload>")]
pub async fn chat_completion_request(
    _token: Wrapper<ChatCompletionRequestHeaders>,
    payload: Wrapper<ChatCompletionRequestBody>,
) -> (Status, String) {
    let unwrapped_payload = payload.0;
    let result = ollama_chat_completion_request(&unwrapped_payload).await;
    match result {
        Ok(res) => (Status::Ok, res),
        Err(e) => (Status::BadRequest, e.to_string()),
    }
}
