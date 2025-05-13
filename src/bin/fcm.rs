extern crate argparse;
extern crate serde;
extern crate serde_json;

use tai_projects::{finite_context_model::FiniteContextModel, *};
use argparse::{ArgumentParser, Store};
use model_saver_loader::save_model;

fn main() {

    let mut file_path: String = "".to_string();
    let mut k_value: usize = 3;
    let mut alpha: f64 = 0.01;

    {
        let mut argument_parser: ArgumentParser<'_> = ArgumentParser::new();
        argument_parser.set_description("Algorithmic Theory of Information First Project");

        // File path
        argument_parser.refer(&mut file_path)
            .add_argument("File Path", Store, "Path to the file to be analysed")
            .required();

        // Size of the sliding window - k
        argument_parser.refer(&mut k_value)
            .add_option(&["-k"], Store, "Size of the sliding window");

        // Smoothing parameter - alpha
        argument_parser.refer(&mut alpha)
            .add_option(&["-a"], Store, "Smoothing parameter");

        argument_parser.parse_args_or_exit();
    }

    let mut file_reader_struct = file_reader::FileReader{
        filename: String::from(file_path),
        reader: Option::None,
        buffer: Vec::new(),
    };

    if !file_reader::open_file(&mut file_reader_struct).is_ok(){
        println!("error ReadingFile");
        return;
    }

    let mut model = FiniteContextModel::new(k_value, alpha);
    let mut text_length: usize = 0;
    let mut total_info: f64 = 0.0;

    loop {
        match file_reader::read_char(&mut file_reader_struct) {
            Ok(Some(char)) => {
                model.train_char(char);
            }
            Ok(None) => break,
            Err(e) => {
                eprintln!("Error reading file: {}", e);
                break;
            }
        }
    }

    let output_filename = "trained_model";
    save_model(&model, output_filename);

    file_reader_struct.reader = None;
    file_reader::open_file(&mut file_reader_struct).unwrap();
    let mut text_buff = String::new();
    let size = 200;

    while let Ok(bytes_read) = file_reader::read_buff(&mut file_reader_struct, &mut text_buff, size) {
        if bytes_read == 0 {
            break;
        }
        total_info += model.calculate_information_content(&text_buff);
        text_length += text_buff.chars().count();
    }

    println!("Average information content: {}", total_info / text_length as f64);

}