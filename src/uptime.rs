use std::time::Duration;

use crate::fetchitem::FetchItem;

const SECONDS_MIN: u64 = 60;
const SECONDS_HOUR: u64 = SECONDS_MIN * 60;
const SECONDS_DAY: u64 = SECONDS_HOUR * 24;
pub struct Uptime(pub Duration);

impl Uptime {
    pub fn new() -> Result<Self, String> {
        Ok(Self(uptime_lib::get()?))
    }
}

impl std::fmt::Display for Uptime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let days = self.0.as_secs() / SECONDS_DAY;
        let hours = (self.0.as_secs() - days * SECONDS_DAY) / SECONDS_HOUR;
        let minutes = (self.0.as_secs() - hours * SECONDS_HOUR - days * SECONDS_DAY) / SECONDS_MIN;
        let seconds =
            self.0.as_secs() - minutes * SECONDS_MIN - hours * SECONDS_HOUR - days * SECONDS_DAY;

        let v = Vec::from([
            ("day", days),
            ("hour", hours),
            ("minute", minutes),
            ("second", seconds),
        ]);

        let mut s = String::new();

        let len = v.len();
        for unit in v.iter().enumerate() {
            if unit.1 .1 > 0 {
                s.push_str(unit.1 .1.to_string().as_str());
                s.push(' ');
                s.push_str(unit.1 .0);
                if unit.1 .1 != 1 {
                    s.push('s')
                };
                if unit.0 < len - 1 {
                    s.push_str(", ")
                };
            }
        }
        write!(f, "{}", s)
    }
}

impl FetchItem for Uptime {
    fn name(&self) -> String {
        String::from("Uptime")
    }
}
