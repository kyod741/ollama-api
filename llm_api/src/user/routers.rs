use rocket::serde::{Deserialize, json::Json};

#[derive(Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct GenerateRequest{
    model: String,
    prompt: String
}

#[get("/", data = "<request>")]
pub fn generate(request: Json<GenerateRequest>){
    println!("model {0:?} \n prompt {1:?}", request.model, request.prompt);
}
