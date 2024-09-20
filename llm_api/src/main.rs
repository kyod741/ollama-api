use std::env::args;
mod utils;

const INFO: &str = "
Usage
    help:
        prints help
    new:
        creates a new token and prints it
";

fn help() -> (){
    println!("{INFO}");
}

fn new_token() -> String{
    utils::generate_token().unwrap()
}
fn main(){
    let args: Vec<String> = args().collect();
    if args.len() >= 2 && args[1] == "new" {
        let token: String = new_token();
        println!("{token}");
    }else if (args.len() >=2 && args[1] == "help") || args.len() == 1 {
        help();
    }else{
        println!("could not recognize any command");
        help();
    }
}
