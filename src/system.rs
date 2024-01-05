use crate::{Time, TimeDiff, OFFSET_1601};
use chrono::{DateTime, NaiveDateTime};
use core::fmt::Display;
use std::time::SystemTime;

/// System time, as grabbed from the system (obviously). Its timezone is dependent on the system's timezone as configured in the BIOS
///
/// `inner_secs` is the time as seconds since `1601-01-01 00:00:00`, from `std::time`
/// `inner_milliseconds` is the subsec milliseconds
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct System {
    inner_secs: u64,
    inner_milliseconds: u64,
}

impl Display for System {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.strftime("%Y-%m-%d %H:%M:%S"))
    }
}

impl TimeDiff for System {}

impl Time for System {
    fn now() -> Self {
        System {
            inner_secs: SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_secs()
                + (OFFSET_1601 as u64),
            inner_milliseconds: SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .subsec_millis() as u64,
        }
    }

    fn strptime<T: ToString, G: ToString>(s: T, format: G) -> Self {
        let s = s.to_string();
        let format = format.to_string();
        let temp = DateTime::parse_from_str(&s, &format);
        let x = match temp {
            Err(_) => {
                if !format.contains("%z") {
                    return Self::strptime(s + " +0000", format + "%z");
                }
                panic!("Bad format string");
            }
            Ok(dt) => dt,
        };
        System {
            inner_secs: (x.timestamp() + (OFFSET_1601 as i64)) as u64,
            inner_milliseconds: x.timestamp_subsec_millis() as u64,
        }
    }

    fn unix(&self) -> u64 {
        self.inner_secs - (OFFSET_1601 as u64)
    }

    fn unix_ms(&self) -> u64 {
        ((self.inner_secs * 1000) + self.inner_milliseconds) - ((OFFSET_1601 as u64) * 1000)
    }

    fn strftime(&self, format: &str) -> String {
        let timestamp = if self.inner_secs >= (OFFSET_1601 as u64) {
            (self.inner_secs - (OFFSET_1601 as u64)) as i64
        } else {
            -((OFFSET_1601 as i64) - (self.inner_secs as i64))
        };
        NaiveDateTime::from_timestamp_opt(timestamp, 0)
            .unwrap()
            .format(format)
            .to_string()
    }

    fn from_epoch(timestamp: u64) -> Self {
        System {
            inner_secs: timestamp,
            inner_milliseconds: timestamp % 1000,
        }
    }

    fn raw(&self) -> u64 {
        (self.inner_secs * 1000) + self.inner_milliseconds
    }
}
