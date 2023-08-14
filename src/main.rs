use std::{
    fs::{create_dir_all, read_to_string, File},
    io::Write,
    process::{Command, Stdio},
};

use ahc_evaluation::{arguments::Arguments, config::Config, stop_watch::StopWatch};
use clap::Parser;
use indicatif::{ProgressBar, ProgressStyle};
use itertools::Itertools;
use rayon::{prelude::IntoParallelRefIterator, prelude::*, ThreadPoolBuilder};

fn main() {
    // Parse command line arguments.
    let args = Arguments::parse();

    let config_file_path = std::path::Path::new(&args.config);
    let config = read_to_string(config_file_path).expect("Failed to read configuration file.");
    let config: Config =
        toml::from_str(&config).expect("Failed to deserialize configuration file.");

    // Set the number of threads to be used in the rayon thread pool.
    if let Some(thread_num) = config.thread.thread_num {
        ThreadPoolBuilder::new()
            .num_threads(thread_num)
            .build_global()
            .expect("Failed to set the number of threads.");
    }

    // Read seeds.
    let seeds = read_to_string(&config.path.seed_file).unwrap_or_else(|_| {
        panic!(
            "Failed to read seed file ({}).",
            config.path.seed_file.to_str().unwrap()
        );
    });
    let seeds = seeds
        .split_whitespace()
        .map(|seed| {
            seed.parse::<usize>().unwrap_or_else(|_| {
                panic!("Failed to parse \"{}\" as a seed.", seed);
            })
        })
        .collect_vec();

    // Create output directory.
    create_dir_all(&config.path.output_dir).expect("Failed to create output directory.");

    // Build tester.
    build_tester(&config);

    // Build the program to be submitted.
    build_submission(&config);

    // Style of progress bar.
    let progress_style = ProgressStyle::template(
        ProgressStyle::default_bar(),
        "{prefix}\n{wide_bar} {pos:>3}/{len:3} {percent:>3}% [{elapsed_precise}<{eta_precise}]",
    )
    .unwrap();

    // Progress bar during running tester.
    let progress_bar = ProgressBar::new(seeds.len() as u64);
    progress_bar.set_style(progress_style);
    progress_bar.set_prefix("[tester] Running...");

    // Run tester and retrieve evaluations.
    let evaluations: Vec<Evaluation> = seeds
        .par_iter()
        .map(|&seed| {
            let evaluation = run_tester(&config, seed);

            progress_bar.inc(1);

            evaluation
        })
        .collect();

    // Show score statistics.
    let scores = evaluations
        .iter()
        .map(|evaluation| evaluation.score)
        .collect_vec();
    println!("\n");
    show_score_statistics(&seeds, &scores);

    // Show execution time statistics.
    let exe_times = evaluations
        .iter()
        .map(|evaluation| evaluation.execution_time)
        .collect_vec();
    println!();
    show_execution_time_statistics(&seeds, &exe_times);

    // Output seed, score and execution time to evaluation file with comma delimiter.
    let mut output_evaluation_file =
        File::create(&args.evaluation).expect("Failed to create output file of evaluation.");
    output_evaluation_file
        .write_all(b"seed,score,execution_time\n")
        .expect("Failed to write to output file of evaluation.");
    for (&seed, evaluation) in seeds.iter().zip(&evaluations) {
        output_evaluation_file
            .write_all(
                format!(
                    "{},{},{}\n",
                    seed, evaluation.score, evaluation.execution_time
                )
                .as_bytes(),
            )
            .expect("Failed to write to output file of evaluation.");
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Evaluation {
    pub score: usize,
    pub execution_time: f64,
}

fn build_tester(config: &Config) {
    let process_handle = Command::new(&config.command.build.tester[0])
        .args(&config.command.build.tester[1..])
        .spawn()
        .expect("Failed to execute process for build tester.");

    process_handle
        .wait_with_output()
        .expect("Failed to build tester.");
}

fn build_submission(config: &Config) {
    let process_handle = Command::new(&config.command.build.submission[0])
        .args(&config.command.build.submission[1..])
        .spawn()
        .expect("Failed to execute process for build submission program.");

    process_handle
        .wait_with_output()
        .expect("Failed to build submission program.");
}

fn run_tester(config: &Config, seed: usize) -> Evaluation {
    let input_file_path = config.path.input_dir.join(format!("{:04}.txt", seed));

    let input_text = read_to_string(&input_file_path).unwrap_or_else(|_| {
        panic!(
            "Failed to read text from input file ({}).",
            input_file_path.to_str().unwrap()
        );
    });

    let stop_watch = StopWatch::new();

    let args = config
        .command
        .run
        .tester
        .iter()
        .chain(&config.command.run.submission)
        .collect_vec();

    let process_handle = Command::new(args[0])
        .args(&args[1..])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to execute process for tester execution.");

    process_handle
        .stdin
        .as_ref()
        .unwrap()
        .write_all(input_text.as_bytes())
        .expect("Failed to input text to tester.");

    let output = process_handle
        .wait_with_output()
        .expect("Failed to run the tester.");

    let execution_time = stop_watch.elapsed_time();

    assert!(
        output.status.success(),
        "
Seed {} run terminated with exit status {}.

-------------------------- Standard Output (tester) ---------------------------
{}
-------------------------------------------------------------------------------

----------------------- Standard Error Output (tester) ------------------------
{}
-------------------------------------------------------------------------------
",
        seed,
        output.status,
        String::from_utf8(output.stdout).unwrap(),
        String::from_utf8(output.stderr).unwrap(),
    );

    let output_file_path = config.path.output_dir.join(format!("{:04}.txt", seed));
    let mut output_file = File::create(&output_file_path).expect("Failed to create output file.");
    output_file.write_all(&output.stdout).unwrap_or_else(|_| {
        panic!(
            "Failed to write output from tester to output file ({}).",
            output_file_path.to_str().unwrap(),
        );
    });

    let stderr = String::from_utf8(output.stderr).unwrap();

    let score = stderr
        .split_whitespace()
        .last()
        .and_then(|score| score.parse::<usize>().ok())
        .unwrap_or_else(|| {
            panic!(
                "
Failed to retrieve score from standard error output of the tester.

----------------------- Standard Error Output (tester) ------------------------
{}
-------------------------------------------------------------------------------
",
                stderr
            )
        });

    Evaluation {
        score,
        execution_time,
    }
}

/// Shows score statistics.
fn show_score_statistics(seeds: &Vec<usize>, scores: &Vec<usize>) {
    assert_eq!(seeds.len(), scores.len());

    let exe_num = seeds.len();

    let total_score: usize = scores.iter().sum();
    let avg_score = total_score as f64 / exe_num as f64;

    let min_score = *scores.iter().min().unwrap();
    let min_score_idx = scores.iter().position(|&score| score == min_score).unwrap();

    let max_score = *scores.iter().max().unwrap();
    let max_score_idx = scores.iter().position(|&score| score == max_score).unwrap();

    println!(
        "\
[Score Statistics]
Total: {}
Average: {:.3}
Min: {} (seed = {})
Max: {} (seed = {})",
        total_score, avg_score, min_score, seeds[min_score_idx], max_score, seeds[max_score_idx],
    );
}

/// Shows execution time statistics.
fn show_execution_time_statistics(seeds: &Vec<usize>, exe_times: &Vec<f64>) {
    assert_eq!(seeds.len(), exe_times.len());

    let exe_num = seeds.len();

    let total_exe_time: f64 = exe_times.iter().sum();
    let avg_exe_time = total_exe_time / exe_num as f64;

    let min_exe_time = *exe_times
        .iter()
        .min_by(|x, y| x.partial_cmp(y).unwrap())
        .unwrap();
    let min_exe_time_idx = exe_times
        .iter()
        .position(|&exe_time| exe_time == min_exe_time)
        .unwrap();

    let max_exe_time = *exe_times
        .iter()
        .max_by(|x, y| x.partial_cmp(y).unwrap())
        .unwrap();
    let max_exe_time_idx = exe_times
        .iter()
        .position(|&exe_time| exe_time == max_exe_time)
        .unwrap();

    println!(
        "\
[Execution Time]
Total: {:.3}
Average: {:.3}
Min: {:.3} (seed = {})
Max: {:.3} (seed = {})",
        total_exe_time,
        avg_exe_time,
        min_exe_time,
        seeds[min_exe_time_idx],
        max_exe_time,
        seeds[max_exe_time_idx],
    );
}
