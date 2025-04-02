
use std::collections::HashMap;

use plotters::data;

use crate::file_reader::{self, FileReader};

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
        Self { database: Self::read_samples(&mut file_reader_struct) }
    }

    fn read_samples(file_reader_struct: &mut FileReader) -> HashMap<String,String> {
        let _ = file_reader::open_file(&mut file_reader_struct);
        let mut database: HashMap<String,String>;
        let mut sample:String;
        let mut sample_name:String;
        loop{
            match file_reader::read_line(&mut file_reader_struct) {
                Ok(None) => {
                    break;
                }
                Ok(Some(line)) => {
                     if line.chars().next().unwrap_or('\0') == '@'{

                     }
                }
                Err(e) => {
                    eprintln!("Error reading file: {}", e);
                    return None;
                }
            }
        }
        database
    }
}