
use std::{collections::HashMap, fs::File, io::Write};
use serde::Serialize;

use crate::{file_reader::{self, FileReader}, finite_context_model::FiniteContextModel};

pub struct DataBaseProcessor {
    database: HashMap<String,String>,
}

#[derive(Serialize)]
pub struct ComparisionResult {
    base_sequence: String,
    matches: Vec<MatchScore>
}

#[derive(Serialize)]
pub struct MatchScore {
    target_name: String,
    nrc_score: f64,
}

impl DataBaseProcessor{
    pub fn new(filename:String) -> Self{
        let mut file_reader_struct = FileReader {
            filename: String::from(filename),
            reader: None,
            buffer: Vec::new(),
        };
        let database = Self::read_samples(&mut file_reader_struct).unwrap_or_default();
        Self { database }
    }

    pub fn get_database(&self) -> &HashMap<String,String> {
        &self.database
    }    

    fn read_samples(mut file_reader_struct: &mut FileReader) -> Option<HashMap<String,String>> {
        let _ = file_reader::open_file(&mut file_reader_struct);
        let mut database= HashMap::new();
        let mut sample: String = String::new();
        let mut sample_name = None;

        while let Ok(Some(line)) = file_reader::read_line(file_reader_struct) {
            if line.starts_with('@') {
                // If a previous sample exists, store it
                if let Some(name) = sample_name.take() {
                    database.insert(name, sample.clone());
                }
                sample_name = Some(line);
                sample.clear();
            } else {
                sample.push_str(&line);
            }
        }

        // Insert the last sample if it exists
        if let Some(name) = sample_name {
            database.insert(name, sample);
        }

        Some(database)
    }

    pub fn compute_nrc(&self, model: &FiniteContextModel) -> HashMap<String,f64> {

        let mut nrc_scores: HashMap<String, f64> = HashMap::new();

        for (name, sequence) in &self.database {
            let compress_size = model.calculate_information_content( sequence);
            let sequence_length = sequence.len() as f64;
            let nrc_score = if sequence_length > 0.0 {
                compress_size / (2.0 * sequence_length)
            } else {
                0.0
            };

            nrc_scores.insert(name.clone(), nrc_score);
        }

        nrc_scores
    }

    pub fn get_sequence_by_name(&self, name: &str) -> Option<&String> {
        self.database.get(name)
    }

    pub fn comparative_nrc_analysis(
        &self, 
        low_score_names: &[String], 
        k: usize, 
        alpha: f64
    ) -> Vec<ComparisionResult> {
        let mut comparison_results: Vec<ComparisionResult> = Vec::new();

        for low_name in low_score_names {
            if let Some(low_sequence) = self.database.get(low_name) {
                // Train a model on the low NRC sequence
                let mut model = FiniteContextModel::new(k, alpha);
                for char in low_sequence.chars() {
                    model.train_char(char);
                }

                // Calculate NRC for all other sequences
                let mut comparisons: Vec<_> = Vec::new();
                for target_name in low_score_names {
                    if let Some(target_sequence) = self.database.get(target_name) {

                        let compressed_size = model.calculate_information_content(target_sequence);
                        let nrc_score = if !target_sequence.is_empty() {
                            compressed_size / (2.0 * target_sequence.len() as f64)
                        } else {
                            0.0
                        };

                        comparisons.push(MatchScore {
                            target_name: target_name.clone(),
                            nrc_score,
                        });
                    }
                }

                // Sort comparisons by NRC score
                comparisons.sort_by(|a, b| a.nrc_score.partial_cmp(&b.nrc_score).unwrap());

                comparison_results.push(ComparisionResult {
                    base_sequence: low_name.clone(),
                    matches: comparisons,
                });
            }
        }

        comparison_results
    }

    pub fn export_nrc_comparisons_to_json(
        &self,
        results: &[ComparisionResult],
        output_file: &str,
    ) -> std::io::Result<()> {
        let json_data = serde_json::to_string_pretty(results)?;
        let mut file = File::create(output_file)?;
        file.write_all(json_data.as_bytes())?;
        Ok(())
    }
}