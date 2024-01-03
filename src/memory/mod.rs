use std::collections::HashSet;
use std::fmt::Write;

use crate::fetcherror::FetchError;
use crate::fetchitem::FetchItem;
use crate::memunit::MemUnits;
use measurements::Data;

use simplesmbios::mem::MemDevice;
use simplesmbios::smbios::SMBios;
use sys_info::MemInfo;

pub struct Memory<'a> {
    display_unit: Option<MemUnits>,
    meminfo: MemInfo,
    devices: Option<Vec<MemDevice<'a>>>,
}

impl<'a> Memory<'a> {
    pub fn new(
        display_unit: Option<MemUnits>,
        smbios: Option<&'a SMBios>,
    ) -> Result<Self, FetchError> {
        let meminfo = sys_info::mem_info().unwrap();

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

    pub fn total(&self) -> Data {
        Data::from_kibioctets(self.meminfo.total as f64)
    }

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
            let mut iter = v.iter();

            for dev in iter {
                if let Some(x) = dev.mem_type() {
                    memtype.push(x);
                }
            }
        }
        memtype.iter().map(|x| x.to_string()).collect()
    }

    fn display_unit(&self, used: f64, total: f64, unit: &str) -> String {
        let types = self.get_type();
        let mut typestring = String::new();

        let mut typeiter = types.iter();

        if let Some(x) = typeiter.next() {
            typestring.push_str(x);
            for y in typeiter {
                typestring.push_str(", ");
                typestring.push_str(y);
            }
        }
        let mut s = String::new();
        write!(
            s,
            "{:.2}{} / {:.2}{} {}",
            used, unit, total, unit, typestring
        )
        .unwrap();
        s
    }

    fn display(&self) -> String {
        match self.display_unit {
            Some(MemUnits::GiB) => self.display_gb(),
            Some(MemUnits::MiB) => self.display_mb(),
            None => self.display_gb(),
        }
    }
}

impl std::fmt::Display for Memory<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.display())
    }
}

impl FetchItem for Memory<'_> {
    fn name(&self) -> String {
        String::from("Memory")
    }
}
