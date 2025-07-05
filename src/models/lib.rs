pub fn run_model(data: Vec<f64>) -> f64 {
    println!("🤖 Running neural model on data: {:?}", data);
    // Mock: Predict the next value (e.g., next closing price)
    let prediction = data.iter().sum::<f64>() / data.len() as f64;
    println!("📈 Prediction result: {}", prediction);
    prediction
}

