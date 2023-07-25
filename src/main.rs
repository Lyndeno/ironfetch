use std::path::Path;
use std::vec;

use ironfetch::fetcherror::FetchError;
use ironfetch::fetchitem::FetchItem;
use ironfetch::fetchsection::FetchSection;
use ironfetch::kernel::Kernel;

use ironfetch::cpu::Cpu;

use ironfetch::memory::Memory;

use ironfetch::memory::mem::SMBiosSource;
use ironfetch::osinfo::OsInfo;

use ironfetch::hostname::HostName;

use ironfetch::uptime::Uptime;

use ironfetch::model::Model;

use ironfetch::shell::Shell;

use clap::Parser;

use ironfetch::args::Args;

fn main() {
    let args = Args::parse();
    let mut lines: Vec<FetchSection> = Vec::with_capacity(8);
    let smbios_source = match args.smbios_path {
        Some(ref p) => SMBiosSource::File(Path::new(p)),
        None => SMBiosSource::Local,
    };
    let lines_result = vec![
        gen_fl(OsInfo::new(), args.long),
        gen_fl(Shell::new(), args.long),
        gen_fl(Kernel::new(), args.long),
        gen_fl(Model::new(), args.long),
        gen_fl(HostName::new(), args.long),
        gen_fl(Uptime::new(), args.long),
        gen_fl(Cpu::new(), args.long),
        gen_fl(Memory::new(args.memory_unit, smbios_source), args.long),
    ];

    for line in lines_result {
        match line {
            Ok(fl) => lines.push(fl),
            Err(e) => {
                if args.debug {
                    eprintln!("Error: {}", e)
                }
            }
        };
    }

    let mut indent = 0;
    for line in &lines {
        let length = line.get_indent(0);
        indent = if length > indent { length } else { indent };
    }
    for line in lines {
        line.fmt(indent);
    }
}

fn gen_fl<T: FetchItem>(
    item: Result<T, FetchError>,
    long: bool,
) -> Result<FetchSection, FetchError> {
    match item {
        Ok(f) => Ok(FetchSection::from(f, long)),
        Err(e) => Err(e),
    }
}
