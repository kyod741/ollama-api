use crate::user::schemas::{CompletionRequestBody, CompletionRequestHeaders};

#[get("/", data = "<request>")]
pub fn completion_request(token: CompletionRequestHeaders, request: CompletionRequestBody) {
    println!("{request:#?}");
    println!("{token:#?}");
}
