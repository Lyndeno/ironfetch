mod array;
mod line;

use std::fmt::Display;

pub use array::Array;
pub use line::{Line, SEPARATOR};

pub trait Fetch: Display {
    fn name(&self) -> &'static str;

    fn as_fetchsection(&self) -> Line {
        (self.name(), self).into()
    }

    fn as_fetchlines(&self) -> Vec<Line> {
        vec![self.as_fetchsection()]
    }
}
