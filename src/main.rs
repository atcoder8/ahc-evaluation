use std::fs::read_to_string;

use ahc_evaluation::{arguments::Arguments, build, config::Config, evaluation};
use anyhow::{ensure, Context};
use clap::Parser;
use rayon::ThreadPoolBuilder;

fn main() -> anyhow::Result<()> {
    // Parses command line arguments.
    let args = Arguments::parse();

    // Reads the configuration from a file.
    let config = Config::read_from_file(args.config)?;

    // Sets the number of threads to be used in the rayon thread pool.
    if let Some(thread_num) = config.thread.thread_num {
        ThreadPoolBuilder::new()
            .num_threads(thread_num)
            .build_global()
            .with_context(|| "Failed to set the number of threads.")?;
    }

    // Reads the seed list from a file.
    let seeds = read_seed_from_file(&config)?;

    // Returns an error if the seed list is empty.
    ensure!(!seeds.is_empty(), "Seed list is empty.");

    // Builds the tester.
    build::build_tester(&config)?;

    // Builds the submission code.
    build::build_submission(&config)?;

    // Executes the tester and retrieve evaluations.
    let evaluation_table = evaluation::evaluate(&config, &seeds)?;

    // Shows statistics about scores and execution times.
    evaluation::show_statistics(&evaluation_table)?;

    // Outputs score and execution time record per seed to CSV file.
    evaluation::write_to_csv(&config.path.evaluation_record, &evaluation_table)?;

    Ok(())
}

/// Reads the seed list from a file.
///
/// From `#` to the end of the line is skipped as a comment.
/// For each line that is not a blank or comment-only line, read the first value separated by a whitespace character as the seed value.
///
/// # Errors
///
/// If the seed cannot be parsed into a non-negative integer, an error is generated.
fn read_seed_from_file(config: &Config) -> anyhow::Result<Vec<usize>> {
    let seeds = read_to_string(&config.path.seed_file)
        .with_context(|| format!("Failed to read seed file `{:?}`.", config.path.seed_file))?
        .lines()
        .filter_map(|line| line.split('#').next()?.split_whitespace().next())
        .map(|seed| {
            seed.parse::<usize>()
                .with_context(|| format!("Failed to parse `{}` as seed.", seed))
        })
        .collect::<Result<Vec<usize>, _>>()?;

    Ok(seeds)
}
