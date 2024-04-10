mod record;
mod stop_watch;

use std::{
    fs::{create_dir_all, read_to_string, File},
    io::Write,
    process::{Command, Stdio},
};

use anyhow::{bail, ensure, Context};
use indicatif::{ParallelProgressIterator, ProgressBar, ProgressStyle};
use rayon::{iter::IntoParallelRefIterator, prelude::ParallelIterator};
use regex::Regex;

use crate::{
    config::Config,
    evaluation::{record::EvaluationRecord, stop_watch::Stopwatch},
};

pub use crate::evaluation::record::{show_statistics, write_to_csv};

/// Executes the submission code and the tester for each seed and collect the score and the execution time.
pub fn evaluate(config: &Config, seeds: &[usize]) -> anyhow::Result<Vec<EvaluationRecord>> {
    // Style of progress bar.
    let progress_style = ProgressStyle::template(
        ProgressStyle::default_bar(),
        "{prefix}\n{wide_bar} {pos:>3}/{len:3} {percent:>3}% [{elapsed_precise}<{eta_precise}]",
    )
    .with_context(|| "Failed to create progress bar style.")?;

    // Progress bar during running tester.
    let progress_bar = ProgressBar::new(seeds.len() as u64);
    progress_bar.set_style(progress_style);
    progress_bar.set_prefix("[tester] Running...");

    // Creates output directory.
    create_dir_all(&config.path.output_dir)
        .with_context(|| "Failed to create output directory.")?;

    // Executes the tester and retrieve evaluations.
    seeds
        .par_iter()
        .progress_with(progress_bar)
        .map(|&seed| {
            if config.command.execute.integrated {
                execute_integrated_process(config, seed)
            } else {
                execute_independent_processes(config, seed)
            }
        })
        .collect::<Result<Vec<EvaluationRecord>, _>>()
}

/// Executes the submission code via the tester.
fn execute_integrated_process(config: &Config, seed: usize) -> anyhow::Result<EvaluationRecord> {
    // Reads the input file.
    let input_file_path = config.input_file_path(seed);
    let input_text = read_to_string(&input_file_path)
        .with_context(|| format!("Failed to read text from `{:?}`.", input_file_path))?;

    // Executes the tester as a child process.
    let cmd_args = config.cmd_args_for_execute_tester(seed);
    let process_handle = spawn_process(&cmd_args)?;

    // Starts measuring execution time.
    let stopwatch = Stopwatch::start();

    // Writes the contents of the input file to the standard input.
    process_handle
        .stdin
        .as_ref()
        .unwrap()
        .write_all(input_text.as_bytes())
        .with_context(|| "Failed to input text.")?;

    // Waits for process to terminate.
    let output = process_handle.wait_with_output()?;

    ensure!(
        output.status.success(),
        ExecuteCommandError {
            seed,
            cmd_args,
            output
        }
    );

    // Terminates measurement of execution time.
    let execution_time = stopwatch.elapsed_time();

    // Writes the contents of the standard output to the output file.
    let output_file_path = config.output_file_path(seed);
    File::create(&output_file_path)
        .with_context(|| format!("Failed to create output file `{:?}`.", output_file_path))?
        .write_all(&output.stdout)
        .with_context(|| format!("Failed to write to output file {:?}.", output_file_path))?;

    let score_regex = Regex::new(r"\bScore *= *(?<score>[0-9]*)\b")
        .with_context(|| "Failed to compile regular expression.")?;

    let stderr = String::from_utf8(output.stderr.clone())?;

    let Some(score) = score_regex
        .captures(&stderr)
        .and_then(|caps| caps["score"].parse::<i64>().ok())
    else {
        bail!(format!(
            "
Failed to retrieve score.

Seed: {}

---------------------------- Standard Error Output -----------------------------
{:?}
--------------------------------------------------------------------------------
",
            seed, output.stderr,
        ));
    };

    Ok(EvaluationRecord {
        seed,
        score,
        execution_time,
    })
}

