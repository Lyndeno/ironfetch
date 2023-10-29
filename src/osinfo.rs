use os_release::OsRelease;

use crate::{fetcherror::FetchError, fetchitem::FetchItem, fetchsection::FetchSection};

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

impl FetchItem for OsInfo {
    fn name(&self) -> String {
        String::from("OS")
    }

    fn long_content(&self) -> Option<Vec<FetchSection>> {
        Some(vec![
            ("Name", self.0.name.clone()).into(),
            ("ID", self.build_id()).into(),
            ("Codename", self.0.version_codename.clone()).into(),
            ("Home URL", self.0.home_url.clone()).into(),
            ("Bug Report URL", self.0.bug_report_url.clone()).into(),
        ])
    }
}
