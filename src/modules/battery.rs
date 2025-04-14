use derive_more::Display;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use zbus::{proxy, zvariant::OwnedValue};

use crate::{fetch::Fetch, Result};

#[derive(Clone, Serialize, Deserialize, Fetch, Display)]
#[display("{} at {:.0}%", state, percentage)]
pub struct Battery {
    percentage: f64,
    state: BatteryState,
}

#[derive(
    Copy, Clone, Debug, Serialize_repr, Deserialize_repr, OwnedValue, PartialEq, Eq, Display,
)]
#[repr(u32)]
enum BatteryState {
    #[display("Unknown")]
    Unknown = 0,
    #[display("Charging")]
    Charging = 1,
    #[display("Discharging")]
    Discharging = 2,
    #[display("Empty")]
    Empty = 3,
    #[display("Fully Charged")]
    FullyCharged = 4,
    #[display("Pending Charge")]
    PendingCharge = 5,
    #[display("Pending Discharge")]
    PendingDischarge = 6,
}

#[derive(Copy, Clone, Debug, Serialize_repr, Deserialize_repr, OwnedValue, PartialEq, Eq)]
#[repr(u32)]
enum BatteryLevel {
    Unknown = 0,
    None = 1,
    Low = 3,
    Critical = 4,
    Normal = 6,
    High = 7,
    Full = 8,
}

#[proxy(interface = "org.freedesktop.UPower", assume_defaults = true)]
trait UPower {
    #[zbus(object = "Device")]
    fn get_display_device(&self);
}

#[proxy(
    interface = "org.freedesktop.UPower.Device",
    default_service = "org.freedesktop.UPower",
    assume_defaults = false
)]
trait Device {
    #[zbus(property)]
    fn battery_level(&self) -> zbus::Result<BatteryLevel>;

    #[zbus(property)]
    fn percentage(&self) -> zbus::Result<f64>;

    #[zbus(property)]
    fn state(&self) -> zbus::Result<BatteryState>;
}

impl Battery {
    /// Returns battery if found
    ///
    /// # Errors
    /// Returns an error if there is a problem talking to upower
    ///
    /// # Returns
    /// Returns None if there is no battery.
    pub fn new() -> Result<Option<Self>> {
        let connection = zbus::blocking::Connection::system()?;

        let upower = UPowerProxyBlocking::new(&connection)?;

        let device = upower.get_display_device()?;
        let percentage = device.percentage()?;
        let state = device.state()?;

        if state == BatteryState::Unknown && percentage == 0f64 {
            return Ok(None);
        }

        Ok(Some(Self { percentage, state }))
    }
}
