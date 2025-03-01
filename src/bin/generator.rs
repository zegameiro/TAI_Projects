extern crate argparse;
extern crate rand;

use tai_first_project::{finite_context_model::FiniteContextModel, model_saver_loader::load_model, text_generator, *};
use argparse::{ArgumentParser, Store};

fn main() {
    let mut file_path: String = "".to_string();
    let mut k_value: usize = 3;
    let mut alpha: f64 = 0.01;
    let mut prior: String = String::new();
    let mut sequence_length: usize = 500; 

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

        // Prior sequence
        argument_parser.refer(&mut prior)
            .add_option(&["-p"], Store, "Prior sequence").required();

        // Size of the generated sequence
        argument_parser.refer(&mut sequence_length)
            .add_option(&["-s"], Store, "Length of the generated sequence");

        argument_parser.parse_args_or_exit();
    }

    let mut file_reader_struct = file_reader::FileReader{
        filename:String::from(file_path),
        reader:Option::None,
        buffer:String::new(),
    };

    if !file_reader::open_file(&mut file_reader_struct).is_ok(){
        println!("error ReadingFile");
        return;
    }

    let mut model = FiniteContextModel::new(k_value, alpha);

    println!("Training model...");

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


    // if prior.len() > k_value {
    //     eprint!("Prior sequence length must be less than or equal to k");
    //     return;
    // }

    println!("Model created successfully\nGenerating text...");
    let generated_text = text_generator::generate_text(&model, &prior, sequence_length);
    println!("Generated Text:\n{}", generated_text);

}