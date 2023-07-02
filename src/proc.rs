use std::fs::File;
use std::io::{BufRead, BufReader};

use crate::fetcherror::FetchError;
pub fn proc_parse_try(path: &str, fields: &[&str]) -> Result<String, FetchError> {
    for &field in fields {
        match proc_parse(path, field) {
            Ok(v) => return Ok(v),
            Err(FetchError::Proc) => {}
            Err(e) => return Err(e),
        };
    }
    Err(FetchError::Proc)
}

pub fn proc_parse(path: &str, field: &str) -> Result<String, FetchError> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let l = line?;
        if l.contains(field) {
            let n: Vec<&str> = l.split(':').collect();
            return Ok(n[1].trim().to_string());
        }
    }
    Err(FetchError::Proc)
}
