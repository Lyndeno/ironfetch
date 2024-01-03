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
        self.push(FetchSection::from(item))
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

/// Simple fetching program
pub struct FetchSection {
    pub name: String,
    pub content: String,
}

impl FetchSection {
    pub fn fmt(&self, indent: usize, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "{:>indent$}: {}",
            self.name.red().bold(),
            self.content,
            indent = indent
        )?;
        Ok(())
    }

    pub fn get_indent(&self, level: usize) -> usize {
        let mut indent = self.name.len();
        indent.saturating_sub(level * INDENT_LENGTH)
    }

    pub fn from<T>(value: T) -> Self
    where
        T: FetchItem,
    {
        Self {
            name: value.name(),
            content: value.content(),
        }
    }
}

impl<A: Into<String>, B: Into<String>> From<(A, B)> for FetchSection {
    fn from((name, content): (A, B)) -> Self {
        Self {
            name: name.into(),
            content: content.into(),
        }
    }
}
