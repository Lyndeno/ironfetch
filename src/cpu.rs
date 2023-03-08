use sys_info::{cpu_num,cpu_speed};

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
        let mut s = String::new();
        if let Some(v) = &self.core_count {
            s = String::from(v.to_string());
        };
        write!(f, "{}", s)
    }
}