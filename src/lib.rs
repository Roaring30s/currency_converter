use reqwest;
use std::env;
use dotenv::dotenv;
use serde_json::{Value};

pub const CURRENCIES: [&str; 3] = ["COP", "EUR", "AED"];
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
        let stringified_json = resp.text().unwrap();
        let str_reference: &str = &stringified_json;
        let parsed_json: Value = serde_json::from_str(str_reference).unwrap();
        //refactor me later
        if let Some(rates) = parsed_json.get("rates") {
            if let Some(rate) = rates.get(currency) {
                let captured_rate = rate.clone();
                captured_rate.to_string()
            } else {
                error
            }
        } else {
            error
        }
    } else {
        error
    }
}
