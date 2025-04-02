use std::collections::HashMap;

/// Converts an audio sample to a discrete integer value
/// Arguments:
/// * `sample`: A float value typically between -1.0 and 1.0 representing an audio sample
/// * `quantization_levels`: The number of discrete levels to quantize the sample into
pub fn quantize_audio_sample(sample: f32, quantization_levels: usize) -> usize {
    let normalized = (sample + 1.0) / 2.0;  // Map from [-1,1] to [0,1]
    let quantized = (normalized * (quantization_levels as f32)).floor() as usize;
    // Ensure the value is within bounds
    quantized.min(quantization_levels - 1)
}

pub struct AudioFiniteContextModel {
    k: usize,
    alpha: f64,
    current_context: Vec<usize>,
    counts: HashMap<Vec<usize>, HashMap<usize, usize>>,
    quantization_levels: usize,
}

impl AudioFiniteContextModel {
    
    pub fn new(k: usize, alpha: f64, quantization_levels: usize) -> Self {
        Self {
            k,
            alpha,
            quantization_levels,
            current_context: Vec::with_capacity(k),
            counts: HashMap::new(),
        }
    }

    /// Updates the model's frequency counts with a single audio sample.
    /// Arguments:
    /// * `sample`: A float value representing an audio sample
    pub fn train_sample(&mut self, sample: f32) {
        let quantized = quantize_audio_sample(sample, self.quantization_levels);

        if self.current_context.len() >= self.k {
            let context = self.current_context.clone();
            
            let entry = self.counts.entry(context).or_insert_with(HashMap::new);
            *entry.entry(quantized).or_insert(0) += 1;
            
            self.current_context.drain(..1);
        }

        self.current_context.push(quantized);
    }

    /// Train the model on a batch of audio samples
    /// Arguments:
    /// * `samples`: A slice of float values representing audio samples
    pub fn train_audio_batch(&mut self, samples: &[f32]) {
        for &sample in samples {
            self.train_sample(sample);
        }
    }

     /// Computes the smoothed probability of a symbol following a given context using the stored counts
     /// Arguments:
     /// * `context`: A sequence of previous quantized audio values
     /// * `symbol`: The quantized audio value to calculate probability for
    pub fn compute_probability(&self, context: &[usize], symbol: usize) -> f64 {
        let binding = HashMap::new();
        let symbol_counts: &HashMap<usize, usize> = self.counts.get(context).unwrap_or(&binding);
        let symbol_count: f64 = symbol_counts.get(&symbol).copied().unwrap_or(0) as f64;
        let total_count: f64 = symbol_counts.values().sum::<usize>() as f64;

        (symbol_count + self.alpha) / (total_count + self.alpha * self.quantization_levels as f64)
    }

    /// Calculate the bits needed to encode an audio file using this model
    /// Arguments:
    /// * `audio_samples`: Audio data to encode
    pub fn calculate_encoding_bits(&self, audio_samples: &[f32]) -> f64 {
        let mut total_bits = 0.0;
        let mut context = Vec::with_capacity(self.k);
        
        // Initialize context with the first k samples if possible
        for i in 0..self.k.min(audio_samples.len()) {
            let quantized = quantize_audio_sample(audio_samples[i], self.quantization_levels);
            context.push(quantized);
            // We're not calculating bits for the initial context
        }
        
        // Calculate bits for the remaining samples
        for i in self.k..audio_samples.len() {
            let quantized = quantize_audio_sample(audio_samples[i], self.quantization_levels);
            let probability = self.compute_probability(&context, quantized);
            total_bits += -probability.log2();
            
            // Update context window
            context.remove(0);
            context.push(quantized);
        }
        
        total_bits
    }

    /// Calculate Normalized Relative Compression (NRC) for audio data
    /// Arguments:
    /// * `audio_samples`: Audio data to analyze
    pub fn calculate_nrc(&self, audio_samples: &[f32]) -> f64 {
        let bits_needed = self.calculate_encoding_bits(audio_samples);
        let sample_count = audio_samples.len() as f64;
        let bits_per_symbol = (self.quantization_levels as f64).log2();
        
        bits_needed / (sample_count * bits_per_symbol)
    }

    pub fn get_k(&self) -> usize {
        self.k
    }
    
    pub fn get_quantization_levels(&self) -> usize {
        self.quantization_levels
    }

}