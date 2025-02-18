extern crate argparse;

use argparse::{ArgumentParser, Store};

fn main() {

    // Variables to store the values of the arguments
    let mut file_path: String = "".to_string();
    let mut k_value: i32 = 0;
    let mut a_value: f64 = 0.0;

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
        argument_parser.refer(&mut a_value)
            .add_option(&["--a"], Store, "Smoothing parameter");

        argument_parser.parse_args_or_exit();
    }

}