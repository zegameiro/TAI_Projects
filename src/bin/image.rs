use opencv::{
    prelude::*,
    highgui,
    imgcodecs,
    Result,
};
use tai_first_project::image_processor::ImageProcessor;

use tai_first_project::finite_context_model_byte::FiniteContextModelByte;

fn main() -> Result<()>{
    let k = 6;
    let alpha = 0.01;
    let path = "./data/images";
    let images = ImageProcessor::new(path);

    let mut model = FiniteContextModelByte::new(k, alpha);
    
    let image = imgcodecs::imread("./data/images/yaleB01_P00A-010E-20.png", imgcodecs::IMREAD_COLOR)?;
    
    if image.empty() {
        panic!("Could not open or find the image!");
    }

    let data = image.data_bytes()?;
    for pixel in data.iter(){
        model.train_byte(*pixel);
    }

    let nrc_scores = images.compute_nrc(&model);
    let mut sorted: Vec<(&String, &f64)> = nrc_scores.iter().collect();
    sorted.sort_by(|a, b| a.1.partial_cmp(b.1).unwrap());

    for (key, value) in sorted.iter().take(10) {
        println!("{}: {}", key, value);
    }


    
    Ok(())
}
