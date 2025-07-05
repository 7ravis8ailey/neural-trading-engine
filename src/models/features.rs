pub fn extract_features(data: &[f64]) -> Vec<f64> {
    println!("🔍 Extracting features from data...");
    // Mock: Return normalized deltas, moving averages, etc.
    data.windows(2)
        .map(|w| w[1] - w[0])
        .collect()
}


