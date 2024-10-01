use crate::user::proxy::ollama_completion_request;
use crate::user::schemas::{CompletionRequestBody, CompletionRequestHeaders, Wrapper};

#[post("/completion", data = "<payload>")]
pub async fn completion_request(
    token: Wrapper<CompletionRequestHeaders>,
    payload: Wrapper<CompletionRequestBody>,
) {
    let unwrapped_payload = payload.0;
    println!("{token:#?}");
    println!("{unwrapped_payload:#?}");
    let result = ollama_completion_request(&unwrapped_payload).await;
    match result {
        Ok(res) => println!("{res}"),
        Err(e) => println!("{e:?}"),
    }
}
