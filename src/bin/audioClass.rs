use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::time::Instant;
use hound;
use tai_first_project::finite_context_model_audio::AudioFiniteContextModel; // For WAV file handling
use argparse::{ArgumentParser, Store};

/// Reads a WAV file and converts it to normalized floating-point samples.
/// Arguments:
/// * `file_path`: The path to the WAV file
fn read_audio_samples(file_path: &Path) -> io::Result<Vec<f32>> {
    let reader = hound::WavReader::open(file_path).map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
    let spec = reader.spec();
    
    // Convert samples to normalized f32 values
    let samples: Vec<f32> = match spec.sample_format {
        hound::SampleFormat::Int => {
            if spec.bits_per_sample == 16 {
                reader.into_samples::<i16>()
                      .map(|s| s.unwrap_or(0) as f32 / 32768.0)
                      .collect()
            } else if spec.bits_per_sample == 24 {
                reader.into_samples::<i32>()
                      .map(|s| s.unwrap_or(0) as f32 / 8388608.0)
                      .collect()
            } else if spec.bits_per_sample == 32 {
                reader.into_samples::<i32>()
                      .map(|s| s.unwrap_or(0) as f32 / 2147483648.0)
                      .collect()
            } else {
                return Err(io::Error::new(io::ErrorKind::InvalidData, 
                                         "Unsupported bits per sample"));
            }
        },
        hound::SampleFormat::Float => {
            reader.into_samples::<f32>()
                  .map(|s| s.unwrap_or(0.0))
                  .collect()
        }
    };
    
    Ok(samples)
}

/// Recursively finds all WAV files in a directory and its subdirectories.
/// Arguments:
/// * `dir_path`: The path to the directory to search
fn find_wav_files(dir_path: &Path) -> io::Result<Vec<PathBuf>> {
    let mut wav_files = Vec::new();
    
    if dir_path.is_dir() {
        for entry in fs::read_dir(dir_path)? {
            let entry = entry?;
            let path = entry.path();
            
            if path.is_file() && path.extension().map_or(false, |ext| ext == "wav") {
                wav_files.push(path);
            } else if path.is_dir() {
                // Recursively search subdirectories
                let mut subdirectory_wavs = find_wav_files(&path)?;
                wav_files.append(&mut subdirectory_wavs);
            }
        }
    }
    
    Ok(wav_files)
}

fn main() -> io::Result<()> {
    
    let mut k = 3;
    let mut alpha = 0.01;
    let mut quantization_levels = 256;
    let mut sample_path = "".to_string(); 
    let mut db_dir = "".to_string();
    let mut top_n = 10;

    {
        let mut argument_parser: ArgumentParser = ArgumentParser::new();
        argument_parser.set_description("Algorithmic Theory of Information Second Project - Audio Similarity");

        // Sample audio file path
        argument_parser.refer(&mut sample_path)
            .add_option(&["-s"], Store, "Path to the sample audio file")
            .required();

        // Database directory path
        argument_parser.refer(&mut db_dir)
            .add_option(&["-d"], Store, "Path to a directory that contains multiple WAV files")
            .required();

        // Size of the sliding window - k
        argument_parser.refer(&mut k)
            .add_option(&["-k"], Store, "Size of the sliding window");

        // Smoothing parameter - alpha
        argument_parser.refer(&mut alpha)
            .add_option(&["-a"], Store, "Smoothing parameter");

        // Quantization levels
        argument_parser.refer(&mut quantization_levels)
            .add_option(&["-q"], Store, "Number of quantization levels");

        // Number of top results to display
        argument_parser.refer(&mut top_n)
            .add_option(&["-t"], Store, "Number of top results to display");

        argument_parser.parse_args_or_exit();
    }

    if k < 1 || k > 100 {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "k must be between 1 and 100"));
    }

    if alpha < 0.0 || alpha > 1.0 {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "alpha must be between 0 and 1"));
    }

    if top_n < 1 || top_n > 10 {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "top_sequences must be between 1 and 239"));
    }

    if quantization_levels < 1 || quantization_levels > 256 {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "quantization_levels must be between 1 and 256"));
    }
    if !Path::new(&sample_path).exists() {
        return Err(io::Error::new(io::ErrorKind::NotFound, "Sample audio file not found"));
    }

    if !Path::new(&db_dir).exists() {
        return Err(io::Error::new(io::ErrorKind::NotFound, "Database directory not found"));
    }

    println!("Reading sample audio file...");
    let sample_start = Instant::now();
    let sample_audio = read_audio_samples(Path::new(&sample_path))?;
    println!("Sample read in {:?} with {} samples", sample_start.elapsed(), sample_audio.len());

    println!("Training model on sample audio...");
    let train_start = Instant::now();
    let mut model = AudioFiniteContextModel::new(k, alpha, quantization_levels);
    model.train_audio_batch(&sample_audio);
    println!("Model trained in {:?}", train_start.elapsed());

    println!("Scanning database directory for WAV files...");
    let db_scan_start = Instant::now();
    let wav_files = find_wav_files(Path::new(&db_dir))?;
    println!("Found {} WAV files in {:?}", wav_files.len(), db_scan_start.elapsed());

    println!("Computing NRC for each reference audio...");
    let comparison_start = Instant::now();

    // Store file paths and their NRC values
    let mut results = Vec::new();
    
    for wav_path in &wav_files {
        match read_audio_samples(wav_path) {
            Ok(reference_audio) => {
                let nrc = model.calculate_nrc(&reference_audio);
                let file_name = wav_path.file_name().unwrap().to_string_lossy().into_owned();
                results.push((file_name, nrc));
            },
            Err(e) => {
                eprintln!("Error reading {}: {}", wav_path.display(), e);
            }
        }
    }
    
    println!("NRC computation completed in {:?}", comparison_start.elapsed());
    
    // Sort results by NRC value (lowest first - most similar)
    results.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));
    
    // Take only top N results
    let top_results = results.iter().take(top_n);
    
    println!("\nTop {} most similar audio samples:", top_n);
    println!("{:<50} {:<10}", "File Name", "NRC Value");
    println!("{:-<70}", "");
    
    for (i, (name, nrc)) in top_results.enumerate() {
        println!("{:<3}. {:<59} {:.8}", i+1, name, nrc);
    }

    Ok(())
}