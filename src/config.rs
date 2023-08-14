use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Path {
    /// Path of the input seed list file to be used for evaluation.
    pub seed_file: Box<std::path::Path>,

    /// Path of the directory of input files.
    pub input_dir: Box<std::path::Path>,

    /// Path of the directory of the output destination.
    pub output_dir: Box<std::path::Path>,

    // /// Path for moving the visualization results files.
    // pub move_vis_file: Option<MoveVisFile>,

    // /// Pathname of the visualization results output by the tester.
    // pub vis_file: Option<Box<std::path::Path>>,

    // /// Pathname of the directory to output the visualization results.
    // pub vis_dir: Option<Box<std::path::Path>>,
}

// #[derive(Debug, Clone, Deserialize)]
// pub struct MoveVisFile {
//     /// Pathname of the visualization results output by the tester.
//     pub source_file: Option<Box<std::path::Path>>,

//     /// Pathname of the directory to output the visualization results.
//     pub destination_dir: Option<Box<std::path::Path>>,
// }

#[derive(Debug, Clone, Deserialize)]
pub struct Thread {
    /// Number of concurrent executions.
    pub thread_num: Option<usize>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Build {
    /// Command line arguments to build tester.
    pub tester: Vec<String>,

    /// Command line arguments to build the program to be submitted.
    pub submission: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Run {
    /// Command line arguments to run tester.
    pub tester: Vec<String>,

    /// Command line arguments to run the program to be submitted.
    pub submission: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Command {
    /// Command line arguments to build programs.
    pub build: Build,

    /// Command line arguments to run programs.
    pub run: Run,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    /// Configuration of paths used for evaluation.
    pub path: Path,

    /// Configuration of threads used for evaluation.
    pub thread: Thread,

    /// Configuration of command line arguments for evaluation.
    pub command: Command,
}
