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

    push_fetchline(Distro::new(), &mut lines, args.verbose);
    push_fetchline(Shell::new(), &mut lines, args.verbose);
    push_fetchline(Kernel::new(), &mut lines, args.verbose);
    push_fetchline(Model::new(), &mut lines, args.verbose);
    push_fetchline(HostName::new(), &mut lines, args.verbose);
    push_fetchline(Uptime::new(), &mut lines, args.verbose);
    push_fetchline(Cpu::new(), &mut lines, args.verbose);
    push_fetchline(Memory::new(args.memory_unit), &mut lines, args.verbose);

    let mut indent = 0;
    for line in &mut lines {
        let length = line.name.len();
        indent = if length > indent { length } else { indent };
    }
    for line in lines {
        println!("{:>indent$}: {}", line.name, line.content, indent = indent);
    }
}

fn push_fetchline<T: FetchItem>(
    item: Result<T, FetchError>,
    line_array: &mut Vec<Fetchline>,
    print_error: bool,
) {
    match item {
        Ok(f) => line_array.push(Fetchline::from(f)),
        Err(e) => {
            if print_error {
                println!("Error: {}", e)
            }
        }
    };
}
