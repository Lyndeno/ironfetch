use crate::{fetcherror::FetchError, fetchitem::FetchItem, proc::proc_parse_try};
use sys_info::{cpu_num, cpu_speed};

pub struct Cpu {
    core_count: u32,
    speed: u64,
    model: String,
}

impl Cpu {
    pub fn new() -> Result<Self, FetchError> {
        Ok(Self {
            core_count: cpu_num()?,
            speed: cpu_speed()?,
            model: read_cpu_model()?,
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
            self.speed as f64 / 1000_f64
        )
    }
}

fn read_cpu_model() -> Result<String, FetchError> {
    match proc_parse_try("/proc/cpuinfo", &["model name", "Hardware"]) {
        Ok(v) => Ok(v),
        Err(FetchError::ProcError) => Ok("N/A".to_string()),
        Err(e) => Err(e),
    }
}

impl FetchItem for Cpu {
    fn name(&self) -> String {
        String::from("CPU")
    }
}
