use std::collections::{HashMap, HashSet, VecDeque};
use plotters::prelude::LogScalable;
use serde::{Deserialize, Serialize};
use opencv::core::{Mat, MatTraitConst};

/*
 * Defines the finite-context model structure,
 * storing model parameters and frequency counts
*/
#[derive(Serialize,Deserialize)]
pub struct FiniteContextModelImage {                                   // context length (Order of the Markov model)
    alpha: f64,                                     // smoothing factor to avoid zero probabilities
    current_context: VecDeque<u8>,                     
    symbols: HashSet<u8>,                           
    counts: HashMap<Vec<u8>, HashMap<u8, i32>>,
        // The outer hashmap maps a context or a substring of length k to the inner hashmap
        // The inner hashmap counts the occurences of characters appearing after the context
}

impl FiniteContextModelImage {

    pub fn new (alpha: f64) -> Self {
        Self {
            alpha,
            current_context: VecDeque::new(),
            symbols: HashSet::new(),
            counts: HashMap::new(),
        }
    }

    /*
     * Processes the input text to populate 
     * the frequency table for context-symbol
     * occurrences
    */
    pub fn train_mat_image(&mut self, mat_image: Mat, k:u8) {
        let cols = mat_image.cols();
        let rows = mat_image.rows();

        for r in 0..rows-1{
            for c in 0..cols-1{
                let context = self.get_context(r, c, &mat_image,k);
                let pixel:&u8 = mat_image.at_2d::<u8>(r, c).unwrap();
                
                self.symbols.insert(*pixel);
                let entry = self.counts.entry(context.into()).or_insert_with(HashMap::new);
                *entry.entry(*pixel).or_insert(0) += 1;
            }
        }
    }

    pub fn get_context(&self,row:i32,col:i32,mat_image: &Mat, k:u8) -> Vec<u8>{
        let pixeis:Vec<[i32;2]>;
        let rows = mat_image.rows();
        let cols = mat_image.cols();
        match k {
            2 => pixeis = vec![[row-1,col],[row,col-1]],
            4 => pixeis = vec![[row-1,col],[row,col-1],[row-1,col-1],[row-1,col+1]],
            6 => pixeis = vec![[row-1,col],[row,col-1],[row-1,col-1],[row-1,col+1],[row-2,col],[row,col-2]],
            _ => pixeis = vec![[row-1,col],[row,col-1]],
        }
        let mut return_vec = vec![0u8; k as usize];
        for (i,pixel) in pixeis.iter().enumerate(){
            if self.is_in_frame(*pixel, rows, cols) {
                return_vec[i] = *mat_image.at_2d::<u8>(pixel[0],pixel[1]).unwrap();
            }else {
                return_vec[i] = *mat_image.at_2d::<u8>(row,col).unwrap();
            }
        }
        return_vec
    }

    fn is_in_frame(&self, pixel: [i32;2], rows:i32, cols:i32) -> bool {
        if pixel[0] > 0 && pixel[0] < rows && pixel[1] > 0 && pixel[1] < cols {
            return  true;
        }
        false
    }
    

    /*
     * Computes the smoothed probability 
     * of a symbol following a given context
     * using the stored counts
    */
    pub fn compute_probability(&self, context: &[u8] , pixel: u8) -> f64 {
        let binding = HashMap::new();
        let symbol_counts = self.counts.get(context).unwrap_or(&binding);
        let symbol_count: f64 = symbol_counts.get(&pixel).copied().unwrap_or(0) as f64;
        let total_count: f64 = symbol_counts.values().sum::<i32>() as f64;

        (symbol_count + self.alpha) / (total_count + self.alpha * self.symbols.len().as_f64())
    }

    /*
     * Calculates the average information
     * content of a text using the trained
     * finite-context model
    */
    pub fn calculate_information_content(&self, mat_image: &Mat, k:u8) -> f64 {
        let cols = mat_image.cols();
        let rows = mat_image.rows();
        let mut total_info = 0.0;

        for r in 0..rows{
            for c in 0 ..cols{
                let context = self.get_context(r, c, &mat_image,k);
                let pixel:&u8 = mat_image.at_2d::<u8>(r, c).unwrap();
                let probability = self.compute_probability(&context, *pixel);
                total_info += -probability.log2();
            }
        }
        total_info
    }
}