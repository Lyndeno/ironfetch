use os_release::OsRelease;

use crate::{fetcherror::FetchError, fetchitem::FetchItem};

pub struct OsInfo {
    name: String,
    build_id: String,
    codename: String,
}

impl OsInfo {
    pub fn new() -> Result<Self, FetchError> {
        let os = OsRelease::new()?;
        Ok(Self {
            name: os.name,
            build_id: match os.extra.get("BUILD_ID") {
                Some(id) => id.replace('\"', ""),
                None => os.version_id.clone(),
            },
            codename: os.version_codename,
        })
    }
}

impl std::fmt::Display for OsInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} ({})", self.name, self.build_id, self.codename,)
    }
}

impl FetchItem for OsInfo {
    fn name(&self) -> String {
        String::from("OS")
    }
}
