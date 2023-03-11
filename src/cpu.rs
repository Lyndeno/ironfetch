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
    let file = File::open("/proc/cpuinfo").unwrap();
    let reader = BufReader::new(file);

    let mut model = String::new();
    for line in reader.lines() {
        let l = line.unwrap();
        if l.contains("model name") {
            let n: Vec<&str> = l.split(":").collect();
            model = n[1].trim().to_string();
            break;
        }
    }
    model
}