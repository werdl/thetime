use std::time::SystemTime;
use core::fmt::Display;
use chrono::{DateTime, Local};
use crate::{Time, TimeDiff};

/// System time, as grabbed from the system (obviously)
/// 
/// `inner` is the time as a SystemTime struct, from `std::time`
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct System {
    inner: SystemTime,
}

impl Display for System {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.unix())
    }
}

impl TimeDiff for System {
    fn diff<T: Time>(&self, other: &T) -> f64 {
        (self.unix() - other.unix()) as f64
    }

    fn diff_ms<T: Time>(&self, other: &T) -> f64 {
        (self.unix_ms() - other.unix_ms()) as f64
    }
}

impl Time for System {
    fn now() -> Self {
        System{
            inner: SystemTime::now()
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
            inner: x.into()
        }
    }

    fn unix(&self) -> u64 {
        self.inner.duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs()
    }

    fn unix_ms(&self) -> f64 {
        self.inner.duration_since(SystemTime::UNIX_EPOCH).unwrap().as_millis() as f64
    }
    
    fn strftime(&self, format: &str) -> String {
        let x: DateTime<Local> = DateTime::from(self.inner);
            x.format(format)
            .to_string()
    }
}