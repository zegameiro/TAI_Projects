# First Project for the class of TAI 2024/2025

## Instructions

### Dependencies

- argparse, version 0.2.2
- plotters, version 0.3.7
- rand , version 0.9.0

#### Rust
Rust and Cargo need to be installed.
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Run the programs

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


# Second Project for the class of TAI 2024/2025
At the second project, the goal was to develop a tool that applies NRC to identify the most similar organisms from a known reference database.

## Authors

| Author | Percentage |
| :--: | :--: |
| Guilherme Amorim | 1/3 |
| José Gameiro | 1/3 |
| Tomás Victal | 1/3 |

## Dependencies
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

## Executables

There are two Bash scripts located in the root of the project. Make them executable:

```bash
cd scripts/bash/
chmod +x run_meta.sh
```

### Meta

Run this using the following command:

```bash
cd scripts/bash/
./run_meta.sh
```
The script builds and runs the metaClass program using the following example arguments:

- db.txt
- meta.txt
- k = 10
- alpha = 0.01
