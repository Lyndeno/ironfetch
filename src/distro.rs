use os_release::OsRelease;

use crate::{fetcherror::FetchError, fetchitem::FetchItem};

pub struct Distro(OsRelease);

impl Distro {
    pub fn new() -> Result<Self, FetchError> {
        let os = OsRelease::new()?;
        Ok(Self(os))
    }
}

impl std::fmt::Display for Distro {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {} ({})",
            self.0.name,
            match self.0.extra.get("BUILD_ID") {
                Some(id) => id.replace('\"', ""),
                None => self.0.version_id.clone(),
            },
            self.0.version_codename
        )
    }
}

impl FetchItem for Distro {
    fn name(&self) -> String {
        String::from("OS")
    }
}
