use crate::proc::proc_parse_try;
use sys_info::{cpu_num, cpu_speed, Error};

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
        write!(
            f,
            "{} ({}) @ {:.3}GHz",
            self.model,
            self.core_count,
            self.speed as f64 / 1000 as f64
        )
    }
}

// TODO: Error handling, these unwraps are gross
fn read_cpu_model() -> String {
    let path = "/proc/cpuinfo";
    match proc_parse_try(path, &["model name", "Hardware"]) {
        Ok(Some(v)) => v,
        Ok(None) => "N/A".to_string(),
        Err(_) => "ERROR".to_string(),
    }
}
