use os_release::OsRelease;

use crate::{fetch::AsLine, Result};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
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

impl std::fmt::Display for OsInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {} ({})",
            self.name, self.build_id, self.version_codename,
        )
    }
}

impl AsLine for OsInfo {
    const NAME: &'static str = "OS";
}
