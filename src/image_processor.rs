use opencv::{imgcodecs, prelude::*};

use std::{collections::HashMap, fs};

use crate::finite_context_model_image::FiniteContextModelImage;

pub struct ImageProcessor {
   pub images_list: Vec<String>
}


impl ImageProcessor{
    pub fn new(path:&str) -> Self{
        let mut images_paths = Vec::new();
        for f in fs::read_dir(path).unwrap(){
            match f{
                Ok(f)=>{
                    if let Some(path_str) = f.path().to_str() {
                        images_paths.push(path_str.to_string());
                    } else {
                        eprintln!("Invalid UTF-8 path: {:?}", f.path());
                    }
                }
                Err(e) => {print!("error{}",e);}
            }
        };

        Self { 
            images_list: images_paths, 
        }
    }

    pub fn get_database(&self) -> &Vec<String> {
        &self.images_list
    }

    pub fn compute_ncr_multi_model(&self, models: &HashMap<u8,FiniteContextModelImage>,levels: i32) -> HashMap<String,f64> {
        let mut aggregated_scores: HashMap<String, Vec<f64>> = HashMap::new();

        for (k, model) in models.iter() {
            let scores = self.compute_nrc(model, levels, *k);
            for (image, score) in scores {
                aggregated_scores
                    .entry(image)
                    .or_insert_with(Vec::new)
                    .push(score);
            }
        }

        // Agora computamos a média ponderada (média simples)
        let mut nrc_scores: HashMap<String, f64> = HashMap::new();
        for (image, scores) in aggregated_scores {
            let sum: f64 = scores.iter().sum();
            let mean = sum / scores.len() as f64;
            nrc_scores.insert(image, mean);
        }

        nrc_scores
    }

    pub fn compute_weighted_nrc(&self, models: &HashMap<u8, FiniteContextModelImage>, levels: i32, gamma: f64) -> HashMap<String, f64> {
        let mut nrc_scores = HashMap::new();
    
        for file in &self.images_list {
            let mut image = imgcodecs::imread(file.as_str(), imgcodecs::IMREAD_GRAYSCALE).unwrap();
            quantize_image(&mut image, levels);
    
            let rows = image.rows();
            let cols = image.cols();
    
            let ks = vec![2, 4, 6];
            let mut weights: HashMap<u8, f64> = ks.iter().map(|&k| (k, 1.0 / ks.len() as f64)).collect();
    
            let mut info_content = 0.0;
    
            for r in 0..rows {
                for c in 0..cols {
                    let mut probs = HashMap::new();
                    let mut new_weights = HashMap::new();
    
                    let pixel = *image.at_2d::<u8>(r, c).unwrap();
    
                    let mut weight_sum = 0.0;
    
                    for &k in &ks {
                        let model = models.get(&k).unwrap();
                        let context = model.get_context(r, c, &image, k);
                        let prob = model.compute_probability(&context, pixel);
                        probs.insert(k, prob);
    
                        let prev_weight = weights.get(&k).copied().unwrap_or(1.0 / ks.len() as f64);
                        let new_weight = prev_weight.powf(gamma) * prob;
                        new_weights.insert(k, new_weight);
                        weight_sum += new_weight;
                    }
    
                    for w in new_weights.values_mut() {
                        *w /= weight_sum;
                    }                    
    
                    weights = new_weights;
    
                    let mixed_prob: f64 = ks.iter()
                        .map(|&k| probs.get(&k).unwrap() * weights.get(&k).unwrap())
                        .sum();
    
                    info_content += -mixed_prob.log2();
                }
            }
    
            let sequence_length = (rows * cols) as f64;
            let normalized_info = info_content / ((levels as f64).log2() * sequence_length);
            nrc_scores.insert(file.clone(), normalized_info);
        }
    
        nrc_scores
    }

    pub fn compute_nrc(&self, model: &FiniteContextModelImage,levels: i32, k:u8 ) -> HashMap<String,f64> {

        let mut nrc_scores: HashMap<String, f64> = HashMap::new();

        for file in &self.images_list {
            let mut image = imgcodecs::imread(file.as_str(), imgcodecs::IMREAD_GRAYSCALE).unwrap();
            quantize_image(&mut image, levels);
            let compress_size = model.calculate_information_content(&image,k);
            let size = image.size().unwrap();
            let sequence_length = (size.width * size.height) as f64;
            let nrc_score = if sequence_length > 0.0 {
                compress_size / ((levels as f64).log2() * sequence_length)
            } else {
                0.0
            };

            nrc_scores.insert(file.clone(), nrc_score);
        }

        nrc_scores
    }

