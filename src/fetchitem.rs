use std::fmt;

use crate::fetchsection::{FetchSection, FetchType};

pub trait FetchItem: fmt::Display {
    fn name(&self) -> String;

    fn content(&self, long: bool) -> FetchType {
        match long {
            false => FetchType::Short(self.to_string()),
            true => match self.long_content() {
                Some(v) => FetchType::Long(v),
                None => FetchType::Short(self.to_string()),
            },
        }
    }

    fn long_content(&self) -> Option<Vec<FetchSection>> {
        None
    }
}
