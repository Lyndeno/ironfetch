use crate::fetch::{AsLine, AsLines, IntoFetch, Line};
use crate::{Error, Result};
use measurements::Data;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::path::Path;
use std::str::FromStr;
use sys_info::MemInfo;
use udev::{Device, Entry};

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
            total: value.total,
            free: value.free,
            avail: value.avail,
            buffers: value.buffers,
            cached: value.cached,
            swap_total: value.swap_total,
            swap_free: value.swap_free,
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Memory {
    pub meminfo: MemStats,
    pub devices: Option<Vec<MemDevice>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MemDevice {
    properties: HashMap<String, String>,
}

impl MemDevice {
    /// Returns new `MemDevice`
    ///
    /// # Errors
    /// Returns error if getting memory information fails
    pub fn new(index: usize) -> Result<Self> {
        let udev = Device::from_syspath(Path::new("/sys/devices/virtual/dmi/id"))?;
        let props = udev.properties();
        let props_vec: Vec<Entry<'_>> = props.collect();

        let mut propmap = HashMap::new();

        for prop in &props_vec {
            if let Some(clean_name) = prop
                .name()
                .to_string_lossy()
                .to_string()
                .strip_prefix(&format!("MEMORY_DEVICE_{index}_"))
            {
                propmap.insert(
                    clean_name.to_string(),
                    prop.value().to_string_lossy().into_owned(),
                );
            }
        }

        Ok(MemDevice {
            properties: propmap,
        })
    }
    pub fn frequency(&self) -> Option<usize> {
        self.pull_value("CONFIGURED_SPEED_MTS")
    }

    pub fn manufactuer(&self) -> Option<String> {
        self.pull_value("MANUFACTURER")
    }

    pub fn form_factor(&self) -> Option<String> {
        self.pull_value("FORM_FACTOR")
    }

    pub fn get_type(&self) -> Option<String> {
        self.pull_value("TYPE")
    }

    fn pull_value<T: FromStr>(&self, name: &str) -> Option<T> {
        if let Some(v) = self.properties.get(name) {
            return str::parse::<T>(v).ok();
        }
        None
    }
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
        let mut props = udev.properties();

        let count_entry = props
            .find(|x| {
                x.name()
                    .to_string_lossy()
                    .contains("MEMORY_ARRAY_NUM_DEVICE")
            })
            .ok_or(Error::OsStr)?;

        let count = str::parse::<usize>(&count_entry.value().to_string_lossy())?;

        let mut devs = Vec::with_capacity(count);

        for i in 0..count {
            devs.push(MemDevice::new(i)?);
        }

        Ok(Self {
            meminfo,
            // This will usually error do to permission errors, so just wrap it None instead
            // as it is not needed for basic use
            devices: Some(devs),
        })
    }

    pub fn used(&self) -> Data {
        self.total() - self.available()
    }

    pub fn swap_used(&self) -> Data {
        self.swap_total() - self.swap_free()
    }

    #[allow(clippy::cast_precision_loss)]
    pub fn total(&self) -> Data {
        Data::from_kibioctets(self.meminfo.total as f64)
    }

    #[allow(clippy::cast_precision_loss)]
    pub fn available(&self) -> Data {
        Data::from_kibioctets(self.meminfo.avail as f64)
    }

    #[allow(clippy::cast_precision_loss)]
    pub fn swap_total(&self) -> Data {
        Data::from_kibioctets(self.meminfo.swap_total as f64)
    }

    #[allow(clippy::cast_precision_loss)]
    pub fn swap_free(&self) -> Data {
        Data::from_kibioctets(self.meminfo.swap_free as f64)
    }

    pub fn display(&self) -> String {
        self.display_unit(
            self.used().as_gibioctets(),
            self.total().as_gibioctets(),
            "GiB",
        )
    }

    pub fn display_swap(&self) -> String {
        display_mem_unit(
            self.swap_used().as_gibioctets(),
            self.swap_total().as_gibioctets(),
            "GiB",
        )
    }
    fn get_type(&self) -> Vec<String> {
        let mut memtype = Vec::new();
        if let Some(v) = &self.devices {
            for dev in v {
                if let Some(x) = dev.get_type() {
                    memtype.push(x);
                }
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
            for dev in v {
                if let Some(x) = dev.form_factor() {
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

    fn get_speed(&self) -> Vec<usize> {
        let mut speeds = Vec::new();
        if let Some(v) = &self.devices {
            for dev in v {
                if let Some(x) = dev.frequency() {
                    speeds.push(x);
                }
            }
        }
        speeds
    }

    fn display_unit(&self, used: f64, total: f64, unit: &str) -> String {
        let typestring = print_strings(self.get_type());
        let avg_freq = avg_frequency(self.get_speed());
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

impl AsLine for Memory {
    const NAME: &'static str = "Memory";
}

impl AsLines for Memory {
    fn as_asfetchlines(&self) -> Vec<Line> {
        self.clone().into()
    }
}

impl IntoFetch for Memory {}

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

fn sum_frequency(f: Vec<usize>) -> usize {
    let mut sum = 0;
    for freq in f {
        sum += freq;
    }
    sum
}

#[allow(clippy::cast_precision_loss)]
fn avg_frequency(f: Vec<usize>) -> usize {
    let count = f.len();
    if count > 0 {
        sum_frequency(f) / count
    } else {
        0
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
        let desired = "0.00GiB / 0.00GiB DDR4 (DIMM) @ 3600 MHz";
        assert_eq!(&display, desired);
    }
}
