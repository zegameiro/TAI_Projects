use tai_first_project::
    {file_reader, 
    finite_context_model::FiniteContextModel,
    data_base_processor::DataBaseProcessor,
};

fn main(){
    let file_path = "./data/test_meta.txt";
    let database_file_path = "./data/db.txt";

    let k = 3;
    let alpha = 0.1;

    let mut file_reader_struct = file_reader::FileReader{
        filename: String::from(file_path),
        reader: Option::None,
        buffer: Vec::new(),
    };

    if !file_reader::open_file(&mut file_reader_struct).is_ok(){
        println!("error ReadingFile");
        return;
    }

    let mut model = FiniteContextModel::new(k, alpha);

    loop {
        match file_reader::read_char(&mut file_reader_struct) {
            Ok(Some(char)) => {
                if char != '\n' {
                    model.train_char(char);
                }
            }
            Ok(None) => break,
            Err(e) => {
                eprintln!("Error reading file: {}", e);
                break;
            }
        }
    }

    let data_processor = DataBaseProcessor::new(database_file_path.to_string());
    let nrc_scores = data_processor.compute_nrc(&model);
    let mut sorted: Vec<(&String, &f64)> = nrc_scores.iter().collect();
    sorted.sort_by(|a, b| a.1.partial_cmp(b.1).unwrap());

    for (key, value) in sorted {
        println!("{}: {}", key, value);
    }
    
}