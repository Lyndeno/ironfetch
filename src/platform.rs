use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use crate::{fetchsection::FetchSection, Result};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
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

impl From<Profile> for FetchSection {
    fn from(value: Profile) -> Self {
        ("Profile", value).into()
    }
}
