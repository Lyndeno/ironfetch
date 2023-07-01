use sys_info::hostname;

use crate::fetchitem::FetchItem;
pub struct HostName(pub String);

impl HostName {
    pub fn new() -> Result<Self, sys_info::Error> {
        Ok(Self(hostname()?))
    }
}

impl std::fmt::Display for HostName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FetchItem for HostName {
    fn name(&self) -> String {
        String::from("Hostname")
    }
}
