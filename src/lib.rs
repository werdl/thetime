pub mod ntp;
pub mod system;

pub use ntp::*;
pub use system::*;

extern crate time;

/// Returns the current time in seconds since Unix epoch
/// 
/// # Examples
/// ```rust
/// use thetime::now;
/// println!("{} seconds since Unix epoch", now());
/// ```
pub fn now() -> u64 {
    Ntp::now().unix()
}


/// Implements the core functionality of the library
pub trait Time {
    /// Get current time, returning the relevant struct
    ///
    /// # Examples
    /// ```rust
    /// use thetime::{System, Ntp, Time};
    /// println!("{} says the system, but {} says the server", System::now(), Ntp::now());
    /// ```
    fn now() -> Self;

    /// Parse a string into a time struct
    ///
    /// # Examples
    /// ```rust
    /// use thetime::{System, Ntp, Time};
    /// println!("The time was {}", System::strptime("2015-01-18 23:16:09", "%Y-%m-%d %H:%M:%S"));
    /// ```
    fn strptime<T: ToString, G: ToString>(s: T, format: G) -> Self;

    /// Get the time in seconds since Unix epoch
    ///
    /// # Examples
    /// ```rust
    /// use thetime::{System, Ntp, Time};
    /// println!("{} seconds since Unix epoch", System::now().unix());
    /// println!("{} seconds since Unix epoch from pool.ntp.org", Ntp::now().unix());
    /// ```
    fn unix(&self) -> u64;

    /// Get the time in milliseconds since Unix epoch
    ///
    /// # Examples
    /// ```rust
    /// use thetime::{System, Ntp, Time};
    /// println!("{} milliseconds since Unix epoch", System::now().unix_ms());
    /// println!("{} milliseconds since Unix epoch from pool.ntp.org", Ntp::now().unix_ms());
    /// ```
    fn unix_ms(&self) -> f64;

    /// Gets the time in nanoseconds (approximate) since Windows epoch (1601-01-01 00:00:00)
    /// 
    /// # Examples
    /// ```rust
    /// use thetime::{System, Ntp, Time};
    /// println!("{} nanoseconds since Windows epoch", System::now().windows_ns());
    /// println!("{} nanoseconds since Windows epoch from pool.ntp.org", Ntp::now().windows_ns());
    /// ```
    fn windows_ns(&self) -> u64 {
        (self.epoch() * 1e4) as u64
    }

    /// Get the time in seconds since the Mac OS epoch (1904-01-01 00:00:00)
    ///
    /// # Examples
    /// ```rust
    /// use thetime::{System, Ntp, Time};
    /// println!("{} seconds since Mac OS epoch", System::now().mac_os());
    /// println!("{} seconds since Mac OS epoch from pool.ntp.org", Ntp::now().mac_os());
    /// ```
    fn mac_os(&self) -> u64 {
        self.unix() + 2082844800
    }

    /// Get the time in seconds since the Mac OS Absolute epoch (2001-01-01 00:00:00)
    ///
    /// # Examples
    /// ```rust
    /// use thetime::{System, Ntp, Time};
    /// println!("{} seconds since Mac OS Absolute epoch", System::now().mac_os_cfa());
    /// println!("{} seconds since Mac OS Absolute epoch from pool.ntp.org", Ntp::now().mac_os_cfa());
    /// ```
    fn mac_os_cfa(&self) -> u64 {
        self.unix() - 978307200
    }

    /// Get the time in seconds since the SAS 4GL epoch (1960-01-01 00:00:00)
    /// 
    /// # Examples
    /// ```rust
    /// use thetime::{System, Ntp, Time};
    /// println!("{} seconds since SAS 4GL epoch", System::now().sas_4gl());
    /// println!("{} seconds since SAS 4GL epoch from pool.ntp.org", Ntp::now().sas_4gl());
    /// ```
    fn sas_4gl(&self) -> u64 {
        self.unix() + 315619200
    }
    /// Format the time according to the given format string
    ///
    /// # Examples
    /// ```rust
    /// use thetime::{System, Ntp, Time};
    /// println!("{}", System::now().strftime("%Y-%m-%d %H:%M:%S"));
    /// println!("{}", Ntp::now().strftime("%Y-%B-%d %H:%M:%S"));
    /// ```
    fn strftime(&self, format: &str) -> String;

