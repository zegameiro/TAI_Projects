use std::collections::{HashMap, HashSet, VecDeque};
use plotters::prelude::LogScalable;
use serde::{Deserialize, Serialize};

/*
 * Defines the finite-context model structure,
 * storing model parameters and frequency counts
*/
#[derive(Serialize,Deserialize)]
pub struct FiniteContextModelByte {
    k: usize,                                       // context length (Order of the Markov model)
    alpha: f64,                                     // smoothing factor to avoid zero probabilities
    current_context: VecDeque<u8>,                     
    symbols: HashSet<u8>,                           
    counts: HashMap<Vec<u8>, HashMap<u8, i32>>,
        // The outer hashmap maps a context or a substring of length k to the inner hashmap
        // The inner hashmap counts the occurences of characters appearing after the context
}

impl FiniteContextModelByte {

    pub fn new (k: usize, alpha: f64) -> Self {
        Self {
            k,
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
    pub fn train_byte(&mut self, byte: u8) {
        self.symbols.insert(byte);

        if self.current_context.len() >= self.k {
            
            let context = self.current_context.clone();
            
            // Insert the count into the HashMap
            let entry = self.counts.entry(context.into()).or_insert_with(HashMap::new);
            *entry.entry(byte).or_insert(0) += 1;
            
            
            self.current_context.pop_front();
        }

        self.current_context.push_back(byte);
    }
    

    /*
     * Computes the smoothed probability 
     * of a symbol following a given context
     * using the stored counts
    */
    pub fn compute_probability(&self, context: &[u8] , byte: u8) -> f64 {
        let binding = HashMap::new();
        let symbol_counts = self.counts.get(context).unwrap_or(&binding);
        let symbol_count: f64 = symbol_counts.get(&byte).copied().unwrap_or(0) as f64;
        let total_count: f64 = symbol_counts.values().sum::<i32>() as f64;

        (symbol_count + self.alpha) / (total_count + self.alpha * self.symbols.len().as_f64())
    }

    /*
     * Calculates the average information
     * content of a text using the trained
     * finite-context model
    */
    pub fn calculate_information_content(&self, data: &[u8]) -> f64 {
        let mut total_info = 0.0;
        
        for window in data.windows(self.k + 1) {
            if let Some((&next_byte, context_bytes)) = window.split_last() {
                let probability = self.compute_probability(context_bytes, next_byte);
                total_info += -probability.log2();
            }
        }
    
        total_info
    }

    pub fn get_k(&self) -> usize {
        self.k
    }
}