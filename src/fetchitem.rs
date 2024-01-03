use std::fmt;

use crate::fetchsection::FetchSection;

pub trait FetchItem: fmt::Display {
    fn name(&self) -> String;

    fn content(&self) -> String {
        self.to_string()
    }

    fn long_content(&self) -> Option<Vec<FetchSection>> {
        None
    }
}
