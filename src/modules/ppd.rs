use crate::{fetch::Fetch, Result};
use ppd::PpdProxyBlocking;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Fetch)]
pub struct Ppd {
    current: String,
    choices: String,
}

impl Ppd {
    /// Get system model information
    ///
    /// # Errors
    ///
    /// Returns io errors if information cannot be read
    pub fn new() -> Result<Self> {
        let conn = zbus::blocking::Connection::system()?;
        let proxy = PpdProxyBlocking::new(&conn)?;
        let current = proxy.active_profile()?;
        let choice_vec: Vec<_> = proxy.profiles()?.into_iter().map(|v| v.profile).collect();
        let choices = choice_vec.join(" ");
        Ok(Self { current, choices })
    }
}

impl std::fmt::Display for Ppd {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = self
            .choices
            .replace(&self.current, &("[".to_owned() + &self.current + "]"));
        write!(f, "{s}")
    }
}
