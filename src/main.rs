mod kernel;
use std::vec;

use crate::kernel::Kernel;

mod cpu;
use crate::cpu::Cpu;

mod mem;
use crate::mem::Memory;

mod osinfo;
use crate::osinfo::OsInfo;

mod hostname;
use crate::hostname::HostName;

mod uptime;
use crate::uptime::Uptime;

mod model;
use crate::model::Model;

mod shell;
use crate::shell::Shell;

use clap::Parser;
use colored::Colorize;
use fetcherror::FetchError;
use fetchitem::FetchItem;

mod fetchitem;

mod fetcherror;

mod memunit;

mod args;
use crate::args::Args;

const INDENT_LENGTH: usize = 4;

pub enum FetchType {
    Short(String),
    Long(Vec<FetchSection>),
}

/// Simple fetching program
pub struct FetchSection {
    name: String,
    content: FetchType,
}

impl FetchSection {
    pub fn new_short<A: Into<String>, B: Into<String>>(name: A, content: B) -> Self {
        Self {
            name: name.into(),
            content: FetchType::Short(content.into()),
        }
    }

    pub fn fmt(&self, indent: usize) {
        match self.content {
            FetchType::Short(ref s) => println!(
                "{:>indent$}: {}",
                self.name.red().bold(),
                s,
                indent = indent
            ),
            FetchType::Long(ref c) => {
                println!("{:>indent$}:", self.name.red().bold(), indent = indent);
                for line in c {
                    line.fmt(indent + INDENT_LENGTH);
                }
            }
        }
    }

    pub fn get_indent(&self, level: usize) -> usize {
        let mut indent = self.name.len();
        match self.content {
            FetchType::Short(_) => {}
            FetchType::Long(ref v) => {
                for line in v {
                    let length = line.get_indent(level + 1);
                    indent = if length > indent { length } else { indent };
                }
            }
        };
        indent.saturating_sub(level * INDENT_LENGTH)
    }

    fn from<T>(value: T, long: bool) -> Self
    where
        T: FetchItem,
    {
        Self {
            name: value.name(),
            content: value.content(long),
        }
    }
}

fn main() {
    let args = Args::parse();
    let mut lines: Vec<FetchSection> = Vec::with_capacity(8);
    let lines_result = vec![
        gen_fl(OsInfo::new(), args.long),
        gen_fl(Shell::new(), args.long),
        gen_fl(Kernel::new(), args.long),
        gen_fl(Model::new(), args.long),
        gen_fl(HostName::new(), args.long),
        gen_fl(Uptime::new(), args.long),
        gen_fl(Cpu::new(), args.long),
        gen_fl(Memory::new(args.memory_unit), args.long),
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
