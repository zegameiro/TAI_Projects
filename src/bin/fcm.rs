extern crate argparse;

use tai_first_project::{finite_context_model::FiniteContextModel, *};
use argparse::{ArgumentParser, Store};

fn main() {

    let mut file_path: String = "".to_string();
    let mut k_value: usize = 0;
    let mut alpha: f64 = 0.0;

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

    let mut file_reader_struct = file_reader::FileReader{filename:String::from(file_path),reader:Option::None};

    if !file_reader::open_file(&mut file_reader_struct).is_ok(){
        println!("error ReadingFile");
        return;
    }

    let mut buff: [u8; 1] = [0u8;1];
    let mut model = FiniteContextModel::new(k_value, alpha);
    let mut text_length: usize = 0;
    let mut total_info: f64 = 0.0;

    loop {
        match file_reader::read_byte(&mut file_reader_struct,&mut buff) {
            Ok(true) => {
                model.train_char(buff[0] as char);
            }
            Ok(false) => break, // EOF reached
            Err(e) => {
                eprintln!("Error reading file: {}", e);
                break;
            }
        }
    }

    file_reader_struct.reader = None;
    let result = file_reader::open_file(&mut file_reader_struct);
    let mut text_buff = [0u8;200];

    while let Ok(bytes_read) = file_reader::read_buff(&mut file_reader_struct, &mut text_buff) {
        if bytes_read == 0 {
            break;
        }
        total_info += model.calculate_information_content(&String::from_utf8_lossy(&text_buff[..bytes_read]));
        text_length += bytes_read;
    }

    println!("Average information content: {}", total_info / text_length as f64);

}