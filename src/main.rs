use std::env;
mod lib;

fn main() {

    let args: Vec<String> = env::args().collect();

    if args.len() > 2 {
        panic!("Only one currency please");
    }

    let selected_currency = args[1].to_uppercase();
    let forex_key = lib::obtain_forex_key(); 
    println!("{}", forex_key);

    //conduct api call using lib code here. 
    
}

/*
 * take in the cmd argument
 * verify if empty or valid supported currency
 * call to free api endpoint to fetch current exchange rate 
 * return exchange rate to the cmd prompt
 * Set a help flag that will return the supported currencies. 
*/