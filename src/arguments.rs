use clap::Parser;

const DEFAULT_CONFIG_FILE_PATHNAME: &str = "evaluation/config.toml";
const DEFAULT_OUTPUT_EVALUATION_FILE_PATHNAME: &str = "evaluation/evaluation.csv";

#[derive(Debug, Clone, Parser)]
#[clap(author, version)]
pub struct Arguments {
    /// Pathname of the configuration file.
    #[clap(short = 'c', long = "config", default_value_t = String::from(DEFAULT_CONFIG_FILE_PATHNAME))]
    pub config: String,

    /// Pathname of the output file of evaluation.
    #[clap(short = 'd', long = "detail", default_value_t = String::from(DEFAULT_OUTPUT_EVALUATION_FILE_PATHNAME))]
    pub evaluation: String,

    #[clap(short = 'v', long = "visualization", default_value_t = false)]
    pub visualization: bool,
}
