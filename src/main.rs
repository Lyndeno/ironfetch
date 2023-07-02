mod kernel;
use crate::kernel::Kernel;

mod cpu;
use crate::cpu::Cpu;

mod mem;
use crate::mem::{MemUnits, Memory};

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

mod fetcherror;

/// Simple fetching program
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Memory units to use
    #[arg(long, value_enum)]
    memory_unit: Option<MemUnits>,
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
    lines.push(Fetchline::from(Shell::new().unwrap()));
    lines.push(Fetchline::from(Kernel::new().unwrap()));
    lines.push(Fetchline::from(Model::new().unwrap()));
    lines.push(Fetchline::from(HostName::new().unwrap()));
    lines.push(Fetchline::from(Uptime::new().unwrap()));
    lines.push(Fetchline::from(Cpu::new().unwrap()));

    lines.push(Fetchline::from(Memory::new(args.memory_unit).unwrap()));

    let mut indent = 0;
    for line in &mut lines {
        let length = line.name.len();
        indent = if length > indent { length } else { indent };
    }
    for line in lines {
        println!("{:>indent$}: {}", line.name, line.content, indent = indent);
    }
}
