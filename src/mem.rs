use std::fmt::Write;

use crate::fetcherror::FetchError;
use crate::fetchitem::FetchItem;
use crate::memunit::MemUnits;
use procfs::Meminfo;

use measurements::data::Data;
pub struct Memory {
    total: Data,
    //free: MemBytes,
    avail: Data,
    //buffers: MemBytes,
    //cached: MemBytes,
    //swap_total: MemBytes,
    //swap_free: MemBytes,
    display_unit: Option<MemUnits>,
}

impl Memory {
    pub fn new(display_unit: Option<MemUnits>) -> Result<Self, FetchError> {
        let meminfo = Meminfo::new().unwrap();
        Ok(Self {
            total: Data::from_octets(meminfo.mem_total as f64),
            avail: Data::from_octets(meminfo.mem_available.unwrap() as f64),
            display_unit,
        })
    }

    pub fn used(&self) -> Data {
        self.total - self.avail
    }

    pub fn display_gb(&self) -> String {
        self.display_unit(
            self.used().as_gibioctets(),
            self.total.as_gibioctets(),
            "GiB",
        )
    }
    pub fn display_mb(&self) -> String {
        self.display_unit(
            self.used().as_mebioctets(),
            self.total.as_mebioctets(),
            "MiB",
        )
    }

    fn display_unit(&self, used: f64, total: f64, unit: &str) -> String {
        let mut s = String::new();
        write!(s, "{:.2}{} / {:.2}{}", used, unit, total, unit).unwrap();
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

impl std::fmt::Display for Memory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.display())
    }
}

impl FetchItem for Memory {
    fn name(&self) -> String {
        String::from("Memory")
    }
}
