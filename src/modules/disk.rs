use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::{fetch::Fetch, Result, GIGABYTE, TERABYTE};

#[derive(Serialize, Deserialize, Clone)]
pub struct Disk {
    pub capacity: u64,
}

impl Disk {
    /// Returns disk capacity
    ///
    /// # Errors
    /// Returns an error if there is an issue retrieving disk capacity
    pub fn new() -> Result<Self> {
        Ok(Self {
            capacity: futures::executor::block_on(get_capacity())?,
        })
    }
}

/// Gets disk capacity from all drives on system
///
/// # Errors
/// Returns an error if there is a problem communicating with udisks
#[allow(clippy::cast_precision_loss)]
pub async fn get_capacity() -> Result<u64> {
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
            if let Ok(b) = o.block().await {
                if let Ok(d) = client.drive_for_block(&b).await {
                    hm.insert(d.id().await?, d.size().await?);
                }
            }
        }
    }
    Ok(hm.into_iter().map(|x| x.1).sum())
}

impl std::fmt::Display for Disk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.capacity < TERABYTE {
            write!(f, "{:.1} GB", self.capacity as f64 / GIGABYTE as f64)
        } else {
            write!(f, "{:.1} TB", self.capacity as f64 / TERABYTE as f64)
        }
    }
}

impl Fetch for Disk {
    fn name(&self) -> &'static str {
        "Disk"
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tb() {
        let disk = Disk { capacity: TERABYTE };
        let display = disk.to_string();
        let desired = "1.0 TB";
        assert_eq!(&display, desired);
    }

    #[test]
    fn test_gb() {
        let disk = Disk {
            capacity: 100 * GIGABYTE,
        };
        let display = disk.to_string();
        let desired = "100.0 GB";
        assert_eq!(&display, desired);
    }
}
