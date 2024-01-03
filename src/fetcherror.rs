use std::{env::VarError, error::Error, fmt::Display, io};

use nix::errno::Errno;

#[derive(Debug)]
pub enum FetchError {
    Sys(sys_info::Error),
    Io(io::Error),
    Nix(Errno),
    OsStr,
    Var(VarError),
    UpTime,
}

impl Display for FetchError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            FetchError::Sys(..) => write!(f, "Error getting system information"),
            FetchError::Io(..) => write!(f, "IO error"),
            FetchError::Nix(..) => write!(f, "Generic *nix error"),
            FetchError::OsStr => write!(f, "OsStr parsing error"),
            FetchError::Var(..) => write!(f, "Error parsing environment variable"),
            FetchError::UpTime => write!(f, "Error getting Uptime"),
        }
    }
}

impl Error for FetchError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match *self {
            FetchError::Sys(ref e) => Some(e),
            FetchError::Io(ref e) => Some(e),
            FetchError::Nix(ref e) => Some(e),
            FetchError::OsStr => None,
            FetchError::Var(ref e) => Some(e),
            FetchError::UpTime => None,
        }
    }
}

impl From<sys_info::Error> for FetchError {
    fn from(err: sys_info::Error) -> Self {
        Self::Sys(err)
    }
}

impl From<io::Error> for FetchError {
    fn from(err: io::Error) -> Self {
        Self::Io(err)
    }
}

impl From<Errno> for FetchError {
    fn from(err: Errno) -> Self {
        Self::Nix(err)
    }
}

impl From<VarError> for FetchError {
    fn from(err: VarError) -> Self {
        Self::Var(err)
    }
}

impl From<uptime_lib::Error> for FetchError {
    fn from(_err: uptime_lib::Error) -> Self {
        Self::UpTime
    }
}
