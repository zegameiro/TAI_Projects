use std::collections::HashMap;
use rand::Rng;
use serde::{Deserialize, Serialize};

/*
 * Defines the finite-context model structure,
 * storing model parameters and frequency counts
*/
#[derive(Serialize,Deserialize)]
pub struct FiniteContextModel {
    k: usize,                                       // context length (Order of the Markov model)
    alpha: f64,                                     // smoothing factor to avoid zero probabilities
    current_context: String,                     
    symbols: Vec<char>,                           
    pub counts: HashMap<String, HashMap<char, usize>>,
        // The outer hashmap maps a context or a substring of length k to the inner hashmap
        // The inner hashmap counts the occurences of characters appearing after the context
}

impl FiniteContextModel {

    pub fn new (k: usize, alpha: f64) -> Self {
        Self {
            k,
            alpha,
            current_context: String::new(),
            symbols: Vec::new(),
            counts: HashMap::new(),
        }
    }

    /*
     * Processes the input text to populate 
     * the frequency table for context-symbol
     * occurrences
    */
    pub fn train_char(&mut self, current_char: char) {
        if !self.symbols.contains(&current_char) {
            self.symbols.push(current_char);
        }
    
        if self.current_context.len() >= self.k {
            let context = self.current_context.clone();
            
            // Insert the count into the HashMap
            let entry = self.counts.entry(context).or_insert_with(HashMap::new);
            *entry.entry(current_char).or_insert(0) += 1;
            
            // Slide the context window (remove the first char)
            self.current_context.remove(0);
        }
    
        // Append the new character to the context
        self.current_context.push(current_char);
    }
    

    /*
     * Computes the smoothed probability 
     * of a symbol following a given context
     * using the stored counts
    */
    pub fn compute_probability(&self, context: &str, symbol: char) -> f64 {
        let binding: HashMap<char, usize> = HashMap::new();
        let symbol_counts: &HashMap<char, usize> = self.counts.get(context).unwrap_or(&binding);
        let symbol_count: f64 = *symbol_counts.get(&symbol).unwrap_or(&0) as f64;
        let total_count: f64 = symbol_counts.values().sum::<usize>() as f64;

        (symbol_count + self.alpha) / (total_count + self.alpha * 256 as f64)
    }

    /*
     * Calculates the average information
     * content of a text using the trained
     * finite-context model
    */
    pub fn calculate_information_content(&self, text: &str) -> f64 {
        let mut total_info = 0.0;
        let chars: Vec<char> = text.chars().collect();
    
        for i in 0..chars.len().saturating_sub(self.k) {
            let context: String = chars[i..i + self.k].iter().collect(); 
            let next_char = chars.get(i + self.k).copied().unwrap_or('\0');
            let probability = self.compute_probability(&context, next_char);
            total_info += -probability.log2();
        }
    
        total_info
    }
    

    /*
     * Samples a character based on stored probabilities
     * with frozen counts
    */
    pub fn sample_next_char(&self, context: &str) -> char {
        let mut rng = rand::rng(); // Random number generator
        let binding: HashMap<char, usize> = HashMap::new();
        let symbol_counts = self.counts.get(context).unwrap_or(&binding);

        let total_count: f64 = symbol_counts.values().sum::<usize>() as f64;

        if total_count == 0.0 {
            return ' '; // default fallback
        }

        let mut cumulative_probability = 0.0;
        let threshold = rng.random::<f64>();

        for (&symbol, &count) in symbol_counts {
            cumulative_probability += (count as f64) / total_count;
            if threshold <= cumulative_probability {
                return symbol;
            }
        }

        ' ' // Fallback
    }

    pub fn get_k(&self) -> usize {
        self.k
    }
}