mod fetcharray;
mod fetchline;

pub use fetcharray::Array;
pub use fetchline::{AsLine, AsLines, Line, SEPARATOR};

pub trait IntoFetch: AsLines {}
