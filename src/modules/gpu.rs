use std::collections::HashSet;
use std::fs;
use std::path::Path;

use serde::{Deserialize, Serialize};
use udev::Enumerator;

use crate::fetch::{Fetch, Line};
use crate::{Result, GIBIBYTE};

#[derive(Serialize, Deserialize, Clone)]
pub struct GpuDevice {
    name: String,
    vram_total: Option<u64>,
    vram_used: Option<u64>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Gpu {
    devices: Vec<GpuDevice>,
}

impl Gpu {
    /// Returns all GPU devices found on the system
    ///
    /// # Errors
    /// Returns an error if the udev enumeration fails
    pub fn new() -> Result<Self> {
        let mut enumerator = Enumerator::new()?;
        enumerator.match_subsystem("drm")?;

        let mut seen = HashSet::new();
        let mut devices = Vec::new();

        for device in enumerator.scan_devices()? {
            let sysname = device.sysname().to_string_lossy();
            // Skip connectors (card0-DP-1, card0-HDMI-A-1, etc.) and non-card entries
            if !sysname.starts_with("card") || sysname.contains('-') {
                continue;
            }

            let Some(parent) = device.parent() else {
                continue;
            };

            let parent_path = parent.syspath().to_owned();
            if !seen.insert(parent_path.clone()) {
                continue;
            }

            let name = parent
                .property_value("ID_MODEL_FROM_DATABASE")
                .map(|v| v.to_string_lossy().into_owned())
                .unwrap_or_else(|| "Unknown GPU".to_owned());

            let vram_total = read_sysfs_u64(&parent_path.join("mem_info_vram_total"));
            let vram_used = read_sysfs_u64(&parent_path.join("mem_info_vram_used"));

            devices.push(GpuDevice {
                name,
                vram_total,
                vram_used,
            });
        }

        Ok(Self { devices })
    }
}

fn read_sysfs_u64(path: &Path) -> Option<u64> {
    fs::read_to_string(path).ok()?.trim().parse().ok()
}

impl std::fmt::Display for Gpu {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut iter = self.devices.iter().peekable();
        while let Some(d) = iter.next() {
            write!(f, "{d}")?;
            if iter.peek().is_some() {
                write!(f, ", ")?;
            }
        }
        Ok(())
    }
}

impl Fetch for Gpu {
    fn name(&self) -> &'static str {
        "GPU"
    }

    fn as_fetchlines(&self) -> Vec<Line> {
        if self.devices.len() == 1 {
            vec![("GPU", self.devices[0].to_string()).into()]
        } else {
            self.devices
                .iter()
                .enumerate()
                .map(|(i, d)| (format!("GPU {}", i + 1), d.to_string()).into())
                .collect()
        }
    }
}

#[allow(clippy::cast_precision_loss)]
impl std::fmt::Display for GpuDevice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)?;
        if let (Some(used), Some(total)) = (self.vram_used, self.vram_total) {
            write!(
                f,
                " ({:.2}GiB / {:.2}GiB)",
                used as f64 / GIBIBYTE as f64,
                total as f64 / GIBIBYTE as f64,
            )?;
        }
        Ok(())
    }
}
