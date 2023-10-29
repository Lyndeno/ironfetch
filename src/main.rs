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

    array.push_fetchitem_ok(OsInfo::new(), args.long, args.debug);
    array.push_fetchitem_ok(Shell::new(), args.long, args.debug);
    array.push_fetchitem_ok(Kernel::new(), args.long, args.debug);
    array.push_fetchitem_ok(Model::new(), args.long, args.debug);
    array.push_fetchitem_ok(HostName::new(), args.long, args.debug);
    array.push_fetchitem_ok(Uptime::new(), args.long, args.debug);
    array.push_fetchitem_ok(Cpu::new(), args.long, args.debug);
    array.push_fetchitem_ok(
        Memory::new(args.memory_unit, smbios_ref),
        args.long,
        args.debug,
    );

    print!("{}", array);
}
