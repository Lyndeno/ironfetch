use std::fmt::Display;

use derive_more::Display;
use serde::{Deserialize, Serialize};
use upower_dbus::{BatteryState, UPowerProxy};

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
