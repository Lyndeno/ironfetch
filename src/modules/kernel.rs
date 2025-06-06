use nix::sys::utsname::{uname, UtsName};
use std::ffi::OsStr;

use serde::{Deserialize, Serialize};

use crate::{fetch::Fetch, Error, Result};

use derive_more::Display;

#[derive(Serialize, Deserialize, Clone, Fetch, Display)]
#[display("{} {} {}", name, release, architecture)]
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
    #[cfg(target_os = "linux")]
    pub fn new() -> Result<Option<Self>> {
        let info: UtsName = uname()?;
        Ok(Some(Self {
            // TODO: Error correction
            release: Self::os_str_to_string(info.release())?,
            architecture: Self::os_str_to_string(info.machine())?,
            name: Self::os_str_to_string(info.sysname())?,
        }))
    }
    #[cfg(not(target_os = "linux"))]
    pub fn new() -> Result<Option<Self>> {
        Ok(None)
    }

    fn os_str_to_string(v: &OsStr) -> Result<String> {
        match v.to_str() {
            Some(s) => Ok(String::from(s)),
            None => Err(Error::OsStr),
        }
    }
}
