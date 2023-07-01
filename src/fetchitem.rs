use std::fmt;

pub trait FetchItem: fmt::Display {
    fn name(&self) -> String;

    fn content(&self) -> String {
        self.to_string()
    }
}
