use std::fs;

use tai_projects::{audio_reader, ncd};

fn flatten_freqs(freqs: Vec<Vec<f32>>) -> String {
    freqs.iter()
        .flat_map(|seg| seg.iter().map(|f| format!("{:.1}", f)))
        .collect::<Vec<String>>()
        .join(" ")  
}

fn main() {
    let sample_path = "./music/sample04.wav";
    let musics_dir = "./music/music_db/";
    let segment_ms = 500; // 0.5 seconds
    let top_n = 10;
    let top_k = 3;
    let compressor = "gz";

    let samples_freqs = audio_reader::extract_dominant_frequencies(sample_path, segment_ms, top_n);
    let query_std = flatten_freqs(samples_freqs);

    let mut scores: Vec<(String, f64)> = vec![];

    for entry in fs::read_dir(musics_dir).expect("ERROR: Unable to read musics directory") {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) == Some("wav") {
            let fname = path.file_name().unwrap().to_string_lossy().to_string();
            println!("Processing file: {}", fname);

            let freqs = audio_reader::extract_dominant_frequencies(path.to_str().unwrap(), segment_ms, top_n);
            let music_str = flatten_freqs(freqs);

            let ncd_score = ncd::compute_ncd(&query_std, &music_str, compressor);
            println!("    NCD score for: {}", ncd_score);

            scores.push((fname, ncd_score));
        }
    }

    // Sort scores by NCD score ascending
    scores.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

    println!("\nTop {} closest music files:", top_k);
    for (i, (name, score)) in scores.iter().take(top_k).enumerate() {
        println!("{:>2}. {:<30} NCD: {:.4}", i + 1, name, score);
    }
}