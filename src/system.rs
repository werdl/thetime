use crate::{Time, TimeDiff, OFFSET_1601};
use chrono::{DateTime, NaiveDateTime, Local};
use core::fmt::Display;

/// System time, as grabbed from the system (obviously). Its timezone is dependent on the system's timezone as configured in the BIOS
///
/// `inner_secs` is the time as seconds since `1601-01-01 00:00:00`, from `std::time`
/// `inner_milliseconds` is the subsec milliseconds
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct System {
    inner_secs: u64,
    inner_milliseconds: u64,
    pub utc_offset: i32,
}

impl Display for System {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.strftime("%Y-%m-%d %H:%M:%S"))
    }
}

impl TimeDiff for System {}

impl Time for System {
    fn now() -> Self {
        let now: DateTime<Local> = Local::now();
        System {
            inner_secs: (now.timestamp() + OFFSET_1601 as i64) as u64,
            inner_milliseconds: (now.timestamp_subsec_millis()) as u64,
            utc_offset: now.offset().local_minus_utc(),
        }
    }

    fn utc_offset(&self) -> i32 {
        self.utc_offset
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
            utc_offset: x.offset().local_minus_utc() as i32,
        }
    }

    fn unix(&self) -> i64 {
        (self.inner_secs as i64) - (OFFSET_1601 as i64)
    }
    fn unix_ms(&self) -> i64 {
        ((self.inner_secs as i64 * 1000i64) + self.inner_milliseconds as i64) - (OFFSET_1601 as i64 * 1000i64)
    }

    fn strftime(&self, format: &str) -> String {
        let timestamp = if self.inner_secs >= OFFSET_1601 {
            (self.inner_secs - OFFSET_1601) as i64
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
            inner_secs: (timestamp / 1000),
            inner_milliseconds: timestamp % 1000,
            utc_offset: 0,
        }
    }

    fn raw(&self) -> u64 {
        (self.inner_secs * 1000) + self.inner_milliseconds
    }

    fn from_epoch_offset(timestamp: u64, offset: i32) -> Self {
        System {
            inner_secs: (timestamp / 1000),
            inner_milliseconds: timestamp % 1000,
            utc_offset: offset,
        }
    }
}
