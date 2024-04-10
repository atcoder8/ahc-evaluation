use clap::Parser;

const DEFAULT_CONFIG_FILE_PATHNAME: &str = "evaluation/config.toml";

#[derive(Debug, Clone, Parser)]
#[clap(author, version, about)]
pub struct Arguments {
    /// Path of the configuration file.
    #[clap(short = 'c', long = "config", default_value_t = String::from(DEFAULT_CONFIG_FILE_PATHNAME))]
    pub config: String,
}
