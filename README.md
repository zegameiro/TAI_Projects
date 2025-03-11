# First Project for the class of TAI 2024/2025

## Instructions

### Dependencies

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
