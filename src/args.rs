use crate::memunit::MemUnits;
use clap::Parser;

/// A simple system fetcher
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Memory units to use
    #[arg(long, short, value_enum)]
    pub memory_unit: Option<MemUnits>,

    /// Number of colours to show
    #[arg(long, default_value = "16")]
    pub colours: usize,

    /// Number of colour lines to show
    #[arg(long, default_value = "8")]
    pub colour_length: usize,
}
