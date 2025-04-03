use opencv::{
    prelude::*,
    highgui,
    imgcodecs,
    Result,
};
use tai_first_project::image_processor::ImageProcessor;

use tai_first_project::finite_context_model_image::FiniteContextModelImage;

fn main() -> Result<()>{
    let alpha = 0.5;
    let path = "./data/images";
    let images = ImageProcessor::new(path);

    let mut model = FiniteContextModelImage::new(alpha);
    
    let image = imgcodecs::imread("./data/images/12_2.jpg", imgcodecs::IMREAD_GRAYSCALE)?;
    
    if image.empty() {
        panic!("Could not open or find the image!");
    }

    model.train_mat_image(image);
    
    println!("processing database");

    let nrc_scores = images.compute_nrc(&model);
    let mut sorted: Vec<(&String, &f64)> = nrc_scores.iter().collect();
    sorted.sort_by(|a, b| a.1.partial_cmp(b.1).unwrap());

    for (key, value) in sorted.iter().take(15) {
        println!("{}: {}", key, value);
    }

    Ok(())
}
