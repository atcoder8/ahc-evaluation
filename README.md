# ahc-evaluation

Evaluates the submission code for AHC (AtCoder Heuristic Contest).

This program automates:
  - Builds the submission code and the local tester.
  - Executes the submission code and the local tester for each seed, and collects scores and execution times.
    - Communication between program and input/output files.
    - Retrieve scores from the output of the local tester.

## Usage

```
Evaluates the submission code for AHC (AtCoder Heuristic Contest).

Usage: ahc-evaluation [OPTIONS]

Options:
  -c, --config <CONFIG>  Path of the configuration file [default: evaluation/config.toml]
  -h, --help             Print help
  -V, --version          Print version
```

## Configuration

Place the configuration file `evaluation/config.toml` under the current directory. You can also specify the path to the configuration file with the option `--config`.

An example of a configuration file is as follows. Keys cannot be changed, but value must be changed as necessary.

```toml
[thread]
# Number of threads used for evaluation.
# If not specified, it is automatically determined by Rayon.
thread_num = 8

[path]
# Path of the seed list file.
seed_file = "tools/seeds.txt"

# Path of the directory of input files.
input_dir = "tools/in"

# Path of the directory of output files.
output_dir = "evaluation/out"

# Path of the file that outputs a list summarizing the score and execution time for each seed.
evaluation_record = "evaluation/summary.csv"

[command]
# Build command for submission code.
# Specify an empty array if build execution is not required.
build.submission = ["cargo", "build", "--release"]

# Build command for local tester.
# Specify an empty array if build execution is not required.
build.tester = []

# Execution command for submission code.
execute.submission = ["submission/target/release/submission"]

# Execution command for local tester.
# The following placeholders can be used (Placeholders must be quoted independently).
# - `{input}`: The path of the input file corresponding to the seed.
# - `{output}`: The path of the output file corresponding to the seed.
# - `{cmd}`: Execution command of the submission code.
execute.tester = ["tools/tester", "{input}", "{output}"]

# Set this flag to `true` if the submission code is to be executed via the local tester rather than independently.
execute.integrated = false
```
