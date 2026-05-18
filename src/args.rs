use std::path::PathBuf;

use clap::Parser;

/// A simple system fetcher
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Import machine state
    #[arg(short, long)]
    pub input: Option<PathBuf>,

    /// Export machine state
    #[arg(short, long)]
    pub output: Option<PathBuf>,

    /// Show no modules by default; use --show to add specific ones
    #[arg(long)]
    pub none: bool,

    /// Modules to show; comma-separated or repeated, case-insensitive (e.g. --show gpu,memory)
    #[arg(long, value_delimiter = ',', value_name = "MODULE")]
    pub show: Vec<String>,

    /// Modules to hide; comma-separated or repeated, case-insensitive (e.g. --hide gpu,memory)
    #[arg(long, value_delimiter = ',', value_name = "MODULE")]
    pub hide: Vec<String>,

    /// Hide the colour blocks at the bottom
    #[arg(long)]
    pub no_colour_blocks: bool,
}
