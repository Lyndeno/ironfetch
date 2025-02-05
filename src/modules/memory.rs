use crate::fetch::{Fetch, Line};
use crate::{Result, GIBIBYTE, KIBIBYTE};
use memdev::memory::Memory as MemoryDevices;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::path::Path;
use sys_info::MemInfo;
use udev::Device;

#[derive(Serialize, Deserialize, Clone)]
pub struct MemStats {
    total: u64,
    free: u64,
    avail: u64,
    buffers: u64,
    cached: u64,
    swap_total: u64,
    swap_free: u64,
}

impl From<MemInfo> for MemStats {
    fn from(value: MemInfo) -> Self {
        Self {
            total: value.total * KIBIBYTE,
            free: value.free * KIBIBYTE,
            avail: value.avail * KIBIBYTE,
            buffers: value.buffers * KIBIBYTE,
            cached: value.cached * KIBIBYTE,
            swap_total: value.swap_total * KIBIBYTE,
            swap_free: value.swap_free * KIBIBYTE,
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Memory {
    pub meminfo: MemStats,
    pub devices: Option<MemoryDevices>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MemDevice {
    properties: HashMap<String, String>,
}

impl Memory {
    /// Return a new memory object.
    /// # Errors
    ///
    /// Will return an error if the memory stats cannot be parsed.
    /// Does not error on failure to obtain smbios information
    pub fn new() -> Result<Self> {
        let meminfo = MemStats::from(sys_info::mem_info()?);

        let udev = Device::from_syspath(Path::new("/sys/devices/virtual/dmi/id"))?;
        let devices = MemoryDevices::try_from(udev).ok();

        Ok(Self { meminfo, devices })
    }

    pub fn used(&self) -> u64 {
        self.meminfo.total - self.meminfo.avail
    }

    pub fn swap_used(&self) -> u64 {
        self.meminfo.swap_total - self.meminfo.swap_free
    }

    pub fn display(&self) -> String {
        self.display_unit(
            self.used() as f64 / GIBIBYTE as f64,
            self.meminfo.total as f64 / GIBIBYTE as f64,
            "GiB",
        )
    }

    pub fn display_swap(&self) -> String {
        display_mem_unit(
            self.swap_used() as f64 / GIBIBYTE as f64,
            self.meminfo.swap_total as f64 / GIBIBYTE as f64,
            "GiB",
        )
    }
    fn get_type(&self) -> Vec<String> {
        let mut memtype = Vec::new();
        if let Some(v) = &self.devices {
            for dev in v.devices.clone() {
                memtype.push(dev.mem_type);
            }
        }

        let mut string_vec: Vec<String> = memtype
            .iter()
            .map(std::string::ToString::to_string)
            .collect();

        let set: HashSet<_> = string_vec.drain(..).collect();
        string_vec.extend(set);

        string_vec
    }

    fn get_formfactor(&self) -> Vec<String> {
        let mut memff = Vec::new();
        if let Some(v) = &self.devices {
            for dev in v.devices.clone() {
                if let Some(x) = dev.form_factor {
                    memff.push(x);
                }
            }
        }

        let mut string_vec: Vec<String> =
            memff.iter().map(std::string::ToString::to_string).collect();

        let set: HashSet<_> = string_vec.drain(..).collect();
        string_vec.extend(set);

        string_vec
    }

    fn display_unit(&self, used: f64, total: f64, unit: &str) -> String {
        let typestring = print_strings(self.get_type());
        let avg_freq_opt = self.devices.clone().map(|x| x.avg_frequency());
        let formfactors = print_strings(self.get_formfactor());

        let mut s = String::new();
        s.push_str(&display_mem_unit(used, total, unit));

        if let Some(v) = typestring {
            s.push(' ');
            s.push_str(&v);
        }

        if let Some(v) = formfactors {
            s.push_str(&format!(" ({v})"));
        }

        let avg_freq = avg_freq_opt.unwrap_or(0);

        if avg_freq > 0 {
            s.push_str(&format!(" @ {avg_freq} MHz"));
        }
        s
    }
}

fn display_mem_unit(used: f64, total: f64, unit: &str) -> String {
    format!("{used:.2}{unit} / {total:.2}{unit}")
}

impl std::fmt::Display for Memory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.display())
    }
}

impl From<Memory> for Vec<Line> {
    fn from(value: Memory) -> Self {
        [
            ("Memory", &value).into(),
            ("Swap", value.display_swap()).into(),
        ]
        .into()
    }
}

impl Fetch for Memory {
    fn name(&self) -> &'static str {
        "Memory"
    }
    fn as_fetchlines(&self) -> Vec<Line> {
        self.clone().into()
    }
}

fn print_strings(strings: Vec<String>) -> Option<String> {
    if strings.is_empty() {
        None
    } else {
        let mut list = String::new();

        let mut typeiter = strings.into_iter();

        if let Some(x) = typeiter.next() {
            list.push_str(&x);
            for y in typeiter {
                list.push_str(", ");
                list.push_str(&y);
            }
        }
        Some(list)
    }
}

#[cfg(test)]
mod tests {
    use crate::machine::Machine;

    use super::*;

    #[test]
    #[allow(clippy::unwrap_used)]
    fn test_display() {
        let devices = Machine::from_file("./machine.json".into())
            .unwrap()
            .memory
            .unwrap()
            .devices;
        let mem = Memory {
            devices,
            meminfo: MemStats {
                total: 0,
                free: 0,
                avail: 0,
                buffers: 0,
                cached: 0,
                swap_total: 0,
                swap_free: 0,
            },
        };

        let display = mem.to_string();
        let desired = "0.00GiB / 0.00GiB DDR4 (SODIMM) @ 2667 MHz";
        assert_eq!(&display, desired);
    }
}
