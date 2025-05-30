use std::{env, path::PathBuf};

use crate::{fetch::Fetch, Error, Result};
use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Fetch, Display)]
#[display("{}", self.name().unwrap_or_default())]
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
