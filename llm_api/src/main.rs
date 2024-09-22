use std::env::args;
use rocket::{Rocket, Build};
#[macro_use] extern crate rocket;
mod routers;
mod utils;

const INFO: &str = "
Usage
    llm_api [command]

Available Commands:
    help print help
    new create a new token and print it
    launch launches the api
";

fn help() -> (){
    println!("{INFO}");
}

fn new_token() -> String{
    utils::generate_token().unwrap()
}

fn rocket() -> Rocket<Build>{
    rocket::build().mount("/", routes![routers::generate])
}
#[rocket::main]
async fn main(){
    let args: Vec<String> = args().collect();
    if args.len() >= 2 && args[1] == "new" {
        let token: String = new_token();
        println!("{token}");
    }else if (args.len() >=2 && args[1] == "help") || args.len() == 1 {
        help();
    }else if args.len() >=2 && args[1] == "launch"{
        let _ = rocket().launch().await; 
    }else{
        println!("could not recognize any command");
        help();
    }
}

