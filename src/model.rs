use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use crate::{fetcherror::FetchError, fetchitem::FetchItem, fetchsection::FetchSection};
pub struct Model {
    product_name: String,
    board_vendor: String,
    board_name: String,
}

impl Model {
    pub fn new() -> Result<Self, FetchError> {
        Ok(Self {
            product_name: read_product_info("/sys/devices/virtual/dmi/id/product_name")?,
            board_vendor: read_product_info("/sys/devices/virtual/dmi/id/board_vendor")?,
            board_name: read_product_info("/sys/devices/virtual/dmi/id/board_name")?,
        })
    }
}

fn read_product_info(path: &str) -> Result<String, std::io::Error> {
    let f = File::open(path)?;
    let mut s = String::new();
    BufReader::new(f).read_line(&mut s)?;
    Ok(s.replace('\n', ""))
}

impl std::fmt::Display for Model {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {} {}",
            self.board_vendor, self.product_name, self.board_name
        )
    }
}

impl FetchItem for Model {
    fn name(&self) -> String {
        String::from("Model")
    }

    fn long_content(&self) -> Option<Vec<FetchSection>> {
        Some(vec![
            ("Vendor", self.board_vendor.clone()).into(),
            ("Product", self.product_name.clone()).into(),
            ("Board", self.board_name.clone()).into(),
        ])
    }
}
