use std::{env, path::PathBuf};

use crate::fetcherror::FetchError;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Shell {
    pub path: PathBuf,
    pub version: String,
}

impl Shell {
    /// Return shell information
    ///
    /// # Errors
    ///
    /// Returns error if shell variable cannot be read
    pub fn new() -> Result<Self, FetchError> {
        Ok(Self {
            path: PathBuf::from(env::var("SHELL")?),
            version: String::new(),
        })
    }

    /// Returns name of shell
    ///
    /// # Errors
    ///
    /// Returns an error if the string cannot be parsed
    pub fn name(&self) -> Result<String, FetchError> {
        match self.path.file_name() {
            Some(v) => Ok(v.to_string_lossy().to_string()),
            None => Err(FetchError::OsStr),
        }
    }
}

impl std::fmt::Display for Shell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name().unwrap_or_default())
    }
}
