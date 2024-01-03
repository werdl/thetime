use std::time::SystemTime;
use core::fmt::Display;
use chrono::{DateTime, Local};
use crate::Time;

/// System time, as grabbed from the system (obviously)
/// 
/// `inner` is the time as a SystemTime struct, from `std::time`
pub struct System {
    inner: SystemTime,
}

impl Display for System {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.unix())
    }
}

impl Time for System {
    fn now() -> Self {
        System{
            inner: SystemTime::now()
        }
    }
    
    fn unix(&self) -> u64 {
        SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs()
    }

    fn unix_ms(&self) -> f64 {
        SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_millis() as f64
    }
    
    fn strftime(&self, format: &str) -> String {
        let x: DateTime<Local> = DateTime::from(self.inner);
            x.format(format)
            .to_string()
    }
}