
use std::collections::HashMap;

use crate::{file_reader::{self, FileReader}, finite_context_model::FiniteContextModel};

pub struct DataBaseProcessor {
    database: HashMap<String,String>,
}


impl DataBaseProcessor{
    pub fn new(filename:String) -> Self{
        let mut file_reader_struct = FileReader {
            filename: String::from(filename),
            reader: Option::None,
            buffer: Vec::new(),
        };
        Self { database: Self::read_samples(&mut file_reader_struct).unwrap() }
    }

    pub fn get_database(&self) -> &HashMap<String,String> {
        &self.database
    }    

    fn read_samples(mut file_reader_struct: &mut FileReader) -> Option<HashMap<String,String>> {
        let _ = file_reader::open_file(&mut file_reader_struct);
        let mut database: HashMap<String,String> = HashMap::new();
        let mut sample: String = String::new();
        let mut sample_name:String = String::new();
        loop{
            match file_reader::read_line(&mut file_reader_struct) {
                Ok(None) => {
                    break;
                }
                Ok(Some(line)) => {
                    if line.chars().next().unwrap_or('\0') == '@'{

                        if !sample.is_empty() && !sample_name.is_empty() {
                            database.insert(sample_name.clone(), sample.clone());
                            sample.clear();
                            sample_name.clear();
                        }

                        sample_name = line.to_string();
            
                    } else {
                        sample.push_str(line.as_str());
                    }
                }
                Err(e) => {
                    eprintln!("Error reading file: {}", e);
                    return None;
                }
            }
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