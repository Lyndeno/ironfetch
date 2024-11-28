use std::collections::{HashMap, HashSet};

use std::path::Path;
use udev::{Device, Entry};

use crate::fetcherror::FetchError;
use crate::memunit::MemUnits;
use measurements::Data;

use std::str::FromStr;

use sys_info::MemInfo;

pub struct Memory {
    display_unit: Option<MemUnits>,
    meminfo: MemInfo,
    devices: Option<Vec<MemDevice>>,
}

#[derive(Debug)]
pub struct MemDevice {
    properties: HashMap<String, String>,
}

impl MemDevice {
    pub fn new(index: usize) -> Result<Self, FetchError> {
        let udev = Device::from_syspath(Path::new("/sys/devices/virtual/dmi/id"))?;
        let props = udev.properties();
        let props_vec: Vec<Entry<'_>> = props.collect();

        let mut propmap = HashMap::new();

        for prop in props_vec.iter() {
            if let Some(clean_name) = prop
                .name()
                .to_string_lossy()
                .to_string()
                .strip_prefix(&format!("MEMORY_DEVICE_{}_", index))
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
    pub fn new(display_unit: Option<MemUnits>) -> Result<Self, FetchError> {
        let meminfo = sys_info::mem_info()?;

        let udev = Device::from_syspath(Path::new("/sys/devices/virtual/dmi/id"))?;
        let mut props = udev.properties();

        let count_entry = props
            .find(|x| {
                x.name()
                    .to_string_lossy()
                    .contains("MEMORY_ARRAY_NUM_DEVICE")
            })
            .ok_or(FetchError::OsStr)?;

        let count = str::parse::<usize>(&count_entry.value().to_string_lossy())?;

        let mut devs = Vec::with_capacity(count);

        for i in 0..count {
            devs.push(MemDevice::new(i)?);
        }

        Ok(Self {
            display_unit,
            meminfo,
            // This will usually error do to permission errors, so just wrap it None instead
            // as it is not needed for basic use
            devices: Some(devs),
        })
    }

    pub fn used(&self) -> Data {
        self.total() - self.available()
    }

    #[allow(clippy::cast_precision_loss)]
    pub fn total(&self) -> Data {
        Data::from_kibioctets(self.meminfo.total as f64)
    }

    #[allow(clippy::cast_precision_loss)]
    pub fn available(&self) -> Data {
        Data::from_kibioctets(self.meminfo.avail as f64)
    }

    pub fn display_gb(&self) -> String {
        self.display_unit(
            self.used().as_gibioctets(),
            self.total().as_gibioctets(),
            "GiB",
        )
    }
    pub fn display_mb(&self) -> String {
        self.display_unit(
            self.used().as_mebioctets(),
            self.total().as_mebioctets(),
            "MiB",
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
        s.push_str(&format!("{used:.2}{unit} / {total:.2}{unit}"));

        if let Some(v) = typestring {
            s.push(' ');
            s.push_str(&v);
        }

        if let Some(v) = formfactors {
            s.push_str(&format!(" ({v})"));
        }

        if avg_freq > 0 {
            s.push_str(&format!(" @ {} MHz", avg_freq));
        }
        s
    }

    fn display(&self) -> String {
        match self.display_unit {
            None | Some(MemUnits::GiB) => self.display_gb(),
            Some(MemUnits::MiB) => self.display_mb(),
        }
    }
}

impl std::fmt::Display for Memory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.display())
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

fn sum_frequency(f: Vec<usize>) -> usize {
    let mut sum = 0;
    for freq in f {
        sum = sum + freq;
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
    use super::*;

    #[test]
    fn test_display() {
        let device1 = MemDevice {
            properties: HashMap::from(
                [
                    ("FORM_FACTOR", "SODIMM"),
                    ("TYPE_DETAIL", "Synchronous Unbuffered (Unregistered)"),
                    ("TYPE", "DDR4"),
                    ("PART_NUMBER", "TIMETEC-SD4-2666"),
                    ("ASSET_TAG", "00221200"),
                    ("CONFIGURED_VOLTAGE", "1"),
                    ("CONFIGURED_SPEED_MTS", "2667"),
                    ("BANK_LOCATOR", "BANK 0"),
                    ("RANK", "2"),
                    ("DATA_WIDTH", "64"),
                    ("SIZE", "34359738368"),
                    ("MINIMUM_VOLTAGE", "1"),
                    ("SPEED_MTS", "2667"),
                    ("LOCATOR", "DIMM A"),
                    ("TOTAL_WIDTH", "64"),
                    ("MAXIMUM_VOLTAGE", "1"),
                    ("SERIAL_NUMBER", "00000000"),
                    ("MANUFACTURER", "8C260000802C"),
                ]
                .map(|x| (x.0.to_string(), x.1.to_string())),
            ),
        };
        let device2 = MemDevice {
            properties: HashMap::from(
                [
                    ("MANUFACTURER", "8C260000802C"),
                    ("TYPE", "DDR4"),
                    ("BANK_LOCATOR", "BANK 2"),
                    ("CONFIGURED_VOLTAGE", "1"),
                    ("RANK", "2"),
                    ("FORM_FACTOR", "SODIMM"),
                    ("LOCATOR", "DIMM B"),
                    ("MAXIMUM_VOLTAGE", "1"),
                    ("TYPE_DETAIL", "Synchronous Unbuffered (Unregistered)"),
                    ("MINIMUM_VOLTAGE", "1"),
                    ("CONFIGURED_SPEED_MTS", "2667"),
                    ("SIZE", "34359738368"),
                    ("DATA_WIDTH", "64"),
                    ("SPEED_MTS", "2667"),
                    ("ASSET_TAG", "00221200"),
                    ("TOTAL_WIDTH", "64"),
                    ("PART_NUMBER", "TIMETEC-SD4-2666"),
                    ("SERIAL_NUMBER", "00000000"),
                ]
                .map(|x| (x.0.to_string(), x.1.to_string())),
            ),
        };
        let mem = Memory {
            devices: Some(vec![device1, device2]),
            display_unit: None,
            meminfo: MemInfo {
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
