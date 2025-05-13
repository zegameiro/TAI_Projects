extern crate argparse;

use tai_projects::{chart_generator::ChartGenerator, file_reader};
use argparse::{ArgumentParser, Store};

fn main() {
    let mut file_path: String = "".to_string();
    let mut alpha: f32 = 0.01;
    let mut output_file: String = "chart.png".to_string();

    {
        let mut argument_parser: ArgumentParser<'_> = ArgumentParser::new();
        argument_parser.set_description("Algorithmic Theory of Information First Project");

        // File path
        argument_parser.refer(&mut file_path)
            .add_argument("File Path", Store, "Path to the file to be analysed")
            .required();

        // Smoothing parameter - alpha
        argument_parser.refer(&mut alpha)
            .add_option(&["-a"], Store, "Smoothing parameter");

        // Output file
        argument_parser.refer(&mut output_file)
            .add_option(&["-o"], Store, "Output file");

        argument_parser.parse_args_or_exit();
    }

    let mut file_reader_struct = file_reader::FileReader{
        filename:String::from(file_path),
        reader:Option::None,
        buffer: Vec::new(),
    };

    if !file_reader::open_file(&mut file_reader_struct).is_ok(){
        println!("error Reading File");
        return;
    }

    let mut v: Vec<char> = Vec::new();
    
    while let Ok(Some(symbol)) = file_reader::read_char(&mut file_reader_struct) {
        if !v.contains(&symbol) {
            v.push(symbol);
        }
    }

    println!("{}", v.len());

    let mut generator: ChartGenerator = ChartGenerator::new(alpha, v.len() as f32);
    file_reader_struct.reader = None;
    let mut prev_symbol: Option<char> = None;

    file_reader::open_file(&mut file_reader_struct).unwrap();

    while let Ok(Some(symbol)) = file_reader::read_char(&mut file_reader_struct) {
        if let Some(prev) = prev_symbol {
            generator.train_char(prev, symbol);
        }
        prev_symbol = Some(symbol);
    }

    generator.draw_chart(&output_file);
}