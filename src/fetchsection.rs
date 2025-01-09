use std::fmt::Display;

use std::fmt::Write;

pub const SEPARATOR: &str = ": ";
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

impl<T: AsFetchSection> From<T> for FetchSection {
    fn from(value: T) -> Self {
        value.as_fetchsection()
    }
}

pub trait AsFetchSection: Display {
    const NAME: &'static str;

    fn as_fetchsection(&self) -> FetchSection {
        (Self::NAME, self).into()
    }
}
