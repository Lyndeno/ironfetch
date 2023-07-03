use crate::{fetcherror::FetchError, fetchitem::FetchItem, proc::proc_parse_try};
use measurements::frequency::Frequency;
use sys_info::{cpu_num, cpu_speed};

pub struct Cpu {
    core_count: u32,
    speed: Frequency,
    model: String,
}

impl Cpu {
    pub fn new() -> Result<Self, FetchError> {
        Ok(Self {
            core_count: cpu_num()?,
            speed: Frequency::from_megahertz(cpu_speed()? as f64),
            model: read_cpu_model()?,
        })
    }
}

impl std::fmt::Display for Cpu {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} ({}) @ {:.3}",
            self.model, self.core_count, self.speed
        )
    }
}

fn read_cpu_model() -> Result<String, FetchError> {
    match proc_parse_try("/proc/cpuinfo", &["model name", "Hardware"]) {
        Ok(v) => Ok(v),
        Err(FetchError::Proc) => Ok("N/A".to_string()),
        Err(e) => Err(e),
    }
}

impl FetchItem for Cpu {
    fn name(&self) -> String {
        String::from("CPU")
    }
}
