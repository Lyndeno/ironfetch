use sys_info::hostname;
pub struct HostName(String);

impl HostName {
    pub fn new() -> Result<String, sys_info::Error> {
        hostname()
    }
}