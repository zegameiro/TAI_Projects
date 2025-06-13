# Projects for the class of TAI 2024/2025

## Table of Contents
- [First Project](#first-project)
  - [Dependencies](#dependencies)
  - [Compile and Executing](#compile-and-executing)
    - [Compile the project](#compile-the-project)
    - [Run fcm](#run-fcm)
    - [Run generator](#run-generator)
    - [Run chart generator](#run-chart-generator)
    - [Examples](#examples)
  - [Important notes](#important-notes)
- [Second Project](#second-project)
  - [Dependencies](#dependencies-1)
  - [Executables](#executables)
  - [Compiling and executing](#compiling-and-executing)
  - [Important Notes](#important-notes-1)
- [Authors](#authors)
- [Project Structure](#project-structure)

---

## First Project

This project consists on the development of two main components:

- **fcm**: a program that measures the information content of text provided using a learned finite-context model
- **generator**: a text generator that creates text following depending on a model created
Both programs read a text file and train a finite-context model before execution of their main
roles.

### Dependencies

- argparse, version 0.2.2
- plotters, version 0.3.7
- rand , version 0.9.0

##### Rust
Rust and Cargo need to be installed.
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Compile and Executing

#### Compile the project
To compile the project, you need to run the following command at the root of the project:
```bash
cargo build
```
After this, executables will be available in the `target/debug` folder.

#### Run fcm
At the root of the project, run:
```bash
target/debug/fcm {file} -k {k} -a {a}
```

With the following arguments:
- `file`: the path to the file .txt with the data to train the model
- `k`: the context size: the number of characters to consider before the current character
- `a`: the smoothing parameter: the value to add to the counts to avoid zero probabilities

#### Run generator
At the root of the project, run:
```bash
target/debug/generator {file} -k {k} -a {a} -p {p} -s {s} -m {mode}
```

With the following arguments:
- `file`: the path to the file .txt with the data to train the model
- `k`: the context size: the number of characters to consider before the current character
- `a`: the smoothing parameter: the value to add to the counts to avoid zero probabilities
- `p`: the first characters of the generated text
- `s`: the number of characters to generate
- `m`: the mode that generator use the default is `normal` that uses chars as tokens the other mode is `words` that use words as tokens

#### Run chart generator
At the root of the project, run:
```bash
target/debug/chart_generator {file} -a {a} -o {output_file}
```

With the following arguments:
- `file`: the path to the file .txt with the data to train the model
- `a`: the smoothing parameter: the value to add to the counts to avoid zero probabilities
- `output_file`: the path to the output file to save the chart it must end with .png

#### Examples
Some bash scripts are available in the `examples` folder to run the programs with some examples.

### Important notes

- The report can be found in this [location](/docs/assignment_1/report_107162_108840_109018.pdf).
- All the visualizations created for this project are in the [visualizations folder](/visualizations/).

---

## Second Project

In this project, we explore the potential to identify the types of organisms present in a metagenomic sample by comparing their similarity to multiple known reference sequences. To achieve this, we use Normalized Relative Compression (NRC) with a finite-context model. The implementation follows a methodology that compares one sample against multiple references in a database file. The tool begins by training a finite-context model using the sample, and then computes the NRC value for each sequence of DNA in the database. Based on these values, we rank the top candidate matches the sample.

### Dependencies

- Rust, Cargo and [OpenCV](https://opencv.org/get-started/) need to be installed.

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

- argparse, version 0.2.2
- plotters, version 0.3.7
- rand, version 0.9.0
- serde, version 1.0 and an additional feature called derive
- serde_json, version 1.0.139
- bson, version 2.0
- opencv, version 0.94.4
- regex, version 1.11.1
- Python 3.8 or higher
- Python libraries: `numpy`, `pandas`, `matplotlib`, `seaborn`, `plotly`, `plotly.express` (at requirements.txt)
- Rust libraries: `serde`, `serde_json`, `bson`, `opencv`, `hound`, `regex` (at Cargo.toml)

### Executables

There is one Bash script in this [location](/scripts/bash/run_meta.sh). To execute it follow these commands:

```bash
cd scripts/bash/
chmod +x run_meta.sh
./run_meta.sh
```

The script builds and runs the metaClass program using the following example arguments:

- db.txt
- meta.txt
- k = 10
- alpha = 0.01

### Compiling and executing

To compile the project, you need to run the following command at the root of the project:
```bash
cargo build
```
After this, executables will be available in the `target/debug` folder.
To run the metaClass program, you need to run the following command at the root of the project:
```bash
target/debug/metaClass -d {db_file} -s {meta_file} -k {k} -a {alpha}
```

The following arguments can/need to be passed:
```bash
Usage:
  ./target/debug/metaClass [OPTIONS]

Algorithmic Theory of Information Second Project

Optional arguments:
  -h,--help             Show this help message and exit
  -s   Path to the meta file (required)
  -d   Path to the database file (required)
  -k   Size of the sliding window (default: 3, must be 1 <= k <= 100)
  -a   Smoothing parameter (default: 0.01, must be 0 <= alpha <= 1)
  -t   Number of top sequences to display (default: 20, must be 1 <= top_sequences <= 239)
  -l   Threshold for low scores (default: 0.5, must be 0 <= low_score <= 1)
```

### Important Notes

- The report can be found in this [location](/docs/assignment_2/TAI_Report_2.pdf).
- The demo for the project can be found in this [location](/docs/assignment_2/tai_demo_2nd_assignment.mp4).
- All the visualizations created for this project are in the [visualizations folder](/visualizations/).

## Third Project

This project explores the use of Normalized Compression Distance (NCD) for automatic music identification. The main goal is to identify music samples by comparing them to a database of full music tracks using general-purpose compression algorithms such as **gzip**, **bzip2**, **zstd** and **lzma**. The method relies on approximating algorithmic information distance through compression, with the objective of finding the music in the database most similar to a given sample. Additional testing includes the use of noisy samples to evaluate the robustness of our approach.

### Implementation

The implementation is organized into several core components:

#### 1. Audio Feature Extraction

The `audio_reader.rs` module extracts the most signficant frequencies from `.wav` files. Each audio is divided into overlapping segments (like with 1 second windows), and for each segment, a Fast Fourier Transform (FFT) is applied to extract the top-N dominant and least dominant frequencies. This results in a time-series "signature" of frequency values for each audio file.

#### 2. Frequency Flattening

These frequency vectors are then flattened into space-separated strings to prepare them for compression based comparison. This string representation is used to simulate a text-based representation of the audio data that compression algorithms can process.

#### 3. Compression and NCD Calculation

The `compressors.rs` module provides wrappers around standard compressors like **gzip**, **bzip2**, **zstd**, and **lzma** to calculate the compressed size of a frequency string. Using these sizes, `ncd.rs` computes the NCD using the formula:
```mathematica
NCD(x, y) = (C(xy) - min(C(x), C(y))) / max(C(x), C(y))
```

This is done for both dominant and least dominant frequency strings allowing comparison on multiple aspects of the audio.
An alternative method is also supported using a Finite Context Model (FCM), where the information content of each string is computed based on a trained probabilistic model, instead of using external compressors.

#### 4. Music Matching

In `audio.rs`, the system reads a sample `.wav` file and compares it against all `.wav` files in a specified music database. It computes the NCD between the sample and each music track and ranks them by similarity (lowest NCD first). The top-K closest matches are displayed for both dominant and least dominant frequencies.

### Experimental Analysis

#### Data used

In terms of data, our experiments utilized a collection of `.wav` files from various music genres, including pop, punk-rock, hard-rock, grunge, rap, fado, pimba, classical, and jazz. In total, 30 music tracks were used and are available in the [music folder](./music/). 
Brown, Pink, Green and White noise were also used to test the robustness of the approach against noisy samples.

#### Experiment with noisy samples

Original music tracks were used to manually create 20-second audio samples, in Audacity. The musics were selected to cover a diverse range of musical styles, ensuring that at least one sample represented each genre. For each selected song, the following variants were created:
- Clean sample (no added noise)
- White noise
- Pink noise
- Brown noise
- Green noise

For each of the samples created, the NCD was calculated between the sample and a reference dataset containing all the original songs, with the goal to determine if the system could correctly identify the most similar track. This procedure was repeated for the compression algorithms *gzip*, *bzip2*, *xz*, *zstd* and *fcm*.

![noisy_chart](tests/noise/noisy_chart.png)

The bar chart shows the percentage of cases in which the most similar sample identified by the NCD-based method corresponded to the original source, across different types of added noise and grouped by compression algorithm.

##### Experiment the effect of noisy on NCD values

A separate test was conducted to investigate whether the loudness of the original music influenced the effect of added noise on the NCD score. The hypothesis was that quieter songs might be more affected by added noise, as the noise becomes more perceptible relative to the audio signal. For this, four songs were selected

**Louder tracks**

- "hitchin_a_ride_green_day" (red)
- "toxicity_system_of_a_down" (orange)

**Quieter tracks**
- "the_still_sea_the_sweetgreens" (green)
- "the_four_seasons_baroque_festival_orchestra" (blue)

The figure below shows the NCD scores for each song across the noise types.

![noise_effect](tests/noise/ncd_noise_effect.png)

The results do not show a clear distinction between loud and quiet tracks (for example, the blue and green lines - quieter songs - do not consistently have higher NCD scores than the red and orange lines - louder songs. So, no conclusive evidence was found that louder or quieter tracks are more or less affected by noise in terms of NCD similarity.

#### Experiment with different compressors and different sizes of samples

For this experiment our group decided to execute our solution, where the samples were created based on all of the available songs, but with different sizes. This means that for each song 3 samples were created, one with 20s, another with 40s and the last one with 60s. Then with for each sample created we used all the compressors available, meaning that for each sample created, the program was executed 5 times one for each compressor (**gzip**, **bzip2**, **zstd**, **xz** and **lzma**). In total our program executed 450 times, 15 times per sample.
The results obtained can be seen throuhg the following image:

![Compressor Accuracy](./visualizations/compressor_accuracy.png)

Where it is possible to observe that the compressors **gzip**, **bzip2**, **zstd** and **xz** manage to always find the most similar song being the song from where the sample was generated. However for the compressor **lzma**, it did not have a good accuracy rate, he can see that he manage to find the correct most similiar song on **50%** of the samples, where in the other half it failed.

#### Experiment with images

#### Experiment with FCM

### Conclusions (só se for necessário)

### Compilation and Execution

## Authors

| Author | Percentage |
| :--: | :--: |
| Guilherme Amorim | 1/3 |
| José Gameiro | 1/3 |
| Tomás Victal | 1/3 |

## Project Structure

```bash
.
├── Cargo.toml
├── data
│   ├── db.txt
│   ├── generated
│   │   ├── db_test.txt
│   │   ├── meta_varying_mutation_0percent.txt
│   │   ├── meta_varying_mutation_10percent.txt
│   │   ├── meta_varying_mutation_15percent.txt
│   │   ├── meta_varying_mutation_1percent.txt
│   │   ├── meta_varying_mutation_20percent.txt
│   │   ├── meta_varying_mutation_25percent.txt
│   │   └── meta_varying_mutation_5percent.txt
│   ├── meta.txt
│   └── sequences
│       ├── sequence1.txt
│       ├── sequence2.txt
│       ├── sequence3.txt
│       ├── sequence4.txt
│       ├── sequence5.txt
│       ├── sequence6.txt
│       ├── sequence7.txt
│       └── test_sequence.txt
├── docs
│   ├── assignment_1
│   │   ├── report_107162_108840_109018.pdf
│   │   └── TAI_WORK_1_2024_2025_RC.pdf
│   └── assignment_2
│       ├── tai_demo_2nd_assignment.mp4
│       ├── TAI_Report_2.pdf
│       └── TAI_WORK_2_2024_2025_RC.pdf
├── LICENSE
├── README.md
├── scripts
│   ├── bash
│   │   ├── comparative_nrc_results.json
│   │   ├── fcm
│   │   │   ├── fcm1.sh
│   │   │   ├── fcm2.sh
│   │   │   ├── fcm3.sh
│   │   │   ├── fcm4.sh
│   │   │   └── fcm5.sh
│   │   ├── generate_meta.sh
│   │   ├── generator
│   │   │   ├── generator1.sh
│   │   │   ├── generator2_mode_words.sh
│   │   │   ├── generator2.sh
│   │   │   ├── generator3.sh
│   │   │   ├── generator4.sh
│   │   │   └── generator5.sh
│   │   ├── gto
│   │   │   └── generate_meta.sh
│   │   ├── image_results
│   │   │   ├── samples_images_quant.sh
│   │   │   └── samples_images.sh
│   │   └── run_meta.sh
│   └── python
│       ├── compare_execution_time.py
│       ├── compare_image_quantization.py
│       ├── compare_mutations.py
│       ├── compare_nrc_scores.py
│       ├── generate_heatmap.py
│       └── requirements.txt
├── src
│   ├── bin
│   │   ├── charts.rs
│   │   ├── fcm.rs
│   │   ├── generator.rs
│   │   ├── image.rs
│   │   └── metaClass.rs
│   ├── chart_generator.rs
│   ├── data_base_processor.rs
│   ├── file_reader.rs
│   ├── finite_context_model_image.rs
│   ├── finite_context_model.rs
│   ├── finite_context_model_words.rs
│   ├── image_processor.rs
│   ├── lib.rs
│   ├── model_saver_loader.rs
│   └── text_generator.rs
├── tests
│   ├── meta_results.csv
│   ├── meta_results_debug.csv
│   └── meta_results_release.csv
└── visualizations
    ├── avg_nrc_vs_mutation.png
    ├── combined_heatmap.png
    ├── complexity_profiles.png
    ├── heatmap_top20_alpha.png
    ├── heatmap_top20_k.png
    ├── lineplot_nrc_time.png
    ├── lineplot_total_time.png
    ├── lineplot_train_time.png
    ├── nrc_comparison_heatmap.png
    ├── quantization_vs_images.png
    ├── sequence1_chart.png
    ├── sequence2_chart.png
    ├── sequence3_chart.png
    ├── sequence4_chart.png
    ├── sequence5_chart.png
    ├── sequence6_chart.png
    ├── sequence7_chart.png
    └── test_sequence_chart.png

18 directories, 85 files
```
