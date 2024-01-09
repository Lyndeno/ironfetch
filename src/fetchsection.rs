use std::fmt::Display;

use colored::Colorize;

pub const SEPARATOR: &str = ": ";

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
        self.0.push(value.into());
    }

    pub fn get_indent(&self) -> usize {
        let mut indent = 0;
        for line in &self.0 {
            let length = line.get_indent();
            indent = if length > indent { length } else { indent };
        }
        indent
    }
}

impl Display for FetchArray {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let indent = self.get_indent();
        let mut iter = self.0.iter().peekable();
        while let Some(line) = iter.next() {
            line.fmt(indent, f)?;
            if iter.peek().is_some() {
                writeln!(f)?;
            }
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
    /// Write the section to provided formatter
    ///
    /// # Errors
    ///
    /// Propogates writing errors
    pub fn fmt(&self, indent: usize, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:>indent$}{}{}",
            self.name.red().bold(),
            SEPARATOR,
            self.content,
            indent = indent
        )?;
        Ok(())
    }

    pub fn get_indent(&self) -> usize {
        self.name.len()
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
