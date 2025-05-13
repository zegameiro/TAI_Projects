use std::collections::HashMap;

use argparse::{ArgumentParser, Store};
use opencv::{
    prelude::*,
    imgcodecs,
    Result,
};
use tai_projects::image_processor::{quantize_image, ImageProcessor};

use tai_projects::finite_context_model_image::FiniteContextModelImage;

fn main() -> Result<()>{
    let mut alpha = 0.5;
    let mut database_path = "".to_string();
    let mut image_path = "".to_string();
    let mut top_sequences = 10;
    let mut levels:i32 = 256;

    {
        let mut argument_parser: ArgumentParser<'_> = ArgumentParser::new();
        argument_parser.set_description("Algorithmic Theory of Information Second Project");

        // Meta file path
        argument_parser.refer(&mut image_path)
            .add_option(&["-i"], Store, "Path to the image file (required)")
            .required();

        // Database file path
        argument_parser.refer(&mut database_path)
            .add_option(&["-d"], Store, "Path to the database file (required)")
            .required();

        // Smoothing parameter - alpha
        argument_parser.refer(&mut alpha)
            .add_option(&["-a"], Store, "Smoothing parameter (default: 0.01 must be 0 <= alpha <= 1)");

        // Top sequences to display
        argument_parser.refer(&mut top_sequences)
            .add_option(&["-t"], Store, "Number of top sequences to display (default: 10 must be 1 <= top_sequences <= 239)");

        argument_parser.refer(&mut levels)
            .add_option(&["-l"], Store, "Number of levels to quantize images (default: 255)");

        argument_parser.parse_args_or_exit();
    }

    let images = ImageProcessor::new(database_path.as_str());

    
    let mut image = imgcodecs::imread(image_path.as_str(), imgcodecs::IMREAD_GRAYSCALE)?;
    
    quantize_image(&mut image, levels);
    
    if image.empty() {
        panic!("Could not open or find the image!");
    }
    
    let mut models: HashMap<u8,FiniteContextModelImage> = HashMap::new();
    for k in [2,4,6]{
        let mut model = FiniteContextModelImage::new(alpha);
        model.train_mat_image(image.clone(),k);
        models.insert(k,model);
    }
    
    println!("Processing database using weighted NRC...");
    let gamma = 0.99;

    // 👇 Use weighted NRC method here
    let nrc_scores = images.compute_weighted_nrc(&models, levels, gamma);

    let mut sorted: Vec<(&String, &f64)> = nrc_scores.iter().collect();
    sorted.sort_by(|a, b| a.1.partial_cmp(b.1).unwrap());

    for (key, value) in sorted.iter().take(top_sequences) {
        println!("{}: {:.6}", key, value);
    }

    Ok(())
}
