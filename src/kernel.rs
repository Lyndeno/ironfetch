use nix::sys::utsname::{UtsName, uname};
use std::ffi::OsStr;

use crate::fetchline::{Fetchline};
pub struct Kernel {
    release: Option<String>,
    architecture: Option<String>,
    name: Option<String>,
}

impl Kernel {
    pub fn new() -> Self {
        let info: UtsName = uname().unwrap();
        Self {
            // TODO: Error correction
            release: Self::os_str_to_string(info.release()),
            architecture: Self::os_str_to_string(info.machine()),
            name: Self::os_str_to_string(info.sysname()),
        }
    }

    fn os_str_to_string(v: &OsStr) -> Option<String> {
        match v.to_str() {
            Some(s) => Some(String::from(s)),
            None => None,
        }
    }
}

impl std::fmt::Display for Kernel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.gen_fetchline(Vec::from([self.name.as_ref(), self.release.as_ref(), self.architecture.as_ref()])))
    }
}

impl Fetchline for Kernel { }