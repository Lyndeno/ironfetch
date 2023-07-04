use crate::{fetcherror::FetchError, fetchitem::FetchItem};
use measurements::frequency::Frequency;
use procfs::CpuInfo;

pub struct Cpu {
    cpu: CpuInfo,
}

impl Cpu {
    pub fn new() -> Result<Self, FetchError> {
        Ok(Self {
            cpu: CpuInfo::new().unwrap(),
        })
    }

    pub fn core_count(&self) -> usize {
        self.cpu.num_cores()
    }

    pub fn model(&self) -> String {
        self.cpu.model_name(0).unwrap_or("").to_string()
    }

    pub fn speed(&self) -> Frequency {
        let mut sum = Frequency::from_hertz(0f64);
        for core in 0..self.core_count() {
            sum = sum + self.core_speed(core);
        }
        sum / self.core_count() as f64
    }

    fn core_speed(&self, cpu_num: usize) -> Frequency {
        Frequency::from_megahertz(
            self.cpu
                .get_field(cpu_num, "cpu MHz")
                .unwrap()
                .parse::<f64>()
                .unwrap(),
        )
    }
}

impl std::fmt::Display for Cpu {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} ({}) @ {:.3}",
            self.model(),
            self.core_count(),
            self.speed()
        )
    }
}

impl FetchItem for Cpu {
    fn name(&self) -> String {
        String::from("CPU")
    }
}
