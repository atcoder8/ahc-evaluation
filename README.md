# ahc-evaluation

Evaluates the submission code for AHC (AtCoder Heuristic Contest).

This program automates:
  - Builds the submission code and the tester.
  - Executes the submission code and the tester for each seed, and collects scores and execution times.
    - Communication between program and input/output files.
    - Retrieve scores from the output of the tester.

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

Place the configuration file `evaluation/config.toml` under the working directory. You can also specify the path to the configuration file with the option `--config`.

An example of a configuration file is as follows. Keys cannot be changed, but value must be changed as necessary.

```toml
[thread]
# Number of concurrent executions.
# If not set, default number of threads is set.
thread_num = 8

[path]
# Path of the input seed list file to be used for evaluation.
seed_file = "tools/seeds.txt"

# Path of the directory of input files.
input_dir = "tools/in"

# Path of the directory of output files.
output_dir = "evaluation/out"

# Path of the file that outputs the score and execution time for each seed.
evaluation_record = "evaluation/evaluation-table.csv"

[command]
# Command line arguments to build the submission code.
# Specify an empty array if build execution is not required.
build.submission = ["cargo", "build", "--release"]

# Command line arguments to build the tester.
# Specify an empty array if build execution is not required.
build.tester = []

# Command line arguments to execute the submission code.
execute.submission = ["submission/target/release/submission"]

# Command line arguments to execute the tester.
# The following placeholders can be used (Placeholders must be quoted independently).
# - `{input-path}`: The path of the input file corresponding to the seed.
# - `{output-path}`: The path of the output file corresponding to the seed.
# - `{submission-execute}`: Execution command of the submission code.
execute.tester = ["tools/tester", "{input-path}", "{output-path}"]

# Set this flag to `true` if the submission code is to be executed via the tester rather than independently.
execute.integrated = false
```
