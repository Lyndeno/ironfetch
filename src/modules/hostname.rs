use sys_info::hostname;

use derive_more::Display;
use serde::{Deserialize, Serialize};

use crate::{fetch::Fetch, Result};

#[derive(Serialize, Deserialize, Clone, Fetch, Display)]
#[fetch(name = "Hostname")]
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
