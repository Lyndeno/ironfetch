mod kernel;
use crate::kernel::Kernel;

mod cpu;
use crate::cpu::Cpu;

mod mem;
use crate::mem::Memory;

use clap::Parser;

use os_release;

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

fn main() {
    let args = Args::parse();
    let kernel_info = Kernel::new();
    let cpu_info = Cpu::new();
    
    let mem_info = Memory::new();
    let mem_display = match args.gigabyte {
        true => mem_info.display_gb(),
        false => match args.megabyte {
            true => mem_info.display_mb(),
            false => mem_info.display_gb(),
        }
    };

    let os = os_release::OsRelease::new().unwrap();
    println!("Distro: {}", os.pretty_name);
    match kernel_info {
        Ok(fl) => println!("Kernel: {}", fl),
        Err(e) => eprintln!("Error: {:?}", e),
    };
    println!("CPU: {}", cpu_info);
    println!("Memory: {}", mem_display);
    println!("OS: {}", os.pretty_name);
}
