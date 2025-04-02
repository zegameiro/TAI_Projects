use std::{fs::File, io::{BufReader, BufWriter}, str};
use serde_json;
use bson::{Bson, to_bson};
use std::path::Path;
use std::io::Write;

use crate::finite_context_model::FiniteContextModel;


pub fn save_model(model: &FiniteContextModel,file_output: &str){

    std::fs::create_dir_all(Path::new("models")).expect("Failed to create models directory");

    // JSON
    let json_file_path = Path::new("models").join(file_output).with_extension("json");
    let json_string = serde_json::to_string_pretty(model).expect("Failed to serialize model");

    let file = File::create(json_file_path).expect("Failed to create File");
    let mut file_writer = BufWriter::new(file);

    writeln!(file_writer, "{}", json_string).expect("Failed to write to file");

    println!("Model salved as {}.json", file_output);

    // Binary / BSON
    let bson_file_path = Path::new("models").join(file_output).with_extension("bson");
    let bson_value = to_bson(model).expect("Failed to serialize model to BSON");
    
    let bson_file_doc = if let Bson::Document(doc) = bson_value {
        doc
    } else {
        panic!("Failed to convert model to BSON Document");
    };

    let file = File::create(bson_file_path).expect("Failed to create BSON file");
    let mut writer = BufWriter::new(file);

    let bson_data = bson::to_vec(&bson_file_doc).expect("Failed to serialize to BSON");
    writer.write_all(&bson_data).expect("Failed to write BSON data to file");
    
    println!("Model salved as {}.bson", file_output);
}

pub fn load_model(file_input: &str) -> FiniteContextModel {
    let file = File::open(file_input).expect("Failed to open file");
    let reader = BufReader::new(file);

    serde_json::from_reader(reader).expect("Failed to deserialize model")
}