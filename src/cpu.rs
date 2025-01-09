use std::collections::HashSet;

use measurements::frequency::Frequency;
use procfs::prelude::*;
use procfs::CpuInfo;

use serde::{Deserialize, Serialize};

use crate::fetchsection::FetchSection;
use crate::Error;
use crate::Result;

#[derive(Serialize, Deserialize, Clone)]
pub struct Cpu {
    cores: Vec<Core>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Core {
    frequency: Frequency,
    id: Option<usize>,
    model: Option<String>,
}

impl Cpu {
    /// Get current CPU information
    ///
    /// # Errors
    ///
    /// Returns and error if the cpu info cannot be obtained.
    pub fn new() -> Result<Self> {
        let cpu = CpuInfo::current()?;
        let mut cores = Vec::new();

        for i in 0..cpu.num_cores() {
            let core = Core {
                frequency: Frequency::from_megahertz(
                    cpu.get_field(i, "cpu MHz")
                        .unwrap_or("0.00")
                        .parse::<f64>()
                        .unwrap_or(0_f64),
                ),
                id: cpu
                    .get_field(i, "core id")
                    .and_then(|x| x.parse::<usize>().ok()),
                model: cpu.model_name(i).map(Into::into),
            };
            cores.push(core);
        }

        Ok(Self { cores })
    }

    pub fn logical_core_count(&self) -> usize {
        self.cores.len()
    }

    #[allow(clippy::cast_precision_loss)]
    pub fn frequency_avg(&self) -> Frequency {
        let mut sum = Frequency::from_hertz(0_f64);
        for core in &self.cores {
            sum = sum + core.frequency;
        }
        sum / self.logical_core_count() as f64
    }

    pub fn physical_core_count(&self) -> Option<usize> {
        let mut core_id = HashSet::new();
        for core in &self.cores {
            if let Some(v) = core.id {
                core_id.insert(v);
            } else {
                return None;
            }
        }
        Some(core_id.len())
    }

    pub fn model(&self) -> String {
        // TODO: Implement support for multiple CPU models, technically possible
        let string = self.cores[0].model.as_deref().unwrap_or("Unknown Model");
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

impl From<Cpu> for FetchSection {
    fn from(value: Cpu) -> Self {
        ("CPU", value).into()
    }
}