    pub fn compute_ncd(&self, alpha: f64, k: u8, levels: i32, reference_file: &String) -> HashMap<String, f64> {
        let mut ncd_scores: HashMap<String, f64> = HashMap::new();

        let mut ref_image = imgcodecs::imread(reference_file.as_str(), imgcodecs::IMREAD_GRAYSCALE).unwrap();
        quantize_image(&mut ref_image, levels);

        let mut model_cy = FiniteContextModelImage::new(alpha);
        model_cy.train_mat_image(ref_image.clone(), k);
        let c_y = model_cy.calculate_information_content(&ref_image, k);

        for file in &self.images_list {
            let mut image = imgcodecs::imread(file.as_str(), imgcodecs::IMREAD_GRAYSCALE).unwrap();
            quantize_image(&mut image, levels);

            let mut model_cx = FiniteContextModelImage::new(alpha);
            model_cx.train_mat_image(image.clone(), k);
            let c_x = model_cx.calculate_information_content(&image, k);

            let mut images = opencv::core::Vector::<Mat>::new();
            images.push(ref_image.clone());
            images.push(image.clone());

            let mut joint_image = Mat::default();
            opencv::core::vconcat(&images, &mut joint_image).unwrap();

            let mut model_cxy = FiniteContextModelImage::new(alpha);
            model_cxy.train_mat_image(joint_image.clone(), k);
            let c_xy = model_cxy.calculate_information_content(&joint_image, k);

            let ncd = if c_x > 0.0 && c_y > 0.0 {
                (c_xy - c_x.min(c_y)) / c_x.max(c_y)
            } else {
                0.0
            };

            ncd_scores.insert(file.clone(), ncd);
        }

        ncd_scores
    }

}

pub fn quantize_image(img: &mut Mat,levels: i32){
    let mut quantization_levels: Vec<f64> = (0..levels).map(|i| (i as f64) * (256.0 / (levels as f64))).collect();
    let mut prev_quantization_levels = quantization_levels.clone();
    
    let max_iterations = 100;
    let tolerance = 1e-6;
    
    let rows = img.rows();
    let cols = img.cols();
    
    let mut iteration = 0;
    while iteration < max_iterations {
        iteration += 1;

        let mut assignments = vec![0; img.rows() as usize * img.cols() as usize];
        
        for r in 0..rows {
            for c in 0..cols {
                let pixel = *img.at_2d::<u8>(r, c).unwrap() as f64;
                let mut min_dist = f64::MAX;
                let mut assigned_level = 0;

                for (i, &q) in quantization_levels.iter().enumerate() {
                    let dist = (pixel - q).abs();
                    if dist < min_dist {
                        min_dist = dist;
                        assigned_level = i;
                    }
                }

                assignments[(r * cols + c) as usize] = assigned_level;
            }
        }

        let mut new_quantization_levels = vec![0.0; levels as usize];
        let mut counts = vec![0; levels as usize];

        for (i, &assignment) in assignments.iter().enumerate() {
            let pixel = *img.at_2d::<u8>((i as i32 / cols) as i32, (i as i32 % cols) as i32).unwrap() as f64;
            new_quantization_levels[assignment] += pixel;
            counts[assignment] += 1;
        }

        for (i, count) in counts.iter().enumerate() {
            if *count > 0 {
                new_quantization_levels[i] /= *count as f64;
            }
        }

        let max_change = new_quantization_levels.iter()
            .zip(&prev_quantization_levels)
            .map(|(new, old)| (new - old).abs())
            .fold(0.0, f64::max);

        if max_change < tolerance {
            break;
        }

        prev_quantization_levels = new_quantization_levels.clone();

        quantization_levels = new_quantization_levels;
    }

    for r in 0..rows {
        for c in 0..cols {
            let pixel = *img.at_2d::<u8>(r, c).unwrap() as f64;
            let mut min_dist = f64::MAX;
            let mut assigned_level = 0;

            for (i, &q) in quantization_levels.iter().enumerate() {
                let dist = (pixel - q).abs();
                if dist < min_dist {
                    min_dist = dist;
                    assigned_level = i;
                }
            }

            *img.at_2d_mut::<u8>(r, c).unwrap() = quantization_levels[assigned_level] as u8;
        }
    }
}