    /// Get the time since the epoch we use (1601-01-01 00:00:00). we use this for full compataibility with Windows
    /// 
    /// # Examples
    /// ```rust
    /// use thetime::{System, Ntp, Time};
    /// println!("{} milliseconds since the epoch we use", System::now().epoch());
    /// println!("{} milliseconds since the epoch we use from pool.ntp.org", Ntp::now().epoch());
    /// ```
    fn epoch(&self) -> f64 {
        self.unix_ms() + 11_644_473_600_000.0 as f64
    }

    /// Don't use this method, it's for internal use only
    fn from_epoch(timestamp: f64) -> Self;
}

/// Implements the diff functions (optional)
pub trait TimeDiff {
    /// Get the difference between two times in seconds
    ///
    /// # Examples
    /// ```rust
    /// use thetime::{System, Ntp, Time, TimeDiff, StrTime};
    /// let x = System::now();
    /// let y = "2017-01-01 00:00:00".parse_time::<System>("%Y-%m-%d %H:%M:%S");
    /// println!("{} seconds difference", x.diff(&y));
    /// ```
    fn diff<T: Time>(&self, other: &T) -> f64;

    /// Get the difference between two times in milliseconds
    ///
    /// # Examples
    /// ```rust
    /// use thetime::{System, Ntp, Time, TimeDiff};
    /// let x = System::now();
    /// let y = Ntp::now();
    /// println!("{} milliseconds difference", x.diff_ms(&y));
    /// ```
    fn diff_ms<T: Time>(&self, other: &T) -> f64;
}

/// Provides wrappers on string std types to parse into time structs
pub trait StrTime {
    /// Parse a string into a time struct of choice
    ///
    /// # Examples
    /// ```rust
    /// use thetime::{System, Ntp, Time, StrTime};
    /// println!("2017 - {}", "2017-01-01 00:00:00".parse_time::<System>("%Y-%m-%d %H:%M:%S"));
    fn parse_time<T: Time>(&self, format: &str) -> T
    where
        Self: std::fmt::Display,
    {
        T::strptime(self, format)
    }
}

/// Provides wrappers on integer std types to parse into time structs
/// 
/// Note: If there is an error, the function will return the Unix epoch time for the struct of choice
pub trait IntTime: std::fmt::Display + Into<u64> {
    /// Convert an integer into a time struct of choice
    ///
    /// # Examples
    /// ```rust
    /// use thetime::{System, Ntp, Time, IntTime};
    /// println!("2017 - {:#?}", 1483228800u64.from_unix::<System>());
    /// ```
    fn from_unix<T: Time>(self) -> T {
        let unix: u64 = self.into();
        T::from_epoch((unix + 11644473600)as f64)
    }

    /// Convert an integer into a time struct of choice, from a Windows timestamp (100ns since 1601-01-01 00:00:00)
    /// 
    /// # Examples
    /// ```rust
    /// use thetime::{System, Ntp, Time, IntTime};
    /// println!("2017 - {:#?}", 131297344000004217u64.from_windows_ns::<System>());
    /// ```
    fn from_windows_ns<T: Time>(self) -> T {
        T::from_epoch((self.into() / 10_000_000) as f64)
    }

    /// Convert an integer into a time struct of choice, from a Mac OS timestamp (seconds since 1904-01-01 00:00:00)
    /// 
    /// # Examples
    /// ```rust
    /// use thetime::{System, Ntp, Time, IntTime};
    /// println!("2024 - {:#?}", 3787228612u64.from_mac_os::<System>());
    /// ```
    fn from_mac_os<T: Time>(self) -> T {
        let selfu64: u64 = self.into();
        let selfi64: i64 = selfu64 as i64;
        if 2082844800 >= selfu64 {
            return T::strptime("0", "%s");
        }
        let unix: i64 = selfi64 - 2082844800;
        T::from_epoch((unix + 11644473600) as f64)
    }

