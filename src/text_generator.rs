use crate::finite_context_model::FiniteContextModel;

extern crate rand;

pub fn generate_text(model: &FiniteContextModel, seed: &str, length: usize) -> String {
    let mut generated_text = String::from(seed);
    let mut context = String::from(seed);

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