/// Executes the submission code and the tester separately.
fn execute_independent_processes(config: &Config, seed: usize) -> anyhow::Result<EvaluationRecord> {
    // Reads the input file.
    let input_file_path = config.input_file_path(seed);
    let input_text = read_to_string(&input_file_path)
        .with_context(|| format!("Failed to read text from `{:?}`.", input_file_path))?;

    // Executes the submission code as a child process.
    let cmd_args_for_execute_submission = &config.command.execute.submission;
    let submission_process_handle = spawn_process(cmd_args_for_execute_submission)?;

    // Starts measuring execution time.
    let stopwatch = Stopwatch::start();

    // Writes the contents of the input file to the standard input.
    submission_process_handle
        .stdin
        .as_ref()
        .unwrap()
        .write_all(input_text.as_bytes())
        .with_context(|| "Failed to input text.")?;

    // Waits for process to terminate.
    let submission_process_output =
        submission_process_handle
            .wait_with_output()
            .with_context(|| {
                format!(
                    "
Failed to execute the submission code.
List of arguments: {:?}
",
                    cmd_args_for_execute_submission
                )
            })?;

    // Terminates measurement of execution time.
    let execution_time = stopwatch.elapsed_time();

    ensure!(
        submission_process_output.status.success(),
        ExecuteCommandError {
            seed,
            cmd_args: cmd_args_for_execute_submission.to_owned(),
            output: submission_process_output
        }
    );

    // Writes the contents of the standard output to the output file.
    let output_file_path = config.output_file_path(seed);
    File::create(&output_file_path)
        .with_context(|| format!("Failed to create output file `{:?}`.", output_file_path))?
        .write_all(&submission_process_output.stdout)
        .with_context(|| format!("Failed to write to output file {:?}.", output_file_path))?;

    // Executes the tester as a child process.
    let cmd_args_for_execute_tester = config.cmd_args_for_execute_tester(seed);

    // Waits for process to terminate.
    let tester_process_output = spawn_process(&cmd_args_for_execute_tester)?
        .wait_with_output()
        .with_context(|| {
            format!(
                "
Failed to execute the tester.
List of arguments: {:?}
",
                cmd_args_for_execute_tester
            )
        })?;

    ensure!(
        tester_process_output.status.success(),
        ExecuteCommandError {
            seed,
            cmd_args: cmd_args_for_execute_submission.to_owned(),
            output: tester_process_output
        }
    );

    let score_regex = Regex::new(r"\bScore *= *(?<score>[0-9]*)\b")
        .with_context(|| "Failed to compile regular expression.")?;

    let retrieve_score = |text: &str| {
        score_regex
            .captures(text)
            .and_then(|caps| caps["score"].parse::<i64>().ok())
    };

    let stdout = String::from_utf8(tester_process_output.stdout.clone()).unwrap();
    let stderr = String::from_utf8(tester_process_output.stderr.clone()).unwrap();

    let Some(score) = retrieve_score(&stdout).or(retrieve_score(&stderr)) else {
        bail!(format!(
            "
Failed to retrieve score.

Seed: {}

------------------------------- Standard Output --------------------------------
{:?}
--------------------------------------------------------------------------------

---------------------------- Standard Error Output -----------------------------
{:?}
--------------------------------------------------------------------------------
",
            seed, tester_process_output.stdout, tester_process_output.stderr,
        ));
    };

    Ok(EvaluationRecord {
        seed,
        score,
        execution_time,
    })
}

/// Spawns a child process that executes the specified command.
fn spawn_process(cmd_args: &[String]) -> anyhow::Result<std::process::Child> {
    let program = cmd_args
        .first()
        .with_context(|| "The execution command is empty.")?;

    Command::new(program)
        .args(&cmd_args[1..])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .with_context(|| {
            format!(
                "
Failed to start the child process.
List of arguments: {:?}
",
                cmd_args
            )
        })
}

#[derive(Debug)]
pub struct ExecuteCommandError {
    pub seed: usize,
    pub cmd_args: Vec<String>,
    pub output: std::process::Output,
}

impl std::fmt::Display for ExecuteCommandError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let stdout = String::from_utf8(self.output.stdout.clone()).unwrap();
        let stderr = String::from_utf8(self.output.stderr.clone()).unwrap();

        write!(
            f,
            "
Failed to execute command.

Exit code: {}

Seed: {}

Command line arguments: {:?},

------------------------------- Standard Output --------------------------------
{}
--------------------------------------------------------------------------------

---------------------------- Standard Error Output -----------------------------
{}
--------------------------------------------------------------------------------
",
            self.output.status, self.seed, self.cmd_args, stdout, stderr,
        )
    }
}

impl std::error::Error for ExecuteCommandError {}
