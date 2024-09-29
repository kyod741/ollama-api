use crate::user::schemas::{CompletionRequestBody, CompletionRequestHeaders, Wrapper};

#[get("/", data = "<request>")]
pub fn completion_request(
    token: Wrapper<CompletionRequestHeaders>,
    request: Wrapper<CompletionRequestBody>,
) {
    let unwrapped_token = token.0;
    println!("{request:#?}");
    println!("{unwrapped_token:#?}");
}
