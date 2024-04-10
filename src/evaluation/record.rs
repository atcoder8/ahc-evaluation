use std::path::Path;

use anyhow::{ensure, Context};
use itertools::Itertools;

#[derive(Debug, Clone, Copy, serde::Serialize)]
pub struct EvaluationRecord {
    pub seed: usize,
    pub score: i64,
    pub execution_time: f64,
}

/// Shows statistics about scores and execution times.
pub fn show_statistics(evaluation_table: &[EvaluationRecord]) -> anyhow::Result<()> {
    show_score_statistics(evaluation_table)?;
    println!();
    show_execution_time_statistics(evaluation_table)?;

    Ok(())
}

/// Shows score statistics.
fn show_score_statistics(evaluation_table: &[EvaluationRecord]) -> anyhow::Result<()> {
    ensure!(
        !evaluation_table.is_empty(),
        "The evaluation table is empty."
    );

    let total_score = evaluation_table
        .iter()
        .map(|record| record.score)
        .sum::<i64>();
    let avg_score = total_score as f64 / evaluation_table.len() as f64;

    let (min_pos, max_pos) = match evaluation_table
        .iter()
        .position_minmax_by_key(|record| record.score)
    {
        itertools::MinMaxResult::NoElements => unreachable!(),
        itertools::MinMaxResult::OneElement(pos) => (pos, pos),
        itertools::MinMaxResult::MinMax(min_pos, max_pos) => (min_pos, max_pos),
    };

    let min_record = evaluation_table[min_pos];
    let max_record = evaluation_table[max_pos];

    print!(
        "\
[Score Statistics]
Total: {}
Average: {:.3}
Min: {} (seed = {})
Max: {} (seed = {})
",
        total_score,
        avg_score,
        min_record.score,
        min_record.seed,
        max_record.score,
        max_record.seed,
    );

    Ok(())
}

/// Shows execution time statistics.
fn show_execution_time_statistics(evaluation_table: &[EvaluationRecord]) -> anyhow::Result<()> {
    ensure!(
        !evaluation_table.is_empty(),
        "The evaluation table is empty."
    );

    let total_exe_time = evaluation_table
        .iter()
        .map(|record| record.execution_time)
        .sum::<f64>();
    let avg_exe_time = total_exe_time / evaluation_table.len() as f64;

    let (min_pos, max_pos) = match evaluation_table
        .iter()
        .position_minmax_by(|x, y| x.execution_time.partial_cmp(&y.execution_time).unwrap())
    {
        itertools::MinMaxResult::NoElements => unreachable!(),
        itertools::MinMaxResult::OneElement(pos) => (pos, pos),
        itertools::MinMaxResult::MinMax(min_pos, max_pos) => (min_pos, max_pos),
    };

    let min_record = evaluation_table[min_pos];
    let max_record = evaluation_table[max_pos];

    print!(
        "\
[Execution Time]
Total: {:.3}
Average: {:.3}
Min: {:.3} (seed = {})
Max: {:.3} (seed = {})
",
        total_exe_time,
        avg_exe_time,
        min_record.execution_time,
        min_record.seed,
        max_record.execution_time,
        max_record.seed,
    );

    Ok(())
}

/// Outputs score and execution time record per seed to CSV file.
pub fn write_to_csv<P>(
    output_file_path: P,
    evaluation_table: &[EvaluationRecord],
) -> anyhow::Result<()>
where
    P: AsRef<Path>,
{
    let mut writer = csv::Writer::from_path(&output_file_path)
        .with_context(|| "Failed to open file to output evaluation table.")?;

    for record in evaluation_table {
        writer
            .serialize(record)
            .with_context(|| "Failed to serialize the evaluation record.")?;
    }

    writer
        .flush()
        .with_context(|| "Failed to write evaluation table to file.")?;

    Ok(())
}
