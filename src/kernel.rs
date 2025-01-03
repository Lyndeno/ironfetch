use nix::sys::utsname::{uname, UtsName};
use std::ffi::OsStr;

use crate::fetcherror::FetchError;
use serde::{Deserialize, Serialize};

type Result<T> = std::result::Result<T, FetchError>;

#[derive(Serialize, Deserialize)]
pub struct Kernel {
    release: String,
    architecture: String,
    name: String,
}

impl Kernel {
    /// Return kernel information
    ///
    /// # Errors
    ///
    /// Returns an error if there is any problem getting kernel information or parsing the strings
    pub fn new() -> Result<Self> {
        let info: UtsName = uname()?;
        Ok(Self {
            // TODO: Error correction
            release: Self::os_str_to_string(info.release())?,
            architecture: Self::os_str_to_string(info.machine())?,
            name: Self::os_str_to_string(info.sysname())?,
        })
    }

    fn os_str_to_string(v: &OsStr) -> Result<String> {
        match v.to_str() {
            Some(s) => Ok(String::from(s)),
            None => Err(FetchError::OsStr),
        }
    }
}

impl std::fmt::Display for Kernel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.name, self.release, self.architecture)
    }
}
