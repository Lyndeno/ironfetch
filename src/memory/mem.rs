use std::path::Path;

use smbioslib::{
    load_smbios_data_from_file, table_load_from_device, MemorySize, MemorySizeExtended, SMBiosData,
    SMBiosMemoryDevice,
};

// This only works when running as root

// Type to store basic information about memory devices, such as ramsticks.
// Minimally tested, it is possible information shows up in here for empty dimm slots as well.
#[derive(Clone)]
pub struct MemDevice {
    pub speed: Option<Frequency>,
    pub part_number: Option<String>,
    pub location: String,
    pub manufacturer: Option<String>,
    pub size: Option<Data>,
    pub mem_type: Option<String>,
}

pub enum SMBiosSource<'a> {
    Local,
    File(&'a Path),
    Data(SMBiosData),
}

impl MemDevice {
    pub fn from_source(source: SMBiosSource) -> Result<Option<Vec<Self>>, FetchError> {
        match source {
            SMBiosSource::Local => Self::from_smbios_local_device(),
            SMBiosSource::File(path) => Self::from_smbios_dump(path),
            SMBiosSource::Data(data) => Self::from_smbios_data(data),
        }
    }
    fn from_smbios_local_device() -> Result<Option<Vec<Self>>, FetchError> {
        let data = table_load_from_device()?;
        Self::from_smbios_data(data)
    }

    fn from_smbios_dump(path: &Path) -> Result<Option<Vec<Self>>, FetchError> {
        let data = load_smbios_data_from_file(path)?;
        Self::from_smbios_data(data)
    }

    fn from_smbios_data(data: SMBiosData) -> Result<Option<Vec<Self>>, FetchError> {
        let smb = data.defined_struct_iter::<SMBiosMemoryDevice>();
        let mut vec = Vec::new();
        for dev in smb {
            if let Ok(d) = MemDevice::try_from(dev) {
                vec.push(d);
            }
        }
        Ok(Some(vec))
    }
}

impl TryFrom<SMBiosMemoryDevice<'_>> for MemDevice {
    type Error = &'static str;
    fn try_from(dev: SMBiosMemoryDevice<'_>) -> Result<Self, Self::Error> {
        Ok(Self {
            speed: match dev.configured_memory_speed() {
                Some(v) => match v {
                    smbioslib::MemorySpeed::MTs(s) => Some(Frequency::from_megahertz(s as f64)),
                    smbioslib::MemorySpeed::Unknown => None,
                    smbioslib::MemorySpeed::SeeExtendedSpeed => todo!(),
                },
                _ => None,
            },
            // This should not error if something is there
            location: match dev.device_locator().ok() {
                Some(v) => v,
                None => return Err("Error getting device locator"),
            },
            // This String has trailing whitespace
            part_number: dev.part_number().ok().map(|s| s.trim().to_string()),
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
        })
    }
}

use measurements::{data::Data, Frequency};

use crate::fetcherror::FetchError;

#[cfg(test)]
mod tests {
    use super::*;
    fn get_first_mem_device() -> MemDevice {
        let mem = MemDevice::from_smbios_dump(Path::new("./dmi.bin"));
        mem.unwrap().unwrap()[0].clone()
    }

    #[test]
    fn test_manufactuer() {
        let dev = get_first_mem_device();
        assert_eq!(dev.manufacturer, Some(String::from("8C260000802C")));
    }

    #[test]
    fn test_part() {
        let dev = get_first_mem_device();
        assert_eq!(dev.part_number, Some(String::from("TIMETEC-SD4-2666")));
    }

    #[test]
    fn test_type() {
        let dev = get_first_mem_device();
        assert_eq!(dev.mem_type, Some(String::from("DDR4")));
    }

    #[test]
    fn test_capacity() {
        let dev = get_first_mem_device();
        assert_eq!(dev.size, Some(Data::from_gibioctets(32.00)));
    }

    #[test]
    fn test_freq() {
        let dev = get_first_mem_device();
        assert_eq!(dev.speed, Some(Frequency::from_megahertz(2667.00)));
    }
}
