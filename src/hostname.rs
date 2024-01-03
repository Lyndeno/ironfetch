use sys_info::hostname;

use crate::fetcherror::FetchError;
pub struct HostName(pub String);

impl HostName {
    pub fn new() -> Result<Self, FetchError> {
        Ok(Self(hostname()?))
    }
}

impl std::fmt::Display for HostName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
