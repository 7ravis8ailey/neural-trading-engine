mod mcp;

use mcp::preprocessing::{validate_input, fetch_and_prepare_data, run_model};

#[tokio::main]
async fn main() {
    let symbol = "AAPL";

    if !validate_input(symbol) {
        eprintln!("❌ Invalid symbol: {}", symbol);
        return;
    }

    match fetch_and_prepare_data(symbol).await {
        Ok(data) => {
            let prediction = run_model(data);
            println!("💡 Final prediction for {}: {:.2}", symbol, prediction);
        }
        Err(e) => {
            eprintln!("🚨 Error fetching data for {}: {}", symbol, e);
        }
    }
}

