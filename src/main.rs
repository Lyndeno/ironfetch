use clap::Parser;

use ironfetch::args::Args;
use ironfetch::machine::{Machine, ModuleFilter};
use ironfetch::Result;

fn main() -> Result<()> {
    let mut machine;
    let args = Args::parse();
    if let Some(path) = args.input {
        machine = Machine::from_file(path)?;
    } else {
        let filter = ModuleFilter {
            none: args.none,
            show: args.show,
            hide: args.hide,
        };
        machine = Machine::new(&filter);
        machine.colour_blocks = !args.no_colour_blocks;
        if let Some(path) = args.output {
            machine.to_file(path)?;
        }
    }
    println!("{machine}");
    Ok(())
}
