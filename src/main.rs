use ironfetch::colourblocks::colourblocks;
use ironfetch::fetchsection::{FetchArray, SEPARATOR};
use ironfetch::kernel::Kernel;

use ironfetch::cpu::Cpu;

use ironfetch::memory::Memory;

use ironfetch::osinfo::OsInfo;

use ironfetch::hostname::HostName;

use ironfetch::uptime::Uptime;

use ironfetch::model::Model;

use ironfetch::shell::Shell;

use ironfetch::platform::Profile;

use clap::Parser;

use ironfetch::args::Args;

fn main() {
    let args = Args::parse();

    let mut array = FetchArray::default();

    if let Ok(r) = OsInfo::new() {
        array.set_colour(r.color());
        array.push(("OS", r));
    }

    if let Ok(r) = Shell::new() {
        array.push(("Shell", r));
    }

    if let Ok(r) = Kernel::new() {
        array.push(("Kernel", r));
    }

    if let Ok(r) = Model::new() {
        array.push(("Model", r));
    }

    if let Ok(r) = HostName::new() {
        array.push(("Hostname", r));
    }

    if let Ok(r) = Uptime::new() {
        array.push(("Uptime", r));
    }

    if let Ok(r) = Cpu::new() {
        array.push(("CPU", r));
    }

    if let Ok(r) = Memory::new(args.memory_unit) {
        array.push(("Memory", r));
    }

    if let Ok(r) = Profile::new() {
        array.push(("Profile", r));
    }

    println!(
        "{}\n{}",
        array,
        colourblocks(
            array.get_indent() + SEPARATOR.len(),
            args.colours,
            args.colour_length
        )
    );
}
