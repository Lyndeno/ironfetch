use std::path::Path;

use colored::Colorize;
use ironfetch::fetchsection::FetchArray;
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

    print!("{}", array);
    std::io::stdout().write_all(&colourblocks()).unwrap();
}

const COLOUR_RESET: &[u8; 4] = b"\x1b[0m";
use std::io::Write;

fn colourblocks() -> Vec<u8> {
    let mut blocks = Vec::<u8>::new();
    for i in 0..16u8 {
        if i == 8 {
            blocks.extend_from_slice(COLOUR_RESET);
            blocks.push(b'\n');
        }
        let mut buf = Vec::<u8>::new();
        write!(&mut buf, "\x1b[38;5;{}m\x1b[48;5;{}m   ", i, i).unwrap();
        blocks.append(&mut buf);
    }
    blocks.extend_from_slice(COLOUR_RESET);
    blocks.push(b'\n');
    blocks
}
