use crate::finite_context_model::FiniteContextModel;

extern crate rand;

pub fn generate_text(model: &FiniteContextModel, seed: &str, length: usize) -> String {
    let mut generated_text = String::from(seed);
    let len = seed.len();
    let mut context: String;
    let k = model.get_k();
    if len < k {
        context = seed.to_string(); // Return the whole string if it's too short
    } else {
        context = seed[len - k..].to_string(); // Slice the last two characters
    }

    for _ in 0..length {
        let next_char = model.sample_next_char(&context);
        generated_text.push(next_char);
        context.push(next_char);

        if context.len() > model.get_k() {
            context.remove(0); // Keep the context length at k
        }
    }

    generated_text
}