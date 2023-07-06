use crate::{fetcherror::FetchError, fetchitem::FetchItem, FetchSection};
use measurements::frequency::Frequency;
use procfs::CpuInfo;

pub struct Cpu {
    core_count: usize,
    model: String,
    freq: Frequency,
}

impl Cpu {
    pub fn new() -> Result<Self, FetchError> {
        let cpu = CpuInfo::new().unwrap();
        let core_count = cpu.num_cores();

        // TODO: Implement support for multiple CPU models, technically possible
        let model = cpu.model_name(0).unwrap_or("Unknown Model").to_string();

        let mut sum = Frequency::from_hertz(0f64);
        for cpu_num in 0..core_count {
            sum = sum
                + Frequency::from_megahertz(
                    cpu.get_field(cpu_num, "cpu MHz")
                        .unwrap_or("0.00") // FIXME: I really do not like this
                        .parse::<f64>()
                        .unwrap(),
                );
        }
        let freq = sum / core_count as f64;

        Ok(Self {
            core_count,
            model,
            freq,
        })
    }
}

impl std::fmt::Display for Cpu {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({}) @ {:.3}", self.model, self.core_count, self.freq)
    }
}

impl FetchItem for Cpu {
    fn name(&self) -> String {
        String::from("CPU")
    }

    fn long_content(&self) -> Option<Vec<crate::FetchSection>> {
        Some(vec![
            FetchSection::new_short("Model", self.model.clone()),
            FetchSection::new_short("Cores", format!("{}", self.core_count)),
            FetchSection::new_short("Frequency", format!("{:.3}", self.freq)),
        ])
    }
}
