use crate::finite_context_model::FiniteContextModel;

fn compute_nrc(model: &FiniteContextModel, sequence: &str) -> f64 {
    let mut nrc_score = 0.0;
    let compressed_size = model.calculate_information_content(sequence);
    let sequence_length = sequence.len() as f64;

    if sequence_length > 0.0 {
        nrc_score = compressed_size / (2.0 * sequence_length);
    }

    nrc_score
}
