use crate::{fetcherror::FetchError, fetchitem::FetchItem, FetchSection};
use measurements::frequency::Frequency;
use procfs::CpuInfo;

pub struct Cpu {
    core_count: usize,
    physical_core_count: usize,
    model: String,
    freq: Vec<Frequency>,
}

impl Cpu {
    pub fn new() -> Result<Self, FetchError> {
        let cpu = CpuInfo::new().unwrap();
        let core_count = cpu.num_cores();

        // TODO: Implement support for multiple CPU models, technically possible
        let model = cpu.model_name(0).unwrap_or("Unknown Model").to_string();

        let mut freq = Vec::with_capacity(core_count);
        for cpu_num in 0..core_count {
            freq.push(Frequency::from_megahertz(
                cpu.get_field(cpu_num, "cpu MHz")
                    .unwrap_or("0.00") // FIXME: I really do not like this
                    .parse::<f64>()
                    .unwrap(),
            ));
        }

        let mut core_id = Vec::new();
        for cpu_num in 0..core_count {
            core_id.push(
                cpu.get_field(cpu_num, "core id")
                    .unwrap()
                    .parse::<usize>()
                    .unwrap(),
            )
        }
        core_id.sort();
        core_id.dedup();
        let physical_core_count = core_id.len();

        Ok(Self {
            core_count,
            model,
            freq,
            physical_core_count,
        })
    }
}

impl std::fmt::Display for Cpu {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let freq_avg = {
            let mut sum = Frequency::from_hertz(0_f64);
            for cpu_num in 0..self.core_count {
                sum = sum + self.freq[cpu_num];
            }
            sum / self.core_count as f64
        };
        write!(
            f,
            "{} ({}/{}) @ {:.3}",
            self.model, self.core_count, self.physical_core_count, freq_avg
        )
    }
}

impl FetchItem for Cpu {
    fn name(&self) -> String {
        String::from("CPU")
    }

    fn long_content(&self) -> Option<Vec<crate::FetchSection>> {
        let mut freq_vec: Vec<FetchSection> = Vec::with_capacity(self.core_count);
        for i in 0..self.core_count {
            freq_vec.push((format!("Core {}", i), format!("{:.3}", self.freq[i])).into());
        }
        Some(vec![
            ("Model", self.model.clone()).into(),
            ("Logical Cores", format!("{}", self.core_count)).into(),
            ("Physical Cores", format!("{}", self.physical_core_count)).into(),
            FetchSection {
                name: "Frequency".to_string(),
                content: crate::FetchType::Long(freq_vec),
            },
        ])
    }
}
