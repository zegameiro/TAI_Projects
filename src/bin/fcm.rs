extern crate argparse;

use tai_first_project::{finite_context_model::FiniteContextModel, *};
use argparse::{ArgumentParser, Store};

fn main() {

    // Variables to store the values of the arguments
    let mut file_path: String = "".to_string();
    let mut k_value: usize = 0;
    let mut alpha: f64 = 0.0;

    {
        let mut argument_parser: ArgumentParser<'_> = ArgumentParser::new();
        argument_parser.set_description("Algorithmic Theory of Information First Project");

        // Argument file path
        argument_parser.refer(&mut file_path)
            .add_argument("File Path", Store, "Path to the file to be analysed")
            .required();

        // Argument k (size of the sliding window)
        argument_parser.refer(&mut k_value)
            .add_option(&["--k"], Store, "Size of the sliding window");

        // Argument a (smoothing parameter)
        argument_parser.refer(&mut alpha)
            .add_option(&["--a"], Store, "Smoothing parameter");

        argument_parser.parse_args_or_exit();
    }

    let mut file_reader_struct = file_reader::FileReader{filename:String::from(file_path),reader:Option::None};

    if !file_reader::open_file(&mut file_reader_struct).is_ok(){
        println!("error ReadingFile");
        return;
    }

    let mut buff: [u8; 1] = [0u8;1];
    let mut model = FiniteContextModel::new(k_value, alpha);
    let mut text = String::new();

    loop {
        match file_reader::read_byte(&mut file_reader_struct,&mut buff) {
            Ok(true) => {
                text.push(buff[0] as char);
                model.train_char(buff[0] as char);
            }
            Ok(false) => break, // EOF reached
            Err(e) => {
                eprintln!("Error reading file: {}", e); // Print error and exit loop
                break;
            }
        }
    }

    let avg_info = model.calculate_information_content(&text);

    println!("Average information content: {}", avg_info);

}