use fann::Fann;

pub fn run_fann_model(input: Vec<f32>) -> f32 {
    // Load pre-trained neural network
    let ann = Fann::from_file("model/net.float") // Replace with your actual model path
        .expect("🧠 Failed to load neural net model");

    // Run prediction
    let output = ann.run(&input);
    output[0] // Return first output neuron
}

