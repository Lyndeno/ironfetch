use std::collections::HashSet;

use crate::fetcherror::FetchError;
use crate::memunit::MemUnits;
use measurements::{Data, Frequency, Measurement};

use simplesmbios::mem::MemDevice;
use simplesmbios::smbios::SMBios;
use sys_info::MemInfo;

pub struct Memory<'a> {
    display_unit: Option<MemUnits>,
    meminfo: MemInfo,
    devices: Option<Vec<MemDevice<'a>>>,
}

impl<'a> Memory<'a> {
    /// Return a new memory object.
    /// # Errors
    ///
    /// Will return an error if the memory stats cannot be parsed.
    /// Does not error on failure to obtain smbios information
    pub fn new(
        display_unit: Option<MemUnits>,
        smbios: Option<&'a SMBios>,
    ) -> Result<Self, FetchError> {
        let meminfo = sys_info::mem_info()?;

        Ok(Self {
            display_unit,
            meminfo,
            // This will usually error do to permission errors, so just wrap it None instead
            // as it is not needed for basic use
            devices: match smbios {
                Some(s) => MemDevice::from_smbios(s).unwrap_or(None),
                None => None,
            },
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
                if let Some(x) = dev.mem_type() {
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

    fn get_speed(&self) -> Vec<Frequency> {
        let mut speeds = Vec::new();
        if let Some(v) = &self.devices {
            for dev in v {
                if let Some(x) = dev.speed() {
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

        if avg_freq > Frequency::from_base_units(0_f64) {
            s.push_str(&format!(" @ {} MHz", avg_freq.as_megahertz()));
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

impl std::fmt::Display for Memory<'_> {
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

fn sum_frequency(f: Vec<Frequency>) -> Frequency {
    let mut sum = Frequency::from_hertz(0_f64);
    for freq in f {
        sum = sum + freq;
    }
    sum
}

#[allow(clippy::cast_precision_loss)]
fn avg_frequency(f: Vec<Frequency>) -> Frequency {
    let count = f.len();
    sum_frequency(f) / count as f64
}
