use std::ops::{Add, Sub};

use std::fmt::Write;

use crate::fetchitem::FetchItem;
use crate::proc::proc_parse;

#[derive(Copy, Clone)]
pub struct MemBytes(u64);
pub struct Memory {
    total: MemBytes,
    //free: MemBytes,
    avail: MemBytes,
    //buffers: MemBytes,
    //cached: MemBytes,
    //swap_total: MemBytes,
    //swap_free: MemBytes,
}

impl Memory {
    pub fn new() -> Self {
        let path = "/proc/meminfo";
        Self {
            total: MemBytes::from_proc(proc_parse(path, "MemTotal").unwrap().unwrap()),
            avail: MemBytes::from_proc(proc_parse(path, "MemAvailable").unwrap().unwrap()),
        }
    }

    pub fn used(&self) -> MemBytes {
        self.total - self.avail
    }

    pub fn display_gb(&self) -> String {
        self.display(self.used().as_gb(), self.total.as_gb(), "GiB")
    }
    pub fn display_mb(&self) -> String {
        self.display(self.used().as_mb(), self.total.as_mb(), "MiB")
    }

    fn display(&self, used: f64, total: f64, unit: &str) -> String {
        let mut s = String::new();
        write!(s, "{:.2}{} / {:.2}{}", used, unit, total, unit).unwrap();
        s
    }
}

impl MemBytes {
    fn as_gb(&self) -> f64 {
        self.as_mb() / (1024_f64)
    }
    fn as_mb(&self) -> f64 {
        (self.0 as f64) / (1024f64)
    }
    fn from_proc(line: String) -> Self {
        Self::from(line.replace("kB", "").trim().parse::<u64>().unwrap())
    }
}

impl Add for MemBytes {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self(self.0 + rhs.0)
    }
}

impl Sub for MemBytes {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self(self.0 - rhs.0)
    }
}

impl From<u64> for MemBytes {
    fn from(m: u64) -> MemBytes {
        MemBytes(m)
    }
}

impl std::fmt::Display for Memory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.display_gb())
    }
}

impl FetchItem for Memory {
    fn name(&self) -> String {
        String::from("Memory")
    }
}

#[cfg(test)]
mod tests {
    use super::MemBytes;

    #[test]
    fn byte_math() {
        let m = MemBytes(1048576);
        assert_eq!(m.as_mb() / 1024_f64, m.as_gb());
    }
}
