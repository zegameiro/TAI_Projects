use tai_projects::
    {chart_generator::ChartGenerator, data_base_processor::{ComparisionResult, DataBaseProcessor}, file_reader, finite_context_model::FiniteContextModel
};
extern crate argparse;

use argparse::{ArgumentParser, Store};

fn main(){
    let mut meta_file_path: String = "".to_string();
    let mut database_file_path: String= "".to_string();
    let mut k: usize = 3;
    let mut alpha = 0.01;
    let mut top_sequences = 20; 
    let mut treshold = 0.5;

    {
        let mut argument_parser: ArgumentParser<'_> = ArgumentParser::new();
        argument_parser.set_description("Algorithmic Theory of Information Second Project");

        // Meta file path
        argument_parser.refer(&mut meta_file_path)
            .add_option(&["-s"], Store, "Path to the meta file (required)")
            .required();

        // Database file path
        argument_parser.refer(&mut database_file_path)
            .add_option(&["-d"], Store, "Path to the database file (required)")
            .required();

        // Size of the sliding window - k
        argument_parser.refer(&mut k)
            .add_option(&["-k"], Store, "Size of the sliding window (default: 3 must be 1 <= k <= 100)");

        // Smoothing parameter - alpha
        argument_parser.refer(&mut alpha)
            .add_option(&["-a"], Store, "Smoothing parameter (default: 0.01 must be 0 <= alpha <= 1)");

        // Top sequences to display
        argument_parser.refer(&mut top_sequences)
            .add_option(&["-t"], Store, "Number of top sequences to display (default: 20 must be 1 <= top_sequences <= 239)");

        // Threshold for low scores
        argument_parser.refer(&mut treshold)
            .add_option(&["-l"], Store, "Threshold for low scores (default: 0.5)");

        argument_parser.parse_args_or_exit();
    }
    
    // Check if k has a valid value and if it is a number
    if k < 1 || k > 20   {
        println!("Error: k must be greater than 0");
        return;
    }

    if alpha < 0.0 || alpha > 1.0 {
        println!("Error: alpha must be between 0 and 1");
        return;
    }

    if top_sequences < 1 || top_sequences > 239 {
        println!("Error: top_sequences must be greater than 0 and less than 240");
        return;
    }

    if treshold < 0.0 || treshold > 1.0 {
        println!("Error: treshold must be between 0 and 1");
        return;
    }

    let mut file_reader_struct = file_reader::FileReader{
        filename: String::from(meta_file_path.clone()),
        reader: None,
        buffer: Vec::new(),
    };

    if !file_reader::open_file(&mut file_reader_struct).is_ok(){
        println!("error Reading File");
        return;
    }

    use std::time::Instant;
    let now = Instant::now();

    println!("Reading file metagenomic sample in file {} and training model...", &meta_file_path);

    let mut model = FiniteContextModel::new(k, alpha);
    let mut metagonic_sample = String::new();
    loop {
        match file_reader::read_char(&mut file_reader_struct) {
            Ok(Some(char)) => {
                if char != '\n' {
                    metagonic_sample.push(char);
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

    println!("Model trained with k = {} and alpha = {}", k, alpha);

    let elapsed = now.elapsed();

    println!("Reading file database in file {} and computing NRC scores...", &database_file_path);
    let data_processor = DataBaseProcessor::new(database_file_path.to_string());
    let mut nrc_scores: Vec<_> = data_processor.compute_nrc(&model).into_iter().collect();

    println!("NRC scores computed\nSorting NRC scores...");

    let elapsed_nrc = now.elapsed();

    nrc_scores.sort_unstable_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

    println!("Top {} sequences:", top_sequences);
    for (name, score) in nrc_scores.iter().take(top_sequences) {
        println!("{}: {:.6}", name, score);
    }

    let elapsed_final = now.elapsed();

    // Collect sequences with scores bellow the threshold
    let low_scores: Vec<_> = nrc_scores.iter()
        .filter(|(_, score)| *score < treshold)
        .collect();

    let low_score_names: Vec<String> = low_scores.iter().map(|(name, _)| name.clone()).collect();

    println!("\nObtained sequences with NRC scores lower than {}:", treshold);

    println!("\nComputing Similarity Matrix with {} sequences...", low_score_names.len());
    let results: Vec<ComparisionResult> = data_processor.comparative_nrc_analysis(&low_score_names, k, alpha);
    let output_file = "comparative_nrc_results.json";
    let _ = data_processor.export_nrc_comparisons_to_json(&results, output_file);
    println!("Similarity matrix saved to {}", output_file);

    println!("\nGenerating complexity profiles for metagenomic sample and sequences with NRC scores lower than {} ...", treshold);
    let mut profiles: Vec<(&str, Vec<f64>)> = Vec::new();
    let meta_profile = model.complexity_profile(&metagonic_sample);
    profiles.push(("meta", meta_profile));

    for name in &low_score_names {
        if let Some(profile) = data_processor.get_sequence_by_name(name) {
            let profile = model.complexity_profile(&profile);
            profiles.push((name.as_str(), profile));
        }
    }

    let generator: ChartGenerator = ChartGenerator::new(alpha as f32, 4.0);

    println!("Complexity profiles obtained\nDrawing complexity profiles...");
    if let Err(e) = generator.draw_complexity_profiles(profiles, "visualizations/complexity_profiles.png") {
        eprintln!("Failed to draw complexity profiles: {}", e);
    }
    println!("Complexity profiles saved to visualizations/complexity_profiles.png");

    println!("\nTime taken to train the model: {:?}", elapsed);
    println!("Time taken to compute NRC scores: {:?}", elapsed_nrc - elapsed);
    println!("Total time taken: {:?}", elapsed_final);

}