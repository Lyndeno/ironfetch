mod array;
mod line;

use std::fmt::Display;

pub use array::Array;
pub use line::{Line, SEPARATOR};

pub trait Fetch: Display + Clone {
    fn name(&self) -> &'static str;

    fn as_fetchlines(&self) -> Vec<Line> {
        vec![self.clone().into()]
    }
}
