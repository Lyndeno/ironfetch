use std::fmt::Display;

use std::fmt::Write;

use crate::Error;
use crate::Result;

pub const SEPARATOR: &str = ": ";
/// Simple fetching program
pub struct FetchLine {
    pub name: String,
    pub content: String,
}

impl FetchLine {
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

impl<A: Display, B: Display> From<(A, B)> for FetchLine {
    fn from((name, content): (A, B)) -> Self {
        Self {
            name: name.to_string(),
            content: content.to_string(),
        }
    }
}

impl<T: AsFetchSection> From<T> for FetchLine {
    fn from(value: T) -> Self {
        value.as_fetchsection()
    }
}

pub trait AsFetchSection: Display {
    const NAME: &'static str;

    fn as_fetchsection(&self) -> FetchLine {
        (Self::NAME, self).into()
    }
}

impl<T: AsFetchSection> TryFrom<Result<T>> for FetchLine {
    type Error = Error;
    fn try_from(value: Result<T>) -> std::result::Result<Self, Self::Error> {
        value.map(Into::into)
    }
}

impl<T: AsFetchSection> TryFrom<Option<T>> for FetchLine {
    type Error = Error;
    fn try_from(value: Option<T>) -> std::result::Result<Self, Self::Error> {
        value.map(Into::into).ok_or(Error::IsNone)
    }
}
