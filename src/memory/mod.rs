use std::fmt::Write;

use crate::fetchitem::FetchItem;
use crate::memunit::MemUnits;
use crate::FetchSection;
use crate::{fetcherror::FetchError, FetchType};
use measurements::Data;
use procfs::Meminfo;

use smbioslib::{table_load_from_device, SMBiosMemoryDevice};

use self::mem::MemDevice;

mod mem;
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
    pub fn new(display_unit: Option<MemUnits>) -> Result<Self, FetchError> {
        let meminfo = Meminfo::new().unwrap();

        let mem_vec = match table_load_from_device() {
            Ok(s) => {
                let smb = s.defined_struct_iter::<SMBiosMemoryDevice>();
                let mut vec = Vec::new();
                for dev in smb {
                    vec.push(MemDevice::from(dev));
                }
                Some(vec)
            }
            Err(_) => None,
        };

        Ok(Self {
            total: Data::from_octets(meminfo.mem_total as f64),
            avail: Data::from_octets(meminfo.mem_available.unwrap() as f64),
            display_unit,
            devices: mem_vec,
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

    fn long_content(&self) -> Option<Vec<crate::FetchSection>> {
        let mut vec = vec![
            FetchSection::new_short("Total", format!("{:.2}", self.total)),
            FetchSection::new_short("Used", format!("{:.2}", self.used())),
            FetchSection::new_short("Available", format!("{:.2}", self.avail)),
        ];
        if let Some(ref s) = self.devices {
            let mut devices = Vec::new();
            for dev in s {
                devices.push(FetchSection {
                    name: dev.location.clone(),
                    content: FetchType::Long(vec![
                        FetchSection::new_short("Manufacturer", dev.manufacturer.clone()),
                        FetchSection::new_short("Type", dev.mem_type.clone()),
                        FetchSection::new_short("Capacity", format!("{:.2}", dev.size)),
                        FetchSection::new_short(
                            "Speed",
                            format!("{} MT/s", dev.speed.as_megahertz()),
                        ),
                        FetchSection::new_short("Part #", dev.part_number.clone()),
                    ]),
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
