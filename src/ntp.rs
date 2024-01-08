use chrono::{DateTime, NaiveDateTime, Utc};
use core::fmt::Display;
use std::net::UdpSocket;
use core::time::Duration;
use serde::{Deserialize, Serialize};

use crate::{Time, TimeDiff, OFFSET_1601, REF_TIME_1970};

/// NTP time
///
/// `inner_secs` is the time as seconds since `1601-01-01 00:00:00`, from `chrono::Utc`
/// `inner_milliseconds` is the subsec milliseconds
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Ntp {
    inner_secs: u64,
    inner_milliseconds: u64,
    server: String,
    utc_offset: i32,
}

impl Display for Ntp {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.pretty())
    }
}

impl Ntp {
    /// Returns the server address used to get the time
    pub fn server(&self) -> String {
        self.server.to_string()
    }

    /// returns whether the data was fetched from a valid server (ie not strptime or chrono::Utc)
    pub fn valid_server(&self) -> bool {
        !["chrono::Utc", "strptime"].contains(&self.server.as_str())
    }
}

impl TimeDiff for Ntp {}

impl Time for Ntp {
    /// Note - there is a chance that this function fails, in which case we use the System time as a failsafe
    fn now() -> Self {
        match Ntp::new("pool.ntp.org") {
            Ok(x) => x,
            Err(_) => {
                let now = Utc::now();
                Ntp {
                    inner_secs: (now.timestamp() + OFFSET_1601 as i64) as u64,
                    inner_milliseconds: now.timestamp_subsec_millis() as u64,
                    server: "chrono::Utc".to_string(),
                    utc_offset: 0,
                }
            },
        }
    }
    fn unix(&self) -> i64 {
        (self.inner_secs as i64) - (OFFSET_1601 as i64)
    }
    fn unix_ms(&self) -> i64 {
        ((self.inner_secs as i64 * 1000i64) + self.inner_milliseconds as i64) - (OFFSET_1601 as i64 * 1000i64)
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
        Ntp {
            inner_secs: (x.timestamp() + (OFFSET_1601 as i64)) as u64,
            inner_milliseconds: x.timestamp_subsec_millis() as u64,
            server: "strptime".to_string(),
            utc_offset: x.offset().local_minus_utc() as i32,
        }
    }

    fn strftime(&self, format: &str) -> String {
        NaiveDateTime::from_timestamp_opt(self.inner_secs as i64 - OFFSET_1601 as i64, 0)
            .unwrap()
            .format(format)
            .to_string()
    }

    fn from_epoch(timestamp: u64) -> Self {
        Ntp {
            inner_secs: timestamp / 1000,
            inner_milliseconds: timestamp % 1000,
            server: "from_epoch".to_string(),
            utc_offset: 0,
        }
    }

    fn raw(&self) -> u64 {
        (self.inner_secs * 1000) + self.inner_milliseconds
    }

    fn from_epoch_offset(timestamp: u64, offset: i32) -> Self {
        Ntp {
            inner_secs: timestamp,
            inner_milliseconds: timestamp % 1000,
            server: "from_epoch_offset".to_string(),
            utc_offset: offset,
        }
    }
}


impl Ntp {
    /// Fetches the time from an NTP server
    /// 
    /// # Example
    /// ```
    /// use thetime::Ntp;
    /// let ntp = Ntp::new("pool.ntp.org").unwrap();
    /// println!("{}", ntp);
    /// ```
    pub fn new<T: ToString>(server_addr: T) -> Result<Ntp, Box<dyn std::error::Error>> {
        let server = server_addr.to_string();
        let client = UdpSocket::bind("0.0.0.0:0")?;
        client.set_read_timeout(Some(Duration::from_secs(5)))?;
    
        let mut data = vec![0x1b];
        data.extend(vec![0; 47]); // ping
    
        let start_time = Utc::now().timestamp_millis();
        client.send_to(&data, format!("{}:123", server))?;
    
        let mut buffer = [0; 1024];
        let (size, _) = client.recv_from(&mut buffer)?;
    
        if size > 0 {
            let t = u32::from_be_bytes([buffer[40], buffer[41], buffer[42], buffer[43]]) as u64;
            let t = t - REF_TIME_1970;
    
            let elapsed_time = start_time - Utc::now().timestamp_millis();
            let milliseconds = (elapsed_time % 1000).try_into().unwrap_or(0);
    
            return Ok(Ntp {
                server: server.to_string(),
                inner_secs: (t) + OFFSET_1601,
                inner_milliseconds: milliseconds,
                utc_offset: 0,
            });
        }
    
        Err("Failed to receive NTP response".into())
    }
    
}
