// cargo run --example dotenv
use dotenv::dotenv;

#[macro_use]
extern crate dotenv_codegen;

fn main() {
    dotenv().ok();

    /*
    // Grabs all env variables, even those not in .env
    for (key, value) in env::vars() {
        println!("{}: {}", key, value);
    }
    */

    println!("email: {}", dotenv!("EMAIL"));
    println!("password: {}", dotenv!("PASSWORD"));
}
