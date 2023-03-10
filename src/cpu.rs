use sys_info::{cpu_num,cpu_speed};

pub struct Cpu {
    core_count: Option<u32>,
}

impl Cpu {
    pub fn new() -> Self {
        Self {
            core_count: match cpu_num() {
                Ok(c) => Some(c),
                Err(_) => None,
            },
        }
    }
}

impl std::fmt::Display for Cpu {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();
        match self.core_count {
            Some(c) => s.push_str(c.to_string().as_str()),
            None => {},
        };
        write!(f, "Cores: {}", s)
    }
}