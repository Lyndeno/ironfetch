use std::{env, path::PathBuf};

use crate::fetchitem::FetchItem;

pub struct Shell {
    pub path: PathBuf,
    pub version: String,
}

impl Shell {
    pub fn new() -> Self {
        Self {
            path: PathBuf::from(env::var("SHELL").unwrap()),
            version: String::from(""),
        }
    }

    pub fn name(&self) -> String {
        self.path.file_name().unwrap().to_str().unwrap().to_string()
    }
}

impl std::fmt::Display for Shell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}

impl FetchItem for Shell {
    fn name(&self) -> String {
        String::from("Shell")
    }
}
