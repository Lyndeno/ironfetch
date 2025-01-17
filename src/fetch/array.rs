use crate::fetch::Line;

use super::Fetch;

pub struct Array {
    sections: Vec<Line>,
    colour: Option<String>,
}

impl Default for Array {
    fn default() -> Self {
        Self::new()
    }
}

impl Array {
    pub fn new() -> Self {
        Array {
            sections: Vec::new(),
            colour: None,
        }
    }

    pub fn set_colour(&mut self, colour: Option<String>) {
        self.colour = colour;
    }

    pub fn push<T: Into<Line>>(&mut self, value: T) {
        self.sections.push(value.into());
    }

    pub fn push_multi<T: IntoIterator<Item = Line>>(&mut self, values: T) {
        for value in values {
            self.sections.push(value);
        }
    }

    pub fn push_obj<T: Fetch>(&mut self, value: &T) {
        self.push_multi(value.as_fetchlines());
    }

    pub fn push_obj_opt<T: Fetch>(&mut self, value: Option<T>) {
        if let Some(v) = value {
            self.push_obj(&v);
        }
    }

    pub fn get_indent(&self) -> usize {
        let mut indent = 0;
        for line in &self.sections {
            let length = line.get_indent();
            indent = if length > indent { length } else { indent };
        }
        indent
    }

    pub fn append(&mut self, value: &mut Self) {
        self.sections.append(&mut value.sections);
    }
}

impl From<Vec<Line>> for Array {
    fn from(value: Vec<Line>) -> Self {
        Self {
            sections: value,
            colour: None,
        }
    }
}

impl std::fmt::Display for Array {
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
