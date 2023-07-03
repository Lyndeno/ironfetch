use crate::memunit::MemUnits;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Memory units to use
    #[arg(long, value_enum)]
    pub memory_unit: Option<MemUnits>,

    #[arg(long, short, default_value = "false")]
    pub verbose: bool,
}
