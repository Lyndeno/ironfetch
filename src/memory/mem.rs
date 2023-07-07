use smbioslib::{table_load_from_device, MemorySize, MemorySizeExtended, SMBiosMemoryDevice};

// This only works when running as root

// Type to store basic information about memory devices, such as ramsticks.
// Minimally tested, it is possible information shows up in here for empty dimm slots as well.
pub struct MemDevice {
    pub speed: Frequency,
    pub part_number: Option<String>,
    pub location: String,
    pub manufacturer: Option<String>,
    pub size: Option<Data>,
    pub mem_type: Option<String>,
}

impl MemDevice {
    pub fn from_smbios_table() -> Result<Option<Vec<Self>>, FetchError> {
        let smb_vec = table_load_from_device()?;
        let smb = smb_vec.defined_struct_iter::<SMBiosMemoryDevice>();
        let mut vec = Vec::new();
        for dev in smb {
            vec.push(MemDevice::from(dev));
        }
        Ok(Some(vec))
    }
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
            part_number: dev.part_number().ok(),
            manufacturer: dev.manufacturer().ok(),
            size: match dev.size() {
                Some(MemorySize::Kilobytes(d)) => Some(Data::from_kibioctets(d as f64)),
                Some(MemorySize::Megabytes(d)) => Some(Data::from_mebioctets(d as f64)),
                Some(MemorySize::SeeExtendedSize) => match dev.extended_size() {
                    Some(MemorySizeExtended::Megabytes(d)) => Some(Data::from_mebioctets(d as f64)),
                    _ => None,
                },
                _ => None,
            },
            mem_type: match dev.memory_type() {
                Some(v) => Some(format!("{:?}", v.value).to_uppercase()), // TODO: This is gross
                None => None,
            },
        }
    }
}

use measurements::{data::Data, Frequency};

use crate::fetcherror::FetchError;
