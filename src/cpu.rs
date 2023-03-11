use sys_info::{cpu_num,cpu_speed,Error};

pub struct Cpu {
    core_count: u32,
    speed: u64,
}

impl Cpu {
    pub fn new() -> Result<Self, Error> {
        Ok(Self {
            core_count: cpu_num()?,
            speed: cpu_speed()?
        })
    }
}

impl std::fmt::Display for Cpu {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({}) @ {:.3}GHz", "MODEL", self.core_count, self.speed as f64 / 1000 as f64)
    }
}