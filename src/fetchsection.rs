use std::fmt::Display;

use colored::Colorize;

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

    pub fn push<T: Into<FetchSection>>(&mut self, value: T) {
        self.0.push(value.into())
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
        let indent = self.name.len();
        indent.saturating_sub(level * INDENT_LENGTH)
    }
}

impl<A: Display, B: Display> From<(A, B)> for FetchSection {
    fn from((name, content): (A, B)) -> Self {
        Self {
            name: name.to_string(),
            content: content.to_string(),
        }
    }
}
