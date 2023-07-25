use std::fmt::Write;

use crate::fetchitem::FetchItem;
use crate::memunit::MemUnits;
use crate::{fetcherror::FetchError, fetchsection::FetchType};
use crate::{fetchsection::opt_fs, fetchsection::FetchSection};
use measurements::Data;
use procfs::Meminfo;

use simplesmbios::mem::{MemDevice, SMBiosSource};

pub struct Memory {
    total: Data,
    //free: MemBytes,
    avail: Data,
    //buffers: MemBytes,
    //cached: MemBytes,
    //swap_total: MemBytes,
    //swap_free: MemBytes,
    display_unit: Option<MemUnits>,
    devices: Option<Vec<MemDevice>>,
}

impl Memory {
    pub fn new(
        display_unit: Option<MemUnits>,
        smbios_source: SMBiosSource,
    ) -> Result<Self, FetchError> {
        let meminfo = Meminfo::new().unwrap();

        Ok(Self {
            total: Data::from_octets(meminfo.mem_total as f64),
            avail: Data::from_octets(meminfo.mem_available.unwrap() as f64),
            display_unit,
            // This will usually error do to permission errors, so just wrap it None instead
            // as it is not needed for basic use
            devices: MemDevice::from_source(smbios_source).unwrap_or(None),
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

    fn long_content(&self) -> Option<Vec<FetchSection>> {
        let mut vec: Vec<FetchSection> = vec![
            ("Total", format!("{:.2}", self.total)).into(),
            ("Used", format!("{:.2}", self.used())).into(),
            ("Available", format!("{:.2}", self.avail)).into(),
        ];
        if let Some(ref s) = self.devices {
            let mut devices = Vec::new();
            for dev in s {
                devices.push(FetchSection {
                    name: dev.location.clone(),
                    content: {
                        let mut memvec: Vec<FetchSection> = Vec::new();
                        memvec.push(opt_fs(("Manufacturer", dev.manufacturer.clone())));
                        memvec.push(opt_fs(("Part #", dev.part_number.clone())));
                        memvec.push(opt_fs(("Type", dev.mem_type.clone())));
                        if let Some(ref v) = dev.size {
                            memvec.push(("Capacity", format!("{:.2}", v)).into())
                        }
                        if let Some(ref v) = dev.speed {
                            memvec.push(("Speed", format!("{} MT/s", v.as_megahertz())).into());
                        }
                        FetchType::Long(memvec)
                    },
                });
            }
            vec.push(FetchSection {
                name: "Devices".to_string(),
                content: FetchType::Long(devices),
            });
        };
        Some(vec)
    }
}
