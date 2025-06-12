use argparse::{ArgumentParser, Store};
use opencv::{
    imgcodecs, prelude::*, Result
};
use tai_projects::{image_processor::{quantize_image, ImageProcessor}, ncd::compute_ncd};

fn main() -> Result<()>{
    let mut alpha = 0.5;
    let mut database_path = "".to_string();
    let mut image_path = "".to_string();
    let mut top_sequences = 10;
    let mut levels:i32 = 256;
    let mut compressor = "".to_string();

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

        argument_parser.refer(&mut compressor)
            .add_option(&["-c", "--compressor"], Store, "Compressor to use: gz, bz2, xz, zstd, lzma (optional)");


        argument_parser.parse_args_or_exit();
    }

    let images = ImageProcessor::new(database_path.as_str());

    
    let mut image = imgcodecs::imread(image_path.as_str(), imgcodecs::IMREAD_GRAYSCALE)?;
    
    quantize_image(&mut image, levels);
    
    if image.empty() {
        panic!("Could not open or find the image!");
    }
    
    let k = 4;

    // 👇 Use weighted NRC method here
    let ncd_scores = if !compressor.is_empty() {
        // Read and quantize reference image
        let mut ref_image = imgcodecs::imread(image_path.as_str(), imgcodecs::IMREAD_GRAYSCALE)?;
        quantize_image(&mut ref_image, levels);
        let ref_str = mat_to_string(&ref_image);

        let mut scores = std::collections::HashMap::new();
        for file in &images.images_list {
            let mut img = imgcodecs::imread(file, imgcodecs::IMREAD_GRAYSCALE)?;
            quantize_image(&mut img, levels);
            let img_str = mat_to_string(&img);
            let score = compute_ncd(&ref_str, &img_str, &compressor);
            scores.insert(file.clone(), score);
        }
        scores
    } else {
        images.compute_ncd(alpha, k, levels, &image_path)
    };

    let mut sorted: Vec<(&String, &f64)> = ncd_scores.iter().collect();
    sorted.sort_by(|a, b| a.1.partial_cmp(b.1).unwrap());

    for (key, value) in sorted.iter().take(top_sequences) {
        println!("{}: {:.6}", key, value);
    }

    Ok(())
}

fn mat_to_string(mat: &Mat) -> String {
    let mut result = String::new();
    for row in 0..mat.rows() {
        for col in 0..mat.cols() {
            let val = *mat.at_2d::<u8>(row, col).unwrap();
            result.push(val as char);
        }
    }
    result
}
