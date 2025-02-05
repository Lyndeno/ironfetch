pub mod args;
pub mod colourblocks;
mod error;
pub mod fetch;
pub mod machine;
pub mod modules;

pub use error::{Error, Result};

pub(crate) const KIBIBYTE: u64 = 1024;
pub(crate) const MEBIBYTE: u64 = KIBIBYTE * 1024;
pub(crate) const GIBIBYTE: u64 = MEBIBYTE * 1024;
#[allow(dead_code)]
pub(crate) const TEBIBYTE: u64 = GIBIBYTE * 1024;

pub(crate) const KILOBYTE: u64 = 1000;
pub(crate) const MEGABYTE: u64 = KILOBYTE * 1000;
pub(crate) const GIGABYTE: u64 = MEGABYTE * 1000;
pub(crate) const TERABYTE: u64 = GIGABYTE * 1000;
