use crate::fetcherror::FetchError;
use measurements::Data;
use std::collections::HashMap;

pub struct Disk {
    pub capacity: Data,
}

impl Disk {
    pub fn new() -> Result<Self, FetchError> {
        Ok(Self {
            capacity: futures::executor::block_on(get_capacity())?,
        })
    }
}

pub async fn get_capacity() -> Result<Data, FetchError> {
    let client = udisks2::Client::new().await?;
    let manager = client.manager();
    let objects = manager.get_block_devices(HashMap::new()).await?;

    let mut v = Vec::new();
    for obj in objects {
        v.push(obj.to_string());
    }

    let mut hm = HashMap::new();
    for drivestr in v {
        let object = client.object(drivestr.clone());
        if let Ok(o) = object {
            let block = o.block().await;
            if let Ok(b) = block {
                let drive = client.drive_for_block(&b).await;
                if let Ok(d) = drive {
                    hm.insert(d.id().await?, d.size().await?);
                }
            }
        }
    }
    //let sum: u64 = hm.iter().map(|x| x.1).sum();
    //let total = Data::from_octets(sum as f64);
    Ok(Data::from_octets(hm.into_iter().map(|x| x.1 as f64).sum()))
}

impl std::fmt::Display for Disk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.capacity < Data::from_teraoctets(1.0) {
            write!(f, "{:.1} GB", self.capacity.as_gigaoctets())
        } else {
            write!(f, "{:.1} TB", self.capacity.as_teraoctets())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tb() {
        let disk = Disk {
            capacity: Data::from_teraoctets(1.0),
        };
        let display = disk.to_string();
        let desired = "1.0 TB";
        assert_eq!(&display, desired);
    }

    #[test]
    fn test_gb() {
        let disk = Disk {
            capacity: Data::from_gigaoctets(100.0),
        };
        let display = disk.to_string();
        let desired = "100.0 GB";
        assert_eq!(&display, desired);
    }
}
