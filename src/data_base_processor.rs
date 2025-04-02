
use std::collections::HashMap;

use crate::{file_reader::{self, FileReader}, finite_context_model::FiniteContextModel};

pub struct DataBaseProcessor {
    database: HashMap<String,String>,
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
}