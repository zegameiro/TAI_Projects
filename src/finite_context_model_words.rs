use std::collections::HashMap;
use rand::Rng;
use serde::{Deserialize, Serialize};

/*
 * Defines the finite-context model structure,
 * storing model parameters and frequency counts
*/
#[derive(Serialize,Deserialize)]
pub struct FiniteContextModelWords {
    k: usize,                                       // context length (Order of the Markov model)
    alpha: f64,                                     // smoothing factor to avoid zero probabilities
    current_context: Vec<String>,                     
    symbols: Vec<String>,                           
    counts: HashMap<String, HashMap<String, usize>>,
        // The outer hashmap maps a context or a substring of length k to the inner hashmap
        // The inner hashmap counts the occurences of characters appearing after the context
}

impl FiniteContextModelWords {

    pub fn new (k: usize, alpha: f64) -> Self {
        Self {
            k,
            alpha,
            current_context: Vec::new(),
            symbols: Vec::new(),
            counts: HashMap::new(),
        }
    }

    /*
     * Processes the input text to populate 
     * the frequency table for context-symbol
     * occurrences
    */
    pub fn train_word(&mut self, current_word: &String) {
        if !self.symbols.contains(current_word) {
            self.symbols.push(current_word.clone());
        }
    
        if self.current_context.len() >= self.k {
            let context = self.current_context.join(" ");
            
            // Insert the count into the HashMap
            let entry = self.counts.entry(context).or_insert_with(HashMap::new);
            *entry.entry(current_word.clone()).or_insert(0) += 1;
            
            // Slide the context window (remove the first char)
            self.current_context.remove(0);
        }
    
        // Append the new character to the context
        self.current_context.push(current_word.clone());
    }
    

    /*
     * Computes the smoothed probability 
     * of a symbol following a given context
     * using the stored counts
    */
    pub fn compute_probability(&self, context: &str, word: &String) -> f64 {
        let binding: HashMap<String, usize> = HashMap::new();
        let symbol_counts: &HashMap<String, usize> = self.counts.get(context).unwrap_or(&binding);
        let symbol_count: f64 = *symbol_counts.get(word).unwrap_or(&0) as f64;
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
        let words: Vec<String> = text.split_whitespace().map(|s| s.to_string()).collect();
    
        for i in 0..words.len().saturating_sub(self.k) {
            // Join the context words into a single string
            let context: String = words[i..i + self.k].join(" ");
            let next_word = words.get(i + self.k).cloned().unwrap_or_default();
            let probability = self.compute_probability(&context, &next_word);
            total_info += -probability.log2();
        }
    
        total_info
    }
    

    /*
     * Samples a character based on stored probabilities
     * with frozen counts
    */
    pub fn sample_next_word(&self, context: &str) -> String {
        let mut rng = rand::rng(); // Random number generator
        let binding: HashMap<String, usize> = HashMap::new();
        let symbol_counts = self.counts.get(context).unwrap_or(&binding);

        let total_count: f64 = symbol_counts.values().sum::<usize>() as f64;

        if total_count == 0.0 {
            let index = rng.random_range(0..self.symbols.len());
            return self.symbols[index].clone();
        }

        let mut cumulative_probability = 0.0;
        let threshold = rng.random::<f64>();

        for (symbol, &count) in symbol_counts {
            cumulative_probability += (count as f64) / total_count;
            if threshold <= cumulative_probability {
                return symbol.clone();
            }
        }

        let index = rng.random_range(0..self.symbols.len());
        return self.symbols[index].clone();
    }

    pub fn get_k(&self) -> usize {
        self.k
    }
}