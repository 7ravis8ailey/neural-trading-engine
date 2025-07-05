mod mcp {
    pub mod preprocessing;
    pub mod validation;
}
mod models {
    pub mod features;
    pub mod lib;
}

use mcp::preprocessing::load_and_prepare_data;
use mcp::validation::validate_input;
use models::features::extract_features;
use models::lib::run_model;

fn main() {
    let symbol = "AAPL";

    if validate_input(symbol) {
        let raw_data = load_and_prepare_data(symbol);
        let features = extract_features(&raw_data);
        let prediction = run_model(features);

        println!("💡 Final prediction for {}: {}", symbol, prediction);
    } else {
        println!("❌ Invalid symbol input.");
    }
}

