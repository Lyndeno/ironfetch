use std::{env::VarError, io};

use nix::errno::Errno;
use procfs::ProcError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum FetchError {
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
}
