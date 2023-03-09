use sys_info::{cpu_num,cpu_speed};

use crate::fetchline::Fetchline;

pub struct Cpu {
    core_count: Option<u32>,
}

impl Cpu {
    pub fn new() -> Self {
        Self {
            core_count: Some(cpu_num().unwrap()),
        }
    }
}

impl std::fmt::Display for Cpu {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.gen_fetchline(Vec::from([self.core_count.as_ref()])))
    }
}

impl Fetchline for Cpu {}