use std::fs;
use argparse::{ArgumentParser, Store};

use tai_projects::{audio_reader, finite_context_model::FiniteContextModel, ncd};

fn flatten_freqs(freqs: Vec<Vec<f32>>) -> String {
    freqs.iter()
        .flat_map(|seg| seg.iter().map(|f| format!("{:.1}", f)))
        .collect::<Vec<String>>()
        .join(" ")  
}

fn main() {
    let mut sample_path = "".to_string();
    let mut musics_dir = "".to_string();
    let mut segment_ms = 1000; // 1 seconds
    let mut top_n = 10;
    let mut top_k = 4;
    let mut compressor = "gz".to_string();
    let mut start_ms = 0;
    let mut end_ms = 0;

    {
        let mut argument_parser: ArgumentParser<'_> = ArgumentParser::new();
        argument_parser.set_description("Algorithmic Theory of Information Third Project");

        // Sample music path
        argument_parser.refer(&mut sample_path)
            .add_option(&["-s"], Store, "Path to the sample music file (it must be a .wav file)")
            .required();

        // Directory which contains a database of music files
        argument_parser.refer(&mut musics_dir)
            .add_option(&["-d"], Store, "Path to the directory containing music files (all the musics must be .wav files)")
            .required();

        // Segment length in milliseconds
        argument_parser.refer(&mut segment_ms)
            .add_option(&["-l"], Store, "Segment length in milliseconds (default: 500ms)");    

        // Top N frequencies
        argument_parser.refer(&mut top_n)
            .add_option(&["-n"], Store, "Top N frequencies to extract (default: 10)");

        // Top K closest music files
        argument_parser.refer(&mut top_k)
            .add_option(&["-k"], Store, "Top K closest music files to the sample (default: 4)");

        // Compressor
        argument_parser.refer(&mut compressor)
            .add_option(&["-c"], Store, "Compressor to use (gz, bz2, xz, zstd, fcm) (default: gz)");

        // Start time of the sample in milliseconds
        argument_parser.refer(&mut start_ms)
            .add_option(&["--start"], Store, "Start time (in milliseconds) of the sample segment");

        // End time of the sample in milliseconds
        argument_parser.refer(&mut end_ms)
            .add_option(&["--end"], Store, "End time (in milliseconds) of the sample segment");

        argument_parser.parse_args_or_exit();
    }

    // Check if the sample path is a valid .wav file and it exists
    if !sample_path.ends_with(".wav") && !fs::metadata(&sample_path).is_ok() {
        println!("ERROR: Sample path must be a .wav file");
        return;
    }

    // Check if the music directory exists and is a directory
    if !fs::metadata(&musics_dir).is_ok() {
        println!("ERROR: Music directory does not exist or is not a directory");
        return;
    }

    // Check if the segment length is a valid positive integer
    if segment_ms <= 0 {
        println!("ERROR: Segment length must be a positive integer");
        return;
    }

    // Check if the top N frequencies is a valid positive integer
    if top_n <= 0 {
        println!("ERROR: Top N frequencies must be a positive integer");
        return;
    }

    // Check if the top K closest music files is a valid positive integer
    if top_k <= 0 {
        println!("ERROR: Top K closest music files must be a positive integer");
        return;
    }

    // Check if the compressor is valid
    if !["gz", "bz2", "xz", "zstd", "fcm"].contains(&compressor.as_str()) {
        println!("ERROR: Compressor must be one of gz, bz2, xz, zstd, fcm");
        return;
    }

    // Check if the start time is greater than the end time
    if  end_ms != 0 && start_ms != 0 && end_ms <= start_ms {
        println!("ERROR: End time must be greater than start time");
        return;
    }

    let samples_freqs = audio_reader::extract_dominant_frequencies(
        sample_path.as_str(), 
        segment_ms, 
        top_n,
        Some(start_ms),
        Some(end_ms)
    );
    let query_std = flatten_freqs(samples_freqs.get("dominant").unwrap().clone());

    let mut model: Option<FiniteContextModel> = None;
    if &compressor == "fcm" {
        let k: usize = 8;
        let alpha = 0.5;
        let mut fcm = FiniteContextModel::new(k, alpha);
        for char in query_std.chars() {
            fcm.train_char(char);
        }
        model = Some(fcm);
    }

    let mut m_scores: Vec<(String, f64)> = vec![];
    let mut lm_scores: Vec<(String, f64)> = vec![];
    let mut freqs;

    for entry in fs::read_dir(musics_dir).expect("ERROR: Unable to read musics directory") {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) == Some("wav") {
            let fname = path.file_name().unwrap().to_string_lossy().to_string();
            println!("Processing file: {}", fname);

            freqs = audio_reader::extract_dominant_frequencies(path.to_str().unwrap(), segment_ms, top_n, None, None);
            let music_dom_str = flatten_freqs(freqs.get("dominant").unwrap().clone());
            let musice_least_str = flatten_freqs(freqs.get("least_dominant").unwrap().clone());
            let ncd_score;
            let ncd_score_least;

            if &compressor == "fcm" {
                ncd_score = ncd::compute_ncd_fcm(&query_std, &music_dom_str, model.as_ref().unwrap());
                ncd_score_least = ncd::compute_ncd_fcm(&query_std, &musice_least_str, model.as_ref().unwrap());
            } else {
                ncd_score = ncd::compute_ncd(&query_std, &music_dom_str, compressor.as_str());
                ncd_score_least = ncd::compute_ncd(&query_std, &musice_least_str, compressor.as_str());
            }
            println!("    NCD score (max_freqs): {}", ncd_score);

            println!("    NCD score (least_freqs): {}\n", ncd_score_least);

            m_scores.push((fname.clone(), ncd_score));
            lm_scores.push((fname, ncd_score_least));
        }
    }
    

    // Sort scores by NCD score ascending
    m_scores.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

    println!("\nTop {} closest music files (dominant frequencies):", top_k);
    for (i, (name, score)) in m_scores.iter().take(top_k).enumerate() {
        println!("{:>2}. {:<30} NCD: {:.4}", i + 1, name, score);
    }

    // Sort least dominant frequencies scores by NCD score ascending
    lm_scores.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
    println!("\nTop {} closest music files (least dominant frequencies):", top_k);
    for (i, (name, score)) in lm_scores.iter().take(top_k).enumerate() {
        println!("{:>2}. {:<70} NCD: {:.4}", i + 1, name, score);
    }
}