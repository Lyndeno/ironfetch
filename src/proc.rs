use std::fs::File;
use std::io::{BufRead, BufReader};
pub fn proc_parse_try(path: &str, fields: &[&str]) -> Result<Option<String>, std::io::Error> {
    for &field in fields {
        match proc_parse(path, field)? {
            Some(v) => return Ok(Some(v)),
            None => {}
        }
    }
    Ok(None)
}

pub fn proc_parse(path: &str, field: &str) -> Result<Option<String>, std::io::Error> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let l = line?;
        if l.contains(field) {
            let n: Vec<&str> = l.split(":").collect();
            return Ok(Some(n[1].trim().to_string()));
        }
    }
    Ok(None)
}
