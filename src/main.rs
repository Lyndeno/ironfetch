use std::path::Path;

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
        colourblocks(array.get_indent() + SEPARATOR.len())
    );
}

const COLOUR_RESET: &str = "\x1b[0m";
//use std::io::Write;
use std::fmt::Write;

fn colourblocks(indent: usize) -> String {
    let mut blocks = String::new();
    blocks.push_str(&spaces(indent));
    for i in 0..16u8 {
        if i == 8 {
            blocks.push_str(COLOUR_RESET);
            blocks.push('\n');
            blocks.push_str(&spaces(indent));
        }
        write!(&mut blocks, "\x1b[38;5;{}m\x1b[48;5;{}m   ", i, i)
            .expect("Could not write colourblocks for some reason");
    }
    blocks.push_str(COLOUR_RESET);
    blocks
}

fn spaces(count: usize) -> String {
    vec![' '; count].iter().collect()
}
