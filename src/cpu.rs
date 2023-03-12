use std::{fs::File, io::BufReader, io::BufRead};

use sys_info::{cpu_num,cpu_speed,Error};

pub struct Cpu {
    core_count: u32,
    speed: u64,
    model: String,
}

impl Cpu {
    pub fn new() -> Result<Self, Error> {
        Ok(Self {
            core_count: cpu_num()?,
            speed: cpu_speed()?,
            model: read_cpu_model(),
        })
    }
}

impl std::fmt::Display for Cpu {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({}) @ {:.3}GHz", self.model, self.core_count, self.speed as f64 / 1000 as f64)
    }
}

// TODO: Error handling, these unwraps are gross
fn read_cpu_model() -> String {
    let path = "/proc/cpuinfo";
    match proc_parse(path, "model name") {
        Ok(Some(v)) => v,
        Err(_) => "ERROR".to_string(),
        Ok(None) => { match proc_parse(path, "Hardware") {
            Ok(Some(v)) => v,
            Err(_) => "Error".to_string(),
            Ok(None) => "N/A".to_string(),
        }}
    }
}

fn proc_parse(path: &str, field: &str) -> Result<Option<String>, std::io::Error> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let l = line?;
        if l.contains(field) {
            let n: Vec<&str> = l.split(":").collect();
            return Ok(Some(n[1].trim().to_string()))
        }
    }
    Ok(None)
}