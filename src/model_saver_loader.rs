use std::{fs::File, io::{BufReader, BufWriter}, str};
use serde_json;
use std::io::Write;

use crate::finite_context_model::FiniteContextModel;


pub fn save_model(model: &FiniteContextModel,file_output: &str){
    let json_string = serde_json::to_string_pretty(model).expect("Failed to serialize model");

    let file = File::create(file_output).expect("Failed to create File");
    let mut file_writer = BufWriter::new(file);

    writeln!(file_writer, "{}", json_string).expect("Failed to write to file");
}

pub fn load_model(file_input: &str) -> FiniteContextModel {
    let file = File::open(file_input).expect("Failed to open file");
    let reader = BufReader::new(file);

    serde_json::from_reader(reader).expect("Failed to deserialize model")
}