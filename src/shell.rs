use std::{env, path::PathBuf};

use crate::{fetcherror::FetchError, fetchitem::FetchItem, fetchsection::FetchSection};

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
}
