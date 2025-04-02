use opencv::{imgcodecs, prelude::*};

use std::{collections::HashMap, fs};

use crate::finite_context_model_byte::FiniteContextModelByte;

pub struct ImageProcessor {
    images_list: Vec<String>
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

    pub fn compute_nrc(&self, model: &FiniteContextModelByte) -> HashMap<String,f64> {

        let mut nrc_scores: HashMap<String, f64> = HashMap::new();

        for file in &self.images_list {
            let image = imgcodecs::imread(file.as_str(), imgcodecs::IMREAD_COLOR).unwrap();
            let arr: &[u8] = image.data_bytes().unwrap();
            let compress_size = model.calculate_information_content( arr);
            let sequence_length = arr.len() as f64;
            let nrc_score = if sequence_length > 0.0 {
                compress_size / (2.0 * sequence_length)
            } else {
                0.0
            };

            nrc_scores.insert(file.clone(), nrc_score);
        }

        nrc_scores
    }
}