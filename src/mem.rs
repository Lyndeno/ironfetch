use std::ops::{Add,Sub};

use sys_info::{mem_info, MemInfo};
use std::fmt::Write;

#[derive(Copy,Clone)]
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
        Self::from(mem_info().unwrap())
    }

    pub fn used(&self) -> MemBytes {
        self.total - self.avail
    }

    pub fn display_gb(&self) -> String {
        self.display(self.used().to_gb(), self.total.to_gb(), "GiB")
    }
    pub fn display_mb(&self) -> String {
        self.display(self.used().to_mb(), self.total.to_mb(), "MiB")
    }

    fn display(&self, used: f64, total: f64, unit: &str) -> String {
        let mut s = String::new();
        write!(s, "{:.2}{} / {:.2}{}", used, unit, total, unit).unwrap();
        s
    }
}

impl From<MemInfo> for Memory {
    fn from(m: MemInfo) -> Self {
        Self {
            total: MemBytes::from(m.total),
            //free: MemBytes::from(m.free),
            avail: MemBytes::from(m.avail),
            //buffers: MemBytes::from(m.buffers),
            //cached: MemBytes::from(m.cached),
            //swap_total: MemBytes::from(m.swap_total),
            //swap_free: MemBytes::from(m.swap_free),
        }
    }
}

impl MemBytes {
    fn to_gb(&self) -> f64 {
       self.to_mb() / (1024 as f64)
    }
    fn to_mb(&self) -> f64 {
       (self.0 as f64) / (1024 as f64)
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

#[cfg(test)]
mod tests {
    use super::MemBytes;

    #[test]
    fn byte_math() {
        let m = MemBytes(1048576);
        assert_eq!(m.to_mb() / 1024 as f64, m.to_gb());
    }
}
