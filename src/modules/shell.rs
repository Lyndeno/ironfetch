use std::{env, path::PathBuf};

use crate::{
    fetch::{AsLine, AsLines, IntoFetch},
    Error, Result,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
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
    pub fn new() -> Result<Self> {
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
    pub fn name(&self) -> Result<String> {
        match self.path.file_name() {
            Some(v) => Ok(v.to_string_lossy().to_string()),
            None => Err(Error::OsStr),
        }
    }
}

impl std::fmt::Display for Shell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name().unwrap_or_default())
    }
}

impl AsLine for Shell {
    fn name(&self) -> &'static str {
        "Shell"
    }
}

impl AsLines for Shell {}

impl IntoFetch for Shell {}
