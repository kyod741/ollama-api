#[macro_use]
extern crate rocket;

pub mod user;

use dotenv::dotenv;
use rocket::{Build, Rocket};
use std::env::args;

const INFO: &str = "
Usage
    llm_api [command]

Available Commands:
    help print help
    new create a new token and print it
    launch launches the api
";

fn help() {
    println!("{INFO}");
}

fn rocket() -> Rocket<Build> {
    rocket::build().mount("/api", routes![user::routers::completion_request, user::routers::chat_completion_request])
}

#[rocket::main]
async fn main() {
    dotenv().ok();
    let args: Vec<String> = args().collect();
    if args.len() >= 2 && args[1] == "new" {
        let token: String = user::utils::generate_token().unwrap();
        println!("{token}");
    } else if (args.len() >= 2 && args[1] == "help") || args.len() == 1 {
        help();
    } else if args.len() >= 2 && args[1] == "launch" {
        let _ = rocket().launch().await;
    } else {
        println!("could not recognize any command");
        help();
    }
}
