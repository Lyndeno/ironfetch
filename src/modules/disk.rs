use futures::stream::{FuturesUnordered, StreamExt};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use zbus::zvariant::OwnedObjectPath;

use crate::{fetch::Fetch, Result, GIGABYTE, TERABYTE};

#[derive(Serialize, Deserialize, Clone, Fetch)]
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
pub async fn get_capacity() -> Result<u64> {
    let client = udisks2::Client::new().await?;
    let manager = client.manager();
    let objects = manager.get_block_devices(HashMap::new()).await?;

    let f: FuturesUnordered<_> = objects.into_iter().map(|s| get_size(s, &client)).collect();

    let hm: HashMap<String, u64> = f.filter_map(|x| async { x.ok() }).collect().await;
    Ok(hm.into_iter().map(|x| x.1).sum())
}

async fn get_size(drivestr: OwnedObjectPath, client: &udisks2::Client) -> Result<(String, u64)> {
    let object = client.object(drivestr.to_string())?;
    let block_proxy = object.block().await?;
    let drive_proxy = client.drive_for_block(&block_proxy).await?;
    Ok((drive_proxy.id().await?, drive_proxy.size().await?))
}

#[allow(clippy::cast_precision_loss)]
impl std::fmt::Display for Disk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.capacity < TERABYTE {
            write!(f, "{:.1} GB", self.capacity as f64 / GIGABYTE as f64)
        } else {
            write!(f, "{:.1} TB", self.capacity as f64 / TERABYTE as f64)
        }
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
