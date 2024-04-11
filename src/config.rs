use std::{
    fs::read_to_string,
    path::{Path, PathBuf},
};

use anyhow::Context;
use itertools::Itertools;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct ThreadConfig {
    /// Number of threads used for evaluation.
    /// If not specified, it is determined automatically.
    pub thread_num: Option<usize>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PathConfig {
    /// Path of the seed list file.
    pub seed_file: std::path::PathBuf,

    /// Path of the directory of input files.
    pub input_dir: std::path::PathBuf,

    /// Path of the directory of output files.
    pub output_dir: std::path::PathBuf,

    /// Path of the file that outputs the score and execution time for each seed.
    pub evaluation_record: std::path::PathBuf,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Build {
    /// Build command for submission code.
    pub submission: Vec<String>,

    /// Build command for local tester.
    pub tester: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Execute {
    /// Command line arguments to execute the submission code.
    pub submission: Vec<String>,

    /// Command line arguments to execute the local tester.
    ///
    /// The following placeholders can be used (Placeholders must be quoted independently).
    /// - `{input-path}`: The path of the input file corresponding to the seed.
    /// - `{output-path}`: The path of the output file corresponding to the seed.
    /// - `{submission-execute}`: Execution command of the submission code.
    pub tester: Vec<String>,

    /// Set this flag to `true` if the submission code is to be executed via the local tester rather than independently.
    pub integrated: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CommandConfig {
    /// Command line arguments to build codes.
    pub build: Build,

    /// Command line arguments to execute codes.
    pub execute: Execute,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    /// Configuration of threads.
    pub thread: ThreadConfig,

    /// Configuration of paths.
    pub path: PathConfig,

    /// Configuration of command line arguments.
    pub command: CommandConfig,
}

impl Config {
    /// Reads the configuration from the file.
    pub fn read_from_file<P>(config_file_path: P) -> anyhow::Result<Self>
    where
        P: AsRef<Path>,
    {
        let config_str = read_to_string(config_file_path)
            .with_context(|| "Failed to read configuration file.")?;
        toml::from_str(&config_str).with_context(|| "Failed to deserialize configuration file.")
    }

    /// Returns the path to the input file.
    pub fn input_file_path(&self, seed: usize) -> PathBuf {
        self.path.input_dir.join(format!("{:04}.txt", seed))
    }

    /// Returns the path to the output file.
    pub fn output_file_path(&self, seed: usize) -> PathBuf {
        self.path.output_dir.join(format!("{:04}.txt", seed))
    }

    /// Returns the command to execute the local tester with placeholders replaced.
    pub fn cmd_args_for_execute_tester(&self, seed: usize) -> Vec<String> {
        self.command
            .execute
            .tester
            .iter()
            .map(|arg| match arg.as_str() {
                "{input}" => self.input_file_path(seed).to_str().unwrap().to_owned(),
                "{output}" => self.output_file_path(seed).to_str().unwrap().to_owned(),
                "{cmd}" => self.command.execute.submission.iter().join(" "),
                _ => arg.clone(),
            })
            .collect()
    }
}
