use crate::fetchsection::FetchLine;

pub struct FetchArray {
    sections: Vec<FetchLine>,
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

    pub fn push<T: Into<FetchLine>>(&mut self, value: T) {
        self.sections.push(value.into());
    }

    pub fn push_multi<T: IntoIterator<Item = FetchLine>>(&mut self, values: T) {
        for value in values {
            self.sections.push(value);
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

impl From<Vec<FetchLine>> for FetchArray {
    fn from(value: Vec<FetchLine>) -> Self {
        Self {
            sections: value,
            colour: None,
        }
    }
}

impl std::fmt::Display for FetchArray {
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
