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
}
