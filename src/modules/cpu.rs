use std::collections::HashMap;
use std::collections::HashSet;

use procfs::prelude::*;
use procfs::CpuInfo;

use serde::{Deserialize, Serialize};

use crate::fetch::Fetch;
use crate::Result;

#[derive(Serialize, Deserialize, Clone, Fetch)]
#[fetch(name = "CPU")]
pub struct Cpu {
    cores: Vec<Core>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Core {
    frequency: Option<f64>,
    id: Option<usize>,
    model: Option<String>,
}

impl From<HashMap<&str, &str>> for Core {
    fn from(value: HashMap<&str, &str>) -> Self {
        Self {
            frequency: if let Some(v) = value.get("cpu MHz") {
                v.parse().ok()
            } else {
                None
            },
            id: value.get("core id").and_then(|x| x.parse().ok()),
            model: value.get("model name").map(ToString::to_string),
        }
    }
}

impl Cpu {
    /// Get current CPU information
    ///
    /// # Errors
    ///
    /// Returns and error if the cpu info cannot be obtained.
    pub fn new() -> Result<Self> {
        Ok(CpuInfo::current()?.into())
    }

    pub fn logical_core_count(&self) -> usize {
        self.cores.len()
    }

    pub fn frequency_avg(&self) -> Option<f64> {
        let mut sum = 0_f64;
        let mut count = 0;
        for core in &self.cores {
            if let Some(f) = core.frequency {
                count += 1;
                sum += f;
            }
        }
        if count > 0 {
            Some(sum / f64::from(count))
        } else {
            None
        }
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

impl From<CpuInfo> for Cpu {
    fn from(value: CpuInfo) -> Self {
        let mut cores = Vec::new();
        for i in 0..value.num_cores() {
            if let Some(v) = value.get_info(i) {
                cores.push(Core::from(v));
            }
        }
        Self { cores }
    }
}

impl std::fmt::Display for Cpu {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let core_string = if let Some(v) = self.physical_core_count() {
            format!("({}/{})", self.logical_core_count(), v)
        } else {
            format!("({})", self.logical_core_count())
        };
        let freq_string = if let Some(f) = self.frequency_avg() {
            format!(" @ {:.3} GHz", f / 1000_f64)
        } else {
            String::new()
        };
        write!(f, "{} {}{}", self.model(), core_string, freq_string)
    }
}
