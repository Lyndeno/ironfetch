use smbioslib::{MemorySize, MemorySizeExtended, SMBiosMemoryDevice};

// This only works when running as root
pub struct MemDevice {
    pub speed: Frequency,
    pub part_number: String,
    pub location: String,
    pub manufacturer: String,
    pub size: Data,
    pub mem_type: String,
}

impl From<SMBiosMemoryDevice<'_>> for MemDevice {
    fn from(dev: SMBiosMemoryDevice<'_>) -> Self {
        Self {
            speed: match dev.configured_memory_speed() {
                Some(v) => match v {
                    smbioslib::MemorySpeed::MTs(s) => Frequency::from_megahertz(s as f64),
                    _ => Frequency::from_megahertz(0_f64),
                },
                _ => Frequency::from_megahertz(0_f64),
            },
            location: dev.device_locator().ok().unwrap(),
            part_number: dev.part_number().ok().unwrap(),
            manufacturer: dev.manufacturer().ok().unwrap(),
            size: match dev.size().unwrap() {
                MemorySize::Kilobytes(d) => Data::from_kibioctets(d as f64),
                MemorySize::Megabytes(d) => Data::from_mebioctets(d as f64),
                MemorySize::SeeExtendedSize => match dev.extended_size().unwrap() {
                    MemorySizeExtended::Megabytes(d) => Data::from_mebioctets(d as f64),
                    _ => Data::from_bits(0_f64),
                },
                _ => Data::from_bits(0_f64),
            },
            mem_type: format!("{:?}", dev.memory_type().unwrap().value).to_uppercase(), // TODO: This is gross
        }
    }
}

use measurements::{data::Data, Frequency};
