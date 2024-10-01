use crate::user::proxy::ollama_completion_request;
use crate::user::schemas::{CompletionRequestBody, CompletionRequestHeaders, Wrapper};
use rocket::http::Status;

#[post("/api/generate", data = "<payload>")]
pub async fn completion_request(
    _token: Wrapper<CompletionRequestHeaders>,
    payload: Wrapper<CompletionRequestBody>,
) -> (Status, String) {
    let unwrapped_payload = payload.0;
    let result = ollama_completion_request(&unwrapped_payload).await;
    match result {
        Ok(res) => return (Status::Ok, res),
        Err(e) => return (Status::BadRequest, e.to_string()),
    };
}
