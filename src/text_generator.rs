use crate::{finite_context_model::FiniteContextModel, finite_context_model_words::FiniteContextModelWords};
use std::collections::HashMap;

extern crate rand;

pub fn generate_text(models: HashMap<usize, FiniteContextModel>, seed: &str, length: usize, original_k: usize) -> String {
    let mut model: &FiniteContextModel;
    if seed.len() < original_k{
        model = models.get(&seed.len()).unwrap();
    }else {
        model = models.get(&original_k).unwrap();
    }
    let mut generated_text = String::from(seed);
    let mut context: String;
    let k = model.get_k();
    if seed.len() < k {
        context = seed.to_string(); // Return the whole string if it's too short
    } else {
        context = seed[seed.len() - k..].to_string();
    }
    
    println!("context is {}",context);
    println!("model loaded is {}",model.get_k());

    for _ in 0..length {
        let next_char = model.sample_next_char(&context);
        generated_text.push(next_char);
        if generated_text.len() == original_k{
            model = models.get(&original_k).unwrap();
            println!("Changed to model {}",model.get_k());
            context = generated_text[generated_text.len() - model.get_k()..].to_string();
            println!("context length is {}", context.len());
        }
        context.push(next_char);

        if context.len() > model.get_k() {
            context.remove(0); // Keep the context length at k
        }
    }

    generated_text
}

pub fn generate_text_words(models: HashMap<usize, FiniteContextModelWords>, seed: &str, length: usize, original_k: usize) -> String {
    let mut model: &FiniteContextModelWords;
    let mut context: Vec<String> = Vec::new();
    for word in seed.rsplit(' '){
        context.push(String::from(word));
    };
    if context.len() < original_k{
        model = models.get(&context.len()).unwrap();
    }else {
        model = models.get(&original_k).unwrap();
    }
    let mut generated_text = String::from(seed);
    let k = model.get_k();
    while context.len() > k {
        context.remove(0);
    }
    
    println!("context is {}",&context.join(" "));
    println!("model loaded is {}",model.get_k());
    let mut generated_text_length = seed.split(' ').count();

    for _ in 0..length {
        let next_char = model.sample_next_word(&context.join(" "));
        generated_text += " ";
        generated_text += next_char.as_str();
        generated_text_length += 1;
        if generated_text_length == original_k {
            model = models.get(&original_k).unwrap();
            println!("Changed to model {}",model.get_k());
            context = generated_text.split(' ').map(String::from).collect();
        }

        context.push(next_char);

        if context.len() > model.get_k() {
            context.remove(0); // Keep the context length at k
        }
    }

    generated_text
}