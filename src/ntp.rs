use std::net::UdpSocket;
use core::fmt::Display;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use chrono::{DateTime, TimeZone, Local};

use crate::{Time, TimeDiff};

const REF_TIME_1970: u32 = 2208988800; // Reference time

/// NTP time
/// 
/// `inner` is milliseconds since Unix epoch, by default UTC, and `server` is the NTP server address that the time was fetched from.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Ntp {
    inner: u64,
    server: String,
}

impl Display for Ntp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.unix())
    }
}

impl Ntp {
    /// Convert NTP time to local time
    pub fn local(&self) -> Self {
        let seconds = self.inner / 1000;
        let nanoseconds = (self.inner % 1000) * 1_000_000;
        Ntp {
            inner: Local.timestamp_opt(seconds as i64, nanoseconds as u32).unwrap().timestamp_millis() as u64,
            server: self.server.clone(),
        }
        
    }
}

impl TimeDiff for Ntp {
    fn diff<T: Time>(&self, other: &T) -> f64 {
        (self.unix() - other.unix()) as f64
    }

    fn diff_ms<T: Time>(&self, other: &T) -> f64 {
        (self.unix_ms() - other.unix_ms()) as f64
    }
}

impl Time for Ntp {
    fn now() -> Self {
        new("pool.ntp.org").unwrap()
    }
    fn unix(&self) -> u64 {
        self.inner / 1000
    }
    fn unix_ms(&self) -> f64 {
        self.inner as f64
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
            inner: x.timestamp_millis() as u64,
            server: "strptime".to_string(),
        }
    }

    fn strftime(&self, format: &str) -> String {
        let x = UNIX_EPOCH + Duration::from_millis(self.inner);
        let x: DateTime<Local> = DateTime::from(x);
        x.format(format)
            .to_string()
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
        let t = u32::from_be_bytes([
            buffer[40], buffer[41], buffer[42], buffer[43],
        ]) as u64;
        let t = t - u64::from(REF_TIME_1970);

        let elapsed_time = start_time.elapsed()?;
        let milliseconds = elapsed_time.as_secs() * 1000 + u64::from(elapsed_time.subsec_millis());

        return Ok(Ntp{
            server: server.to_string(),
            inner: t * 1000 + milliseconds
        });
    }

    Err("Failed to receive NTP response".into())
}
