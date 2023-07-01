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

mod shell;
use crate::shell::Shell;

use clap::Parser;
use fetchitem::FetchItem;

mod fetchitem;

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

impl<T> From<T> for Fetchline
where
    T: FetchItem,
{
    fn from(value: T) -> Self {
        Self {
            name: value.name(),
            content: value.content(),
        }
    }
}

fn main() {
    let args = Args::parse();
    let mut lines: Vec<Fetchline> = Vec::with_capacity(8);

    lines.push(Fetchline::from(Distro::new().unwrap()));
    lines.push(Fetchline::from(Shell::new()));
    lines.push(Fetchline::from(Kernel::new().unwrap()));
    lines.push(Fetchline::from(Model::new().unwrap()));
    lines.push(Fetchline::from(HostName::new().unwrap()));
    lines.push(Fetchline::from(Uptime::new().unwrap()));
    lines.push(Fetchline::from(Cpu::new().unwrap()));

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
