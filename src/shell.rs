use std::{env, path::PathBuf};

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

    pub fn name(self) -> String {
        self.path.file_name().unwrap().to_str().unwrap().to_string()
    }
}
