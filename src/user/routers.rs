use crate::user::schemas::{CompletionRequestHeaders, CompletionRequestBody};

#[get("/", data = "<request>")]
pub fn completion_request(token: CompletionRequestHeaders, request: CompletionRequestBody) {
    println!("{request:#?}");
    println!("{token:#?}");
}
