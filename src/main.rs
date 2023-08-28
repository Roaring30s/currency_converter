use std::env;
mod lib;

fn main() {

    let args: Vec<String> = env::args().collect();

    if args.len() > 2 {
        panic!("Only one currency please");
    }

    let selected_currency = args[1].to_uppercase(); 

    if !lib::CURRENCIES.iter().any(|&currency| currency == selected_currency) {
        println!("Pick another supported currency or cargo run --help");
    }

    let rate = lib::calculate_exchange_rate(&selected_currency);
    println!("1 Euro is {} {}", rate, selected_currency);

}