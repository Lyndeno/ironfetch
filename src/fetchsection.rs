use std::fmt::Display;

use std::fmt::Write;

pub const SEPARATOR: &str = ": ";

pub struct FetchArray {
    sections: Vec<FetchSection>,
    colour: Option<String>,
}

impl Default for FetchArray {
    fn default() -> Self {
        Self::new()
    }
}

impl FetchArray {
    pub fn new() -> Self {
        FetchArray {
            sections: Vec::new(),
            colour: None,
        }
    }

    pub fn set_colour(&mut self, colour: Option<String>) {
        self.colour = colour;
    }

    pub fn push<T: Into<FetchSection>>(&mut self, value: T) {
        self.sections.push(value.into());
    }

    pub fn get_indent(&self) -> usize {
        let mut indent = 0;
        for line in &self.sections {
            let length = line.get_indent();
            indent = if length > indent { length } else { indent };
        }
        indent
    }
}

impl Display for FetchArray {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let indent = self.get_indent();
        let mut iter = self.sections.iter().peekable();
        while let Some(line) = iter.next() {
            line.fmt(indent, f, self.colour.clone())?;
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
    pub fn fmt(
        &self,
        indent: usize,
        f: &mut std::fmt::Formatter<'_>,
        colour: Option<String>,
    ) -> std::fmt::Result {
        let name_text = format!("{:>indent$}", self.name);
        let name_coloured = match colour {
            Some(s) => {
                let mut t = String::new();
                write!(t, "\x1b[{s}m{name_text}\x1b[0m")?;
                t
            }
            None => self.name.clone(),
        };
        write!(f, "{}{}{}", name_coloured, SEPARATOR, self.content,)?;
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
