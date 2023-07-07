use std::{env, path::PathBuf};

use crate::{fetcherror::FetchError, fetchitem::FetchItem};

pub struct Shell {
    pub path: PathBuf,
    pub version: String,
}

impl Shell {
    pub fn new() -> Result<Self, FetchError> {
        Ok(Self {
            path: PathBuf::from(env::var("SHELL")?),
            version: String::from(""),
        })
    }

    pub fn name(&self) -> Result<String, FetchError> {
        match self.path.file_name() {
            Some(v) => Ok(v.to_string_lossy().to_string()),
            None => Err(FetchError::OsStr),
        }
    }
}

impl std::fmt::Display for Shell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name().unwrap_or(String::from("")))
    }
}

impl FetchItem for Shell {
    fn name(&self) -> String {
        String::from("Shell")
    }

    fn long_content(&self) -> Option<Vec<crate::FetchSection>> {
        Some(vec![
            ("Name", self.name().unwrap_or("".to_string())).into(),
            ("Path", self.path.to_str().unwrap().to_string()).into(),
        ])
    }
}
