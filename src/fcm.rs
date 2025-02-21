use std::collections::HashMap;

/*
 * Defines the finite-context model structure,
 * storing model parameters and frequency counts
*/
pub struct FiniteContextModel {
    k: usize,                                       // context length (Order of the Markov model)
    alpha: f64,                                     // smoothing factor to avoid zero probabilities
    counts: HashMap<String, HashMap<char, usize>>,  // nested hashmap to store symbol occurences based on context

        // The outer hashmap maps a context or a substring of length k to the inner hashmap
        // The inner hashmap counts the occurences of characters appearing after the context
}

impl FiniteContextModel {

    pub fn new (k: usize, alpha: f64) -> Self {
        Self {
            k,
            alpha,
            counts: HashMap::new(),
        }
    }

    /*
     * Processes the input text to populate 
     * the frequency table for context-symbol
     * occurrences
    */
    pub fn train(&mut self, text: &str) {
        for i in 0..text.len().saturating_sub(self.k) { // Avoid overflow until k
            let context: &str = &text[i..i + self.k];
            let next_char: char = text.chars().nth(i + self.k).unwrap_or('\0');

            let entry: &mut HashMap<char, usize> = self.counts.entry(context.to_string()).or_insert_with(HashMap::new);
            *entry.entry(next_char).or_insert(0) += 1;
        }
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

        (symbol_count + self.alpha) / (total_count + self.alpha * 256.0)
    }

    /*
     * Calculates the average information
     * content of a text using the trained
     * finite-context model
    */
    pub fn calculate_information_content(&self, text: &str) -> f64 {
        let mut total_info = 0.0;
        for i in 0..text.len().saturating_sub(self.k) {
            let context = &text[i..i + self.k];
            let next_char = text.chars().nth(i + self.k).unwrap_or('\0');
            let probability = self.compute_probability(context, next_char);
            total_info += -probability.log2();
        }

        total_info / text.len() as f64
    }
}