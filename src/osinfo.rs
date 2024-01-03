use os_release::OsRelease;

use crate::fetcherror::FetchError;

pub struct OsInfo(OsRelease);

impl OsInfo {
    pub fn new() -> Result<Self, FetchError> {
        Ok(Self(OsRelease::new()?))
    }

    fn build_id(&self) -> String {
        match self.0.extra.get("BUILD ID") {
            Some(id) => id.replace('\"', ""),
            None => self.0.version_id.clone(),
        }
    }
}

impl std::fmt::Display for OsInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {} ({})",
            self.0.name,
            self.build_id(),
            self.0.version_codename,
        )
    }
}
