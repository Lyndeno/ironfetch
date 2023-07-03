use std::fmt::Write;

use crate::fetcherror::FetchError;
use crate::fetchitem::FetchItem;
use crate::memunit::MemUnits;
use crate::proc::proc_parse;

use measurements::data::Data;

#[derive(Copy, Clone)]
pub struct MemBytes(u64);
pub struct Memory {
    total: Data,
    //free: MemBytes,
    avail: Data,
    //buffers: MemBytes,
    //cached: MemBytes,
    //swap_total: MemBytes,
    //swap_free: MemBytes,
    display_unit: MemUnits,
}

impl Memory {
    pub fn new(unit: Option<MemUnits>) -> Result<Self, FetchError> {
        let path = "/proc/meminfo";
        Ok(Self {
            total: mem_from_proc(proc_parse(path, "MemTotal")?),
            avail: mem_from_proc(proc_parse(path, "MemAvailable")?),
            display_unit: unit.unwrap_or(MemUnits::GB),
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
            MemUnits::GB => self.display_gb(),
            MemUnits::MB => self.display_mb(),
        }
    }
}

fn mem_from_proc(line: String) -> Data {
    Data::from_kibioctets(line.replace("kB", "").trim().parse::<f64>().unwrap())
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
