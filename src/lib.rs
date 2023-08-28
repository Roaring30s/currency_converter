use std::env;
use dotenv::dotenv;

pub const CURRENCIES: [&str; 2] = ["COP", "EUR"];

pub fn obtain_forex_key() -> String {
    dotenv().ok(); //loads .env info into rust environment
    let key = env::var("FOREX_KEY").expect("Issue with Forex Env Var");
    
    key
}
