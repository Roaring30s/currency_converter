use reqwest;
use std::env;
use dotenv::dotenv;
use serde::Deserialize;

/*
 * derive - forces traits into your struct
 * Debug - allows you to print {:?} struct for debugging purposes
 * Deserialize - grabs this trait from serde that will then map the json keys to your struct keys
 *    a. the opposite is serialization which converts easy-to-read data into JSON, XML or binary data
*/
#[derive(Debug, Deserialize)]
struct RatesBook {
    success: bool,
    timestamp: u64,
    base: String,
    date: String,
    rates: std::collections::HashMap<String, f64>,
}

pub const CURRENCIES: [&str; 3] = ["COP", "USD", "AED"];
const FOREX_ENDPOINT: &str = "http://data.fixer.io/api/latest?access_key=";

fn obtain_forex_key() -> String {
    dotenv().ok(); //loads .env info into rust environment
    let key = env::var("FOREX_KEY").expect("Issue with Forex Env Var");
    
    key
}

pub fn calculate_exchange_rate(currency: &String) -> String {
    
    let fx_key = obtain_forex_key();
    let error = "There was an error calculating the exchange rate.".to_string();
    let url = FOREX_ENDPOINT.to_string()+&fx_key;

    // API Request
    let resp = reqwest::blocking::get(url).unwrap();
    
    // Parse
    if resp.status().is_success() {
        /*
         * To use Json
         * 1. Have a struct declaring what the returned JSON will look like & deserialize it
         *    a. Deserialize? Grabs messy data json and organizes into Rust types i.e. structs
         * 2 . Include the json feature in reqwest with the blocking in Cargo.toml
         */
        //response data being deserialized to json in .json, then deserialized to your struct
        if let Ok(parsed_json) = resp.json::<RatesBook>() { //telling reqwest library to convert the JSON to RatesBook type using the deserialization method provided by serde json
            match parsed_json.rates.get(currency) {
                Some(rate) => rate.to_string(),
                None => error,
            }
        } else {
            error
        }
    } else {
        error
    }
}




/*

2. Implement help flag

3. What is a better way than unwrap

*/