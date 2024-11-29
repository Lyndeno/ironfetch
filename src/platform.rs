use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use crate::fetcherror::FetchError;
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
    pub fn new() -> Result<Self, FetchError> {
        Ok(Self {
            current: read_product_info("/sys/firmware/acpi/platform_profile")?,
            choices: read_product_info("/sys/firmware/acpi/platform_profile_choices")?,
        })
    }
}

fn read_product_info(path: &str) -> Result<String, std::io::Error> {
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
        write!(f, "{}", s)
    }
}
