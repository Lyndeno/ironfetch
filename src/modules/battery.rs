use std::fmt::Display;

use serde::{Deserialize, Serialize};
use upower_dbus::{BatteryState, UPowerProxy};

use crate::{fetch::Fetch, Result};

#[derive(Clone, Serialize, Deserialize)]
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

impl Fetch for Battery {
    fn name(&self) -> &'static str {
        "Battery"
    }
}

impl std::fmt::Display for Battery {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} at {:.0}%", self.state, self.percentage)
    }
}
