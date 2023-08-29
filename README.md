# Rust Currency Exchange Converter

The Rust Currency Exchange Converter is a simple command-line tool that allows you to convert between different currencies using the Euro (EUR) as the base currency. The supported target currencies are Emirati Dirham (AED), Colombian Peso (COP), and US Dollar (USD).

## Usage

1. Obtain an API Key from [fixer.io](https://fixer.io).

2. Create a `.env` file in the root directory of the project and add your API key as an environment variable named `FOREX_KEY`. For example: FOREX_KEY=your_api_key_here

3. To run the converter, you can use the following example commands:

cargo run usd <br />
cargo run aed <br />
cargo run cop
