use nix::sys::utsname::{uname, UtsName};
use std::ffi::OsStr;

type Result<T> = std::result::Result<T, KernelError>;

#[derive(Debug, Clone)]
pub struct KernelError;
pub struct Kernel {
    release: Option<String>,
    architecture: Option<String>,
    name: Option<String>,
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
            release: Self::os_str_to_string(info.release()),
            architecture: Self::os_str_to_string(info.machine()),
            name: Self::os_str_to_string(info.sysname()),
        })
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
        let mut s = String::new();
        let arr = [
            self.name.as_ref(),
            self.release.as_ref(),
            self.architecture.as_ref(),
        ];
        let mut index = 0;
        for item in arr {
            match item {
                Some(s2) => {
                    if index > 0 {
                        s.push_str(" ")
                    };
                    s.push_str(s2.to_string().as_str());
                    index = index + 1;
                }
                None => {}
            };
        }
        write!(f, "{}", s)
    }
}
