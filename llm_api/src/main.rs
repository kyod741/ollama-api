mod utils;

fn main(){
    let token = utils::generate_token().unwrap();
    println!("{}", token);
}
