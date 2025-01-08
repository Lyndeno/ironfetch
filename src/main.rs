use clap::Parser;

use ironfetch::args::Args;
use ironfetch::machine::Machine;
use ironfetch::Result;

fn main() -> Result<()> {
    let machine;
    let args = Args::parse();
    if let Some(path) = args.input {
        machine = Machine::from_file(path)?;
    } else {
        machine = Machine::new();
        if let Some(path) = args.output {
            machine.to_file(path)?;
        }
    }
    println!("{machine}");
    Ok(())
}
