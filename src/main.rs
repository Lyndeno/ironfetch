mod kernel;
use std::vec;

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
use fetcherror::FetchError;
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

    #[arg(long, short, default_value = "false")]
    verbose: bool,
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
    let lines_result = vec![
        gen_fl(Distro::new()),
        gen_fl(Shell::new()),
        gen_fl(Kernel::new()),
        gen_fl(Model::new()),
        gen_fl(HostName::new()),
        gen_fl(Uptime::new()),
        gen_fl(Cpu::new()),
        gen_fl(Memory::new(args.memory_unit)),
    ];

    for line in lines_result {
        match line {
            Ok(fl) => lines.push(fl),
            Err(e) => println!("Error: {}", e),
        };
    }

    let mut indent = 0;
    for line in &mut lines {
        let length = line.name.len();
        indent = if length > indent { length } else { indent };
    }
    for line in lines {
        println!("{:>indent$}: {}", line.name, line.content, indent = indent);
    }
}

fn gen_fl<T: FetchItem>(item: Result<T, FetchError>) -> Result<Fetchline, FetchError> {
    match item {
        Ok(f) => Ok(Fetchline::from(f)),
        Err(e) => Err(e),
    }
}
