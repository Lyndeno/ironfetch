use colored::Colorize;

use crate::fetchitem::FetchItem;

const INDENT_LENGTH: usize = 4;
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
            FetchType::None => {}
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
