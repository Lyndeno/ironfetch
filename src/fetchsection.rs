use std::fmt::Display;

use colored::Colorize;

use crate::{fetcherror::FetchError, fetchitem::FetchItem};

const INDENT_LENGTH: usize = 4;

pub struct FetchArray(Vec<FetchSection>);

impl Default for FetchArray {
    fn default() -> Self {
        Self::new()
    }
}

impl FetchArray {
    pub fn new() -> Self {
        FetchArray(Vec::new())
    }

    pub fn with_capacity(capacity: usize) -> Self {
        FetchArray(Vec::with_capacity(capacity))
    }

    pub fn push(&mut self, value: FetchSection) {
        self.0.push(value)
    }

    // Discard Err and push ok
    pub fn push_ok(&mut self, value: Result<FetchSection, FetchError>) {
        if let Ok(v) = value {
            self.push(v)
        }
    }

    pub fn push_fetchitem<T: FetchItem>(&mut self, item: T, long: bool) {
        self.push(FetchSection::from(item, long))
    }

    pub fn push_fetchitem_ok<T: FetchItem>(
        &mut self,
        item: Result<T, FetchError>,
        long: bool,
        verbose: bool,
    ) {
        match item {
            Ok(v) => self.push_fetchitem(v, long),
            Err(e) => {
                if verbose {
                    eprint!("{}", e)
                }
            }
        }
    }
}

impl Display for FetchArray {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut indent = 0;
        for line in &self.0 {
            let length = line.get_indent(0);
            indent = if length > indent { length } else { indent };
        }
        for line in &self.0 {
            line.fmt(indent, f)?;
        }
        Ok(())
    }
}
pub enum FetchType {
    Short(String),
    Long(Vec<FetchSection>),
    None,
}

pub fn opt_fs<A, B>((name, content): (A, Option<B>)) -> FetchSection
where
    (A, B): Into<FetchSection>,
    A: Into<String>,
    B: Into<String>,
{
    match content {
        Some(v) => (name, v).into(),
        None => FetchSection {
            name: name.into(),
            content: FetchType::None,
        },
    }
}

/// Simple fetching program
pub struct FetchSection {
    pub name: String,
    pub content: FetchType,
}

impl FetchSection {
    pub fn fmt(&self, indent: usize, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.content {
            FetchType::Short(ref s) => writeln!(
                f,
                "{:>indent$}: {}",
                self.name.red().bold(),
                s,
                indent = indent
            )?,
            FetchType::Long(ref c) => {
                writeln!(f, "{:>indent$}:", self.name.red().bold(), indent = indent)?;
                for line in c {
                    line.fmt(indent + INDENT_LENGTH, f)?;
                }
            }
            FetchType::None => {}
        }
        Ok(())
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
            FetchType::None => {}
        };
        indent.saturating_sub(level * INDENT_LENGTH)
    }

    pub fn from<T>(value: T, long: bool) -> Self
    where
        T: FetchItem,
    {
        Self {
            name: value.name(),
            content: value.content(long),
        }
    }
}

impl<A: Into<String>, B: Into<String>> From<(A, B)> for FetchSection {
    fn from((name, content): (A, B)) -> Self {
        Self {
            name: name.into(),
            content: FetchType::Short(content.into()),
        }
    }
}
