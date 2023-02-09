use dotenv::dotenv;
use std::env;

pub fn work() {
    dotenv().expect("Oops, didnt work");

    let key = "MY_VAR";
    match env::var(key) {
        Ok(val) => println!("{key}: {val:?}"),
        Err(e) => println!("couldn't interpret {key}: {e}"),
    }
}
