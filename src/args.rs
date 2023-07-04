use crate::memunit::MemUnits;
use clap::Parser;

/// A simple system fetcher
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Memory units to use
    #[arg(long, short, value_enum)]
    pub memory_unit: Option<MemUnits>,

    /// Print error and other extra information
    #[arg(long, short)]
    pub verbose: bool,
}
