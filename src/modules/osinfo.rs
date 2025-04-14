use os_release::OsRelease;

use crate::{fetch::Fetch, Result};
use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Fetch, Display)]
#[fetch(name = "OS")]
#[display("{} {} ({})", name, build_id, version_codename)]
pub struct OsInfo {
    pub name: String,
    pub build_id: String,
    pub color: Option<String>,
    pub version_codename: String,
}

impl OsInfo {
    /// Returns os-release information
    ///
    /// # Errors
    ///
    /// Returns errors if os-release cannot be parsed
    pub fn new() -> Result<Self> {
        let os = OsRelease::new()?;
        Ok(Self {
            name: os.name,
            build_id: match os.extra.get("BUILD_ID") {
                Some(id) => id.replace('\"', ""),
                None => os.version_id.clone(),
            },
            color: os
                .extra
                .get("ANSI_COLOR")
                .map(|x| x.trim_matches('"').to_owned()),
            version_codename: os.version_codename,
        })
    }
}
