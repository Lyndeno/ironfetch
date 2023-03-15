use std::{
    fs::File,
    io::{BufRead, BufReader},
};
pub struct Model {
    product_name: String,
    board_vendor: String,
    board_name: String,
}

impl Model {
    pub fn new() -> Result<Self, std::io::Error> {
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
    Ok(s.replace("\n", ""))
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