use nix::sys::utsname::{UtsName, uname};
use std::ffi::OsStr;
fn main() {
    println!("Hello, world!");
    let kernel_info = Kernel::new();
    println!("{}", kernel_info);
}

struct Kernel {
    release: Option<String>,
    architecture: Option<String>,
    name: Option<String>,
}

impl Kernel {
    pub fn new() -> Self {
        let info: UtsName = uname().unwrap();
        Self {
            // TODO: Error correction
            release: Self::OsStr_to_String(info.release()),
            architecture: Self::OsStr_to_String(info.machine()),
            name: Self::OsStr_to_String(info.sysname()),
        }
    }

    fn OsStr_to_String(v: &OsStr) -> Option<String> {
        match v.to_str() {
            Some(s) => Some(String::from(s)),
            None => None,
        }
    }
}

impl std::fmt::Display for Kernel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();
        if let Some(v) = &self.name {
            s.push_str(v.as_str());
        };
        if let Some(v) = &self.release {
            s.push_str(" ");
            s.push_str(v.as_str());
        };
        if let Some(v) = &self.architecture {
            s.push_str(" ");
            s.push_str(v.as_str());
        };

        write!(f, "{}", s)
    }
}