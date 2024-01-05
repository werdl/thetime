use chrono::{DateTime, NaiveDateTime};
use core::fmt::Display;
use std::net::UdpSocket;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use crate::{Time, TimeDiff, OFFSET_1601, REF_TIME_1970};

/// NTP time
///
/// `inner_secs` is the time as seconds since `1601-01-01 00:00:00`, from `std::time`
/// `inner_milliseconds` is the subsec milliseconds
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Ntp {
    inner_secs: u64,
    inner_milliseconds: u64,
    server: String,
}

impl Display for Ntp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#?}", self)
    }
}

impl TimeDiff for Ntp {}

impl Time for Ntp {
    /// Note - there is a chance that this function fails, in which case we use the System time as a failsafe
    fn now() -> Self {
        match new("pool.ntp.org") {
            Ok(x) => x,
            Err(_) => Ntp {
                inner_secs: SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs(),
                inner_milliseconds: 0,
                server: "SystemTime".to_string(),
            },
        }
    }
    fn unix(&self) -> u64 {
        self.inner_secs - (OFFSET_1601 as u64)
    }
    fn unix_ms(&self) -> u64 {
        ((self.inner_secs * 1000) + self.inner_milliseconds) - ((OFFSET_1601 as u64) * 1000)
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
            inner_milliseconds: x.timestamp_millis() as u64,
            server: "strptime".to_string(),
        }
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
        Ntp {
            inner_secs: timestamp,
            inner_milliseconds: timestamp % 1000,
            server: "from_epoch".to_string(),
        }
    }

    fn raw(&self) -> u64 {
        (self.inner_secs * 1000) + self.inner_milliseconds
    }
}

fn new<T: ToString>(server_addr: T) -> Result<Ntp, Box<dyn std::error::Error>> {
    let server = server_addr.to_string();
    let client = UdpSocket::bind("0.0.0.0:0")?;
    client.set_read_timeout(Some(Duration::from_secs(5)))?;

    let mut data = vec![0x1b];
    data.extend(vec![0; 47]); // ping

    let start_time = SystemTime::now();
    client.send_to(&data, format!("{}:123", server))?;

    let mut buffer = [0; 1024];
    let (size, _) = client.recv_from(&mut buffer)?;

    if size > 0 {
        let t = u32::from_be_bytes([buffer[40], buffer[41], buffer[42], buffer[43]]) as u64;
        let t = t - u64::from(REF_TIME_1970);

        let elapsed_time = start_time.elapsed()?;
        let milliseconds = elapsed_time.as_secs() * 1000 + u64::from(elapsed_time.subsec_millis());

        return Ok(Ntp {
            server: server.to_string(),
            inner_secs: (t) + (OFFSET_1601 as u64),
            inner_milliseconds: milliseconds,
        });
    }

    Err("Failed to receive NTP response".into())
}
