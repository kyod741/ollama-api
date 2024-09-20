mod utils;

fn main(){
    let token: String = utils::generate_token().unwrap();
    println!("{}", token);
    let result = utils::validate_token(&token);
    match result{
        Ok(_) => println!("git"),
        Err(e) => println!("{e}"),
    };
}
