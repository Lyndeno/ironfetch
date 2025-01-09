use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use crate::{fetchsection::AsFetchSection, Result};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Model {
    product_name: Option<String>,
    board_vendor: String,
    board_name: String,
}

impl Model {
    /// Get system model information
    ///
    /// # Errors
    ///
    /// Returns io errors if information cannot be read
    pub fn new() -> Result<Self> {
        Ok(Self {
            product_name: match read_product_info("/sys/devices/virtual/dmi/id/product_name")?
                .as_str()
            {
                "System Product Name" => None,
                s => Some(s.to_owned()),
            },
            board_vendor: read_product_info("/sys/devices/virtual/dmi/id/board_vendor")?,
            board_name: read_product_info("/sys/devices/virtual/dmi/id/board_name")?,
        })
    }
}

fn read_product_info(path: &str) -> Result<String> {
    let f = File::open(path)?;
    let mut s = String::new();
    BufReader::new(f).read_line(&mut s)?;
    Ok(s.replace('\n', ""))
}

impl std::fmt::Display for Model {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut text = String::new();
        text.push_str(&self.board_vendor);
        if let Some(v) = &self.product_name {
            text.push(' ');
            text.push_str(v);
        }
        text.push(' ');
        text.push_str(&self.board_name);
        write!(f, "{text}")
    }
}

impl AsFetchSection for Model {
    const NAME: &'static str = "Model";
}
