use os_release::OsRelease;

use crate::{fetcherror::FetchError, fetchitem::FetchItem, fetchsection::FetchSection};

pub struct OsInfo {
    name: String,
    build_id: String,
    codename: String,
    home: String,
    bug: String,
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
            home: os.home_url,
            bug: os.bug_report_url,
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

    fn long_content(&self) -> Option<Vec<FetchSection>> {
        Some(vec![
            ("Name", self.name.clone()).into(),
            ("ID", self.build_id.clone()).into(),
            ("Codename", self.codename.clone()).into(),
            ("Home URL", self.home.clone()).into(),
            ("Bug Report URL", self.bug.clone()).into(),
        ])
    }
}
