use std::path::Path;

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
        array.push_fetchitem(r, args.long)
    }

    if let Ok(r) = Shell::new() {
        array.push_fetchitem(r, args.long)
    }

    if let Ok(r) = Kernel::new() {
        array.push_fetchitem(r, args.long)
    }

    if let Ok(r) = Model::new() {
        array.push_fetchitem(r, args.long)
    }

    if let Ok(r) = HostName::new() {
        array.push_fetchitem(r, args.long)
    }

    if let Ok(r) = Uptime::new() {
        array.push_fetchitem(r, args.long)
    }

    if let Ok(r) = Cpu::new() {
        array.push_fetchitem(r, args.long)
    }

    if let Ok(r) = Memory::new(args.memory_unit, smbios_ref) {
        array.push_fetchitem(r, args.long)
    }

    print!("{}", array);
}
