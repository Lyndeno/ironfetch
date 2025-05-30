use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use crate::{fetch::Fetch, Result};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Fetch)]
#[fetch(name = "Platform")]
pub struct Profile {
    current: String,
    choices: String,
}

impl Profile {
    /// Get system model information
    ///
    /// # Errors
    ///
    /// Returns io errors if information cannot be read
    pub fn new() -> Result<Self> {
        Ok(Self {
            current: read_product_info("/sys/firmware/acpi/platform_profile")?,
            choices: read_product_info("/sys/firmware/acpi/platform_profile_choices")?,
        })
    }
}

fn read_product_info(path: &str) -> Result<String> {
    let f = File::open(path)?;
    let mut s = String::new();
    BufReader::new(f).read_line(&mut s)?;
    Ok(s.replace('\n', ""))
}

impl std::fmt::Display for Profile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = self
            .choices
            .replace(&self.current, &("[".to_owned() + &self.current + "]"));
        write!(f, "{s}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn platform_profile_display() {
        let profile = Profile {
            current: "balanced".into(),
            choices: "performance balanced cool quiet".into(),
        };
        assert_eq!(profile.to_string(), "performance [balanced] cool quiet");
    }
}
