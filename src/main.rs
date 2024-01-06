use std::path::Path;

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

use clap::Parser;

use ironfetch::args::Args;
use simplesmbios::smbios::SMBios;

fn main() {
    let args = Args::parse();
    let smbios_result = match args.smbios_path {
        Some(ref p) => SMBios::new_from_file(Path::new(p)),
        None => SMBios::new_from_device(),
    }
    .ok();

    let mut array = FetchArray::default();
    let smbios_ref = smbios_result.as_ref();

    if let Ok(r) = OsInfo::new() {
        array.push(("OS", r))
    }

    if let Ok(r) = Shell::new() {
        array.push(("Shell", r))
    }

    if let Ok(r) = Kernel::new() {
        array.push(("Kernel", r))
    }

    if let Ok(r) = Model::new() {
        array.push(("Model", r))
    }

    if let Ok(r) = HostName::new() {
        array.push(("Hostname", r))
    }

    if let Ok(r) = Uptime::new() {
        array.push(("Uptime", r))
    }

    if let Ok(r) = Cpu::new() {
        array.push(("CPU", r))
    }

    if let Ok(r) = Memory::new(args.memory_unit, smbios_ref) {
        array.push(("Memory", r))
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
