mod kernel;
use crate::kernel::Kernel;

mod cpu;
use crate::cpu::Cpu;

mod mem;
use crate::mem::Memory;

mod distro;
use crate::distro::Distro;

mod hostname;
mod proc;
use crate::hostname::HostName;

mod uptime;
use crate::uptime::Uptime;

mod model;
use crate::model::Model;

use clap::Parser;

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
    content: String,
}

fn main() {
    let args = Args::parse();
    let mut lines: Vec<Fetchline> = Vec::new();

    match Distro::new() {
        Ok(os) => lines.push(Fetchline {
            name: "OS".to_string(),
            content: os.to_string(),
        }),
        Err(e) => eprintln!("Error: {:?}", e),
    };

    //let kernel_info = Kernel::new();
    match Kernel::new() {
        Ok(v) => {
            lines.push(Fetchline {
                name: "Kernel".to_string(),
                content: v.to_string(),
            });
        }
        Err(e) => eprintln!("Error: {:?}", e),
    };

    match Model::new() {
        Ok(v) => {
            lines.push(Fetchline {
                name: "Model".to_string(),
                content: v.to_string(),
            });
        }
        Err(e) => eprintln!("Error: {:?}", e),
    };

    match HostName::new() {
        Ok(v) => {
            lines.push(Fetchline {
                name: "Hostname".to_string(),
                content: v.to_string(),
            });
        }
        Err(e) => eprintln!("Error: {:?}", e),
    };
    match Uptime::new() {
        Ok(v) => {
            lines.push(Fetchline {
                name: "Uptime".to_string(),
                content: v.to_string(),
            });
        }
        Err(e) => eprintln!("Error: {:?}", e),
    };
    //let cpu_info = Cpu::new();
    match Cpu::new() {
        Ok(c) => lines.push(Fetchline {
            name: "CPU".to_string(),
            content: c.to_string(),
        }),
        Err(e) => eprint!("Error: {:?}", e),
    };

    let mem_info = Memory::new();
    let mem_display = match args.gigabyte {
        true => mem_info.display_gb(),
        false => match args.megabyte {
            true => mem_info.display_mb(),
            false => mem_info.display_gb(),
        },
    };
    lines.push(Fetchline {
        name: "Memory".to_string(),
        content: mem_display,
    });

    let mut indent = 0;
    for line in &mut lines {
        let length = line.name.len();
        indent = if length > indent { length } else { indent };
    }
    for line in lines {
        println!("{:>indent$}: {}", line.name, line.content, indent = indent);
    }
}
