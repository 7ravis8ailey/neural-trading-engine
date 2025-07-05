use yahoo_finance_api as yahoo;
use chrono::{Duration, Utc};

pub fn validate_input(symbol: &str) -> bool {
    !symbol.trim().is_empty() && symbol.chars().all(|c| c.is_ascii_alphanumeric())
}

pub async fn fetch_and_prepare_data(symbol: &str) -> Result<Vec<f64>, Box<dyn std::error::Error>> {
    let provider = yahoo::YahooConnector::new();
    let end = Utc::now();
    let start = end - Duration::days(30);

    let response = provider.get_quote_history(symbol, start, end).await?;
    let quotes = response.quotes()?;

    let closing_prices: Vec<f64> = quotes.iter().map(|q| q.close).collect();

    if closing_prices.is_empty() {
        Err("No data returned".into())
    } else {
        Ok(closing_prices)
    }
}

pub fn run_model(data: Vec<f64>) -> f64 {
    // Mock prediction: average closing price
    data.iter().sum::<f64>() / data.len() as f64
}
