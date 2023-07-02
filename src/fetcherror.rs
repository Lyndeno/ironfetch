use std::{
    env::{self, VarError},
    error::Error,
    fmt::Display,
    io,
};

use nix::errno::Errno;

#[derive(Debug)]
pub enum FetchError {
    SysError(sys_info::Error),
    IoError(io::Error),
    NixError(Errno),
    OsStrError,
    ProcError,
    VarError(VarError),
    StringError(String),
}

impl Display for FetchError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            FetchError::SysError(..) => write!(f, "Error getting system information"),
            FetchError::IoError(..) => write!(f, "IO error"),
            FetchError::NixError(..) => write!(f, "Generic *nix error"),
            FetchError::OsStrError => write!(f, "OsStr parsing error"),
            FetchError::ProcError => write!(f, "/proc parsing error"),
            FetchError::VarError(..) => write!(f, "Error parsing environment variable"),
            FetchError::StringError(ref s) => write!(f, "Error: {}", *s),
        }
    }
}

impl Error for FetchError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match *self {
            FetchError::SysError(ref e) => Some(e),
            FetchError::IoError(ref e) => Some(e),
            FetchError::NixError(ref e) => Some(e),
            FetchError::OsStrError => None,
            FetchError::ProcError => None,
            FetchError::VarError(ref e) => Some(e),
            FetchError::StringError(..) => None,
        }
    }
}

impl From<sys_info::Error> for FetchError {
    fn from(err: sys_info::Error) -> Self {
        Self::SysError(err)
    }
}

impl From<io::Error> for FetchError {
    fn from(err: io::Error) -> Self {
        Self::IoError(err)
    }
}

impl From<Errno> for FetchError {
    fn from(err: Errno) -> Self {
        Self::NixError(err)
    }
}

impl From<VarError> for FetchError {
    fn from(err: VarError) -> Self {
        Self::VarError(err)
    }
}

impl From<String> for FetchError {
    fn from(err: String) -> Self {
        Self::StringError(err)
    }
}
