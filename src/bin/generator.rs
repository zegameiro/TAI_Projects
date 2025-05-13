extern crate argparse;
extern crate rand;
use std::collections::HashMap;
use tai_projects::{file_reader::FileReader, finite_context_model::FiniteContextModel,finite_context_model_words::FiniteContextModelWords, text_generator, *};
use argparse::{ArgumentParser, Store};

fn main() {
    let mut file_path: String = "".to_string();
    let mut k_value: usize = 3;
    let mut alpha: f64 = 0.01;
    let mut prior: String = String::new();
    let mut sequence_length: usize = 500;
    let mut mode = "normal".to_string();

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

        // Size of the generated sequence
        argument_parser.refer(&mut mode)
            .add_option(&["-m"], Store, "Mode of context \"normal\":chars, \"words\":words");

        argument_parser.parse_args_or_exit();
    }

    if !["normal","words"].contains(&mode.as_str()){
        println!("invalid mode");
        return;
    }
    
    // Vector of models
    let mut models_n: HashMap<usize, FiniteContextModel> = HashMap::new();
    let mut models_words: HashMap<usize, FiniteContextModelWords> = HashMap::new();

    for k in {
        if mode == "normal"{
            if k_value > prior.len(){
                vec![k_value,prior.len()]
            }else {
                vec![k_value]
            }
        } else {
            let mut context: Vec<String> = Vec::new();
            for word in prior.rsplit(' '){
                context.push(String::from(word));
            };
            if k_value > context.len(){
                vec![k_value,context.len()]
            }else {
                vec![k_value]
            }
        }
    }{
        if mode == "normal"{
            let mut model = FiniteContextModel::new(k, alpha);
            let mut file_reader_struct = open_new_file(file_path.clone());
        
            println!("Training model with k {}",k);
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
            models_n.insert(k, model);
        } else {
            let mut model = FiniteContextModelWords::new(k, alpha);
            let mut file_reader_struct = open_new_file(file_path.clone());
        
            println!("Training model with k {}",k);
            loop {
                match file_reader::read_word(&mut file_reader_struct) {
                    Ok(Some(char)) => {
                        model.train_word(&char);
                    }
                    Ok(None) => break,
                    Err(e) => {
                        eprintln!("Error reading file: {}", e);
                        break;
                    }
                }
            }
            models_words.insert(k, model);
        }
    }
    // if prior.len() > k_value {
    //     eprint!("Prior sequence length must be less than or equal to k");
    //     return;
    // }

    println!("Model created successfully\nGenerating text...");
    let generated_text = if mode == "normal" {
        text_generator::generate_text(models_n, &prior, sequence_length,k_value)
    }else{
        text_generator::generate_text_words(models_words, &prior, sequence_length,k_value)
    };
    println!("Generated Text:\n{}", generated_text);

}



fn open_new_file(file_path: String) -> FileReader{
    let mut file_reader_struct = FileReader{
        filename:String::from(file_path),
        reader:Option::None,
        buffer:Vec::new(),
    };

    if !file_reader::open_file(&mut file_reader_struct).is_ok(){
        println!("error ReadingFile");
    }

    file_reader_struct
}