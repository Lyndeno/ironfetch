mod array;
mod line;

pub use array::Array;
pub use line::{AsLine, AsLines, Line, SEPARATOR};

pub trait IntoFetch: AsLines {}
