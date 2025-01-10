use std::{env::VarError, io, num::ParseIntError};

use nix::errno::Errno;
use procfs::ProcError;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Error getting system information")]
    Sys(#[from] sys_info::Error),
    #[error("IO Error")]
    Io(#[from] io::Error),
    #[error("Generic *nix error")]
    Nix(#[from] Errno),
    #[error("OsStr parsing error")]
    OsStr,
    #[error("Error parsing environment variable")]
    Var(#[from] VarError),
    #[error("Error getting uptime")]
    UpTime(#[from] uptime_lib::Error),
    #[error("Error parsing /proc")]
    Proc(#[from] ProcError),
    #[error("Error parsing Int to String")]
    ParseInt(#[from] ParseIntError),
    #[error("Error talking to udisks2")]
    Udisk(#[from] udisks2::Error),
    #[error("Error with serialization")]
    Serde(#[from] serde_json::Error),
    #[error("Error converting from None")]
    IsNone,
}
