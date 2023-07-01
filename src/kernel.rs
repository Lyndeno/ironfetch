use nix::sys::utsname::{uname, UtsName};
use std::ffi::OsStr;

use crate::fetchitem::FetchItem;

type Result<T> = std::result::Result<T, KernelError>;

#[derive(Debug, Clone)]
pub struct KernelError;
pub struct Kernel {
    release: String,
    architecture: String,
    name: String,
}

impl std::fmt::Display for KernelError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Could not get kernel information!")
    }
}

impl Kernel {
    pub fn new() -> Result<Self> {
        let info: UtsName = uname().map_err(|_| KernelError)?;
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
            None => Err(KernelError),
        }
    }
}

impl std::fmt::Display for Kernel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.name, self.release, self.architecture)
    }
}

impl FetchItem for Kernel {
    fn name(&self) -> String {
        String::from("Kernel")
    }
}
