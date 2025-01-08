use sys_info::hostname;

use serde::{Deserialize, Serialize};

use crate::Result;

#[derive(Serialize, Deserialize)]
pub struct HostName(pub String);

impl HostName {
    /// Return system hostname
    ///
    /// # Errors
    ///
    /// Returns error if hostname cannot be obtained
    pub fn new() -> Result<Self> {
        Ok(Self(hostname()?))
    }
}

impl std::fmt::Display for HostName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
