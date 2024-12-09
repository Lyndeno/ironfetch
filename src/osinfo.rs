use os_release::OsRelease;

use crate::fetcherror::FetchError;

pub struct OsInfo(OsRelease);

impl OsInfo {
    /// Returns os-release information
    ///
    /// # Errors
    ///
    /// Returns errors if os-release cannot be parsed
    pub fn new() -> Result<Self, FetchError> {
        Ok(Self(OsRelease::new()?))
    }

    fn build_id(&self) -> String {
        match self.0.extra.get("BUILD_ID") {
            Some(id) => id.replace('\"', ""),
            None => self.0.version_id.clone(),
        }
    }

    pub fn color(&self) -> Option<String> {
        self.0
            .extra
            .get("ANSI_COLOR")
            .map(|x| x.trim_matches('"').to_owned())
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
