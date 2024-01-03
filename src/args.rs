use crate::memunit::MemUnits;
use clap::Parser;

/// A simple system fetcher
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Memory units to use
    #[arg(long, short, value_enum)]
    pub memory_unit: Option<MemUnits>,

    /// Print errors and other extra information
    #[arg(long, short)]
    pub debug: bool,

    #[arg(long, short, value_enum)]
    pub smbios_path: Option<String>,
}
