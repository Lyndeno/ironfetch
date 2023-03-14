use std::{
    fs::File,
    io::{BufRead, BufReader},
};
pub struct Model(String);

impl Model {
    pub fn new() -> Result<Self, std::io::Error> {
        let f = File::open("/sys/devices/virtual/dmi/id/product_name")?;
        let mut s = String::new();
        BufReader::new(f).read_line(&mut s)?;
        Ok(Model(s.replace("\n", "")))
    }
}

impl std::fmt::Display for Model {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