    /// Convert an integer into a time struct of choice, from a Mac OS Absolute timestamp (seconds since 2001-01-01 00:00:00)
    /// 
    /// # Examples
    /// ```rust
    /// use thetime::{System, Ntp, Time, IntTime};
    /// println!("2024 - {:#?}", 726076923u64.from_mac_os_cfa::<System>());
    /// ```
    fn from_mac_os_cfa<T: Time>(self) -> T {
        let selfu64: u64 = self.into();
        let unix: i64 = (selfu64 as i64) + 978307200;
        T::from_epoch((unix + 11644473600) as f64)
    }

    /// Convert an integer into a time struct of choice, from a SAS 4GL timestamp (seconds since 1960-01-01 00:00:00)
    /// 
    /// # Examples
    /// ```rust
    /// use thetime::{System, Ntp, Time, IntTime};
    /// println!("2024 - {:#?}", 2020003754u64.from_sas_4gl::<System>());
    /// ```
    fn from_sas_4gl<T: Time>(self) -> T {
        let selfu64: u64 = self.into();
        let unix: i64 = (selfu64 as i64) - 315619200;
        T::from_epoch((unix + 11644473600) as f64)
    }
}

impl StrTime for str {}
impl StrTime for String {}
impl<T: std::fmt::Display + Into<u64>> IntTime for T {}



#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_system() {
        let x = System::now();
        println!("{:#?}", x);
        println!("{}", x.unix_ms());
        println!("{}", x.strftime("%Y-%m-%d %H:%M:%S"));
    }

    #[test]
    fn test_ntp() {
        let x = Ntp::now();
        println!("{:#?}", x);
        println!("{}", x.unix_ms());
        println!("{}", x.strftime("%Y-%m-%d %H:%M:%S"));
    }

    #[test]
    fn strptime() {
        let x = System::strptime("2015-02-18 23:16:09", "%Y-%m-%d %H:%M:%S");
        println!("2015 - {}", x);
        let x = Ntp::strptime("2021-01-01 00:00:00 +0000", "%Y-%m-%d %H:%M:%S %z");
        println!("2021 - {}", x);
    }

    #[test]
    fn str_time() {
        println!(
            "2017 - {}",
            "2017-01-01 00:00:00".parse_time::<System>("%Y-%m-%d %H:%M:%S")
        );
    }

    #[test]
    fn time_diff() {
        let x = System::now();
        let y = "2017-01-01 00:00:00".parse_time::<System>("%Y-%m-%d %H:%M:%S");
        println!("{} seconds difference", y.diff(&x));
        println!("{} milliseconds difference", x.diff_ms(&y));
    }

    #[test]
    fn int_time() {
        println!("2017 - {}", 1483228800u32.from_unix::<Ntp>());
        println!(
            "2017 - {}",
            131297344000004217u64.from_windows_ns::<Ntp>()
        );
        println!("2024 - {}", 3787228612u32.from_mac_os::<Ntp>());
        println!(
            "2024 - {}",
            726076923u32.from_mac_os_cfa::<Ntp>()
        );
        println!("1960 - {}", 0u32.from_sas_4gl::<Ntp>());
    }

    #[test]
    fn windows_tests() {
        let x = System::now();
        println!("{} nanoseconds since Windows epoch", x.windows_ns());
    }

    #[test]
    fn mac_os() {
        let x = System::now();
        println!("{} seconds since Mac OS epoch", x.mac_os());
        println!("{} seconds since Mac OS Absolute epoch", x.mac_os_cfa());
    }

    #[test]
    fn sas_4gl() {
        let x = System::now();
        println!("{} seconds since SAS 4GL epoch", x.sas_4gl());
    }
}
