use libc::{time_t, timeval};

use sys_info::boottime;

const SECONDS_MIN: time_t = 60;
const SECONDS_HOUR: time_t = SECONDS_MIN * 60;
const SECONDS_DAY: time_t = SECONDS_HOUR * 24;
pub struct Uptime(pub timeval);

impl Uptime {
    pub fn new() -> Result<Self, sys_info::Error> {
        Ok(Self(boottime()?))
    }
}

impl std::fmt::Display for Uptime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // TODO: Tweak logic so that words aren't always plural
        let uptime_days = self.0.tv_sec / SECONDS_DAY;
        let uptime_hours = (self.0.tv_sec - uptime_days*SECONDS_DAY)/SECONDS_HOUR;
        let uptime_minutes = (self.0.tv_sec - uptime_hours*SECONDS_HOUR - uptime_days*SECONDS_DAY)/SECONDS_MIN;
        let uptime_seconds = self.0.tv_sec - uptime_minutes*SECONDS_MIN - uptime_hours*SECONDS_HOUR - uptime_days*SECONDS_DAY;

        let plural = "s";
        let singular = "";

        let mut s = String::new();

        let mut plurality;
        if uptime_days > 0 {
            if uptime_days != 1 { plurality = plural; } else { plurality = singular; }
            s.push_str(uptime_days.to_string().as_str());
            s.push_str(" day");
            s.push_str(plurality);
            s.push_str(", ");
        } 
        if uptime_hours > 0 {
            if uptime_hours != 1 { plurality = plural; } else { plurality = singular; }
            s.push_str(uptime_hours.to_string().as_str());
            s.push_str(" hour");
            s.push_str(plurality);
            s.push_str(", ");
        }
        if uptime_minutes != 0 {
            if uptime_minutes > 1 { plurality = plural; } else { plurality = singular; }
            s.push_str(uptime_minutes.to_string().as_str());
            s.push_str(" minute");
            s.push_str(plurality);
            s.push_str(", ");
        }
        if uptime_seconds != 1 { plurality = plural; } else { plurality = singular; }
        s.push_str(uptime_seconds.to_string().as_str());
        s.push_str(" second");
        s.push_str(plurality);
        write!(f, "{}", s)
    }
}