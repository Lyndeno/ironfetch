use std::fmt::Display;

use derive_more::Display;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use zbus::{proxy, zvariant::OwnedValue};

use crate::{fetch::Fetch, Result};

#[derive(Clone, Serialize, Deserialize, Fetch, Display)]
#[display("{} at {:.0}%", state, percentage)]
pub struct Battery {
    percentage: f64,
    state: State,
}

#[derive(Clone, Serialize, Deserialize)]
struct State(BatteryState);

impl From<BatteryState> for State {
    fn from(value: BatteryState) -> Self {
        Self(value)
    }
}

#[derive(Copy, Clone, Debug, Serialize_repr, Deserialize_repr, OwnedValue, PartialEq, Eq)]
#[repr(u32)]
enum BatteryState {
    Unknown = 0,
    Charging = 1,
    Discharging = 2,
    Empty = 3,
    FullyCharged = 4,
    PendingCharge = 5,
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

#[allow(clippy::enum_glob_use)]
impl Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use BatteryState::*;
        let s = match self.0 {
            Unknown => "Unknown",
            Charging => "Charging",
            Discharging => "Discharging",
            Empty => "Empty",
            FullyCharged => "Fully Charged",
            PendingCharge => "Pending Charge",
            PendingDischarge => "Pending Discharge",
        };
        write!(f, "{s}")
    }
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
        futures::executor::block_on(async move {
            let connection = zbus::Connection::system().await?;

            let upower = UPowerProxy::new(&connection).await?;

            let device = upower.get_display_device().await?;
            let percentage = device.percentage().await?;
            let state = device.state().await?;

            if state == BatteryState::Unknown && percentage == 0f64 {
                return Ok(None);
            }

            Ok(Some(Self {
                percentage,
                state: state.into(),
            }))
        })
    }
}
