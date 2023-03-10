mod kernel;
use crate::kernel::Kernel;

mod cpu;
use crate::cpu::Cpu;

mod mem;
use crate::mem::Memory;

use clap::Parser;

use os_release;

use std::fmt;

/// Simple fetching program
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Whether to use gigabytes
    #[arg(short, long)]
    gigabyte: bool,
    
    /// Whether to use megabytes
    #[arg(short, long)]
    megabyte: bool,
}

struct Fetchline {
    name: String,
    content: Box<dyn fmt::Display>,
}

fn main() {
    let args = Args::parse();
    let mut lines: Vec<Fetchline> = Vec::new();
    //let kernel_info = Kernel::new();
    match Kernel::new() {
        Ok(v) => {
            lines.push(Fetchline { name: "Kernel".to_string(), content: Box::new(v) });
        },
        Err(e) => eprintln!("Error: {:?}", e),
    };
    //let cpu_info = Cpu::new();
    lines.push(Fetchline { name: "CPU".to_string(), content: Box::new(Cpu::new()) });
    
    let mem_info = Memory::new();
    let mem_display = match args.gigabyte {
        true => mem_info.display_gb(),
        false => match args.megabyte {
            true => mem_info.display_mb(),
            false => mem_info.display_gb(),
        }
    };
    lines.push(Fetchline { name: "Memory".to_string(), content: Box::new(mem_display) });

    let os = os_release::OsRelease::new().unwrap();

    lines.push(Fetchline { name: "OS".to_string(), content:Box::new(os.pretty_name) });
    for line in lines {
        println!("{}: {}", line.name, line.content);
    }

}
