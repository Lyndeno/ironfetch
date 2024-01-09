use crate::fetcherror::FetchError;
use measurements::frequency::Frequency;
use procfs::prelude::*;
use procfs::CpuInfo;

pub struct Cpu {
    cpu: CpuInfo,
}

impl Cpu {
    /// Get current CPU information
    ///
    /// # Errors
    ///
    /// Returns and error if the cpu info cannot be obtained.
    pub fn new() -> Result<Self, FetchError> {
        let cpu = CpuInfo::current()?;

        Ok(Self { cpu })
    }

    pub fn logical_core_count(&self) -> usize {
        self.cpu.num_cores()
    }

    pub fn frequency(&self, cpu_num: usize) -> Frequency {
        Frequency::from_megahertz(
            self.cpu
                .get_field(cpu_num, "cpu MHz")
                .unwrap_or("0.00") // FIXME: I really do not like this
                .parse::<f64>()
                .unwrap_or(0.00),
        )
    }

    #[allow(clippy::cast_precision_loss)]
    pub fn frequency_avg(&self) -> Frequency {
        let mut sum = Frequency::from_hertz(0_f64);
        for cpu_num in 0..self.logical_core_count() {
            sum = sum + self.frequency(cpu_num);
        }
        sum / self.logical_core_count() as f64
    }

    pub fn physical_core_count(&self) -> Option<usize> {
        let mut core_id = Vec::new();
        for cpu_num in 0..self.logical_core_count() {
            let id = self.cpu.get_field(cpu_num, "core id")?.parse::<usize>();
            if let Ok(v) = id {
                core_id.push(v);
            } else {
                return None;
            }
        }
        core_id.sort();
        core_id.dedup();
        Some(core_id.len())
    }

    pub fn model(&self) -> String {
        // TODO: Implement support for multiple CPU models, technically possible
        let string = self
            .cpu
            .model_name(0)
            .unwrap_or("Unknown Model")
            .to_string();
        let strings: Vec<&str> = string.split('@').collect();
        strings[0].trim().to_owned()
    }
}

impl std::fmt::Display for Cpu {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let core_string = if let Some(v) = self.physical_core_count() {
            format!("({}/{})", self.logical_core_count(), v)
        } else {
            format!("({})", self.logical_core_count())
        };
        write!(
            f,
            "{} {} @ {:.3}",
            self.model(),
            core_string,
            self.frequency_avg()
        )
    }
}
