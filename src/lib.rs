pub mod ntp;
pub mod system;

/// export the ntp file for easier access
pub use ntp::*;

/// export the system file for easier access
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
    fn unix_ms(&self) -> u64;

    /// Gets the time in nanoseconds (approximate) since Windows epoch (`1601-01-01 00:00:00`)
    /// 
    /// # Examples
    /// ```rust
    /// use thetime::{System, Ntp, Time};
    /// println!("{} nanoseconds since Windows epoch", System::now().windows_ns());
    /// println!("{} nanoseconds since Windows epoch from pool.ntp.org", Ntp::now().windows_ns());
    /// ```
    fn windows_ns(&self) -> u64 {
        ((self.epoch() as f64) * 1e4) as u64
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

    /// Get the time since the epoch we use (`1601-01-01 00:00:00`). we use this for full compataibility with Windows
    /// 
    /// # Examples
    /// ```rust
    /// use thetime::{System, Ntp, Time};
    /// println!("{} milliseconds since the epoch we use", System::now().epoch());
    /// println!("{} milliseconds since the epoch we use from pool.ntp.org", Ntp::now().epoch());
    /// ```
    fn epoch(&self) -> u64 {
        self.unix_ms() + 11_644_473_600_000
    }

    /// pretty print the time object
    /// 
    /// # Examples
    /// ```rust
    /// use thetime::{System, Ntp, Time};
    /// let date2017 = "2017-01-01 00:00:00".parse_time::<System>("%Y-%m-%d %H:%M:%S");
    /// println!("2017 - {}", date2017.pretty());
    /// assert_eq!(date2017.pretty(), "2017-01-01 00:00:00");
    /// ```
    fn pretty(&self) -> String {
        self.strftime("%Y-%m-%d %H:%M:%S")
    }

    /// Don't use this method, it's for internal use only (for instantiating structs from timestamps using the `1601-01-01 00:00:00` epoch)
    fn from_epoch(timestamp: u64) -> Self;

    /// Don't use this method, it's for internal use only (returns raw struct ms)
    fn raw(&self) -> u64;
}

/// Implements the diff functions (optional)
pub trait TimeDiff {
    /// Get the difference between two times in seconds
    ///
    /// # Examples
    /// ```rust
    /// use thetime::{System, Ntp, Time, TimeDiff, StrTime, IntTime};
    /// let x = "2018-01-01 00:00:00".parse_time::<System>("%Y-%m-%d %H:%M:%S");
    /// let y = "2017-01-01 00:00:00".parse_time::<System>("%Y-%m-%d %H:%M:%S");
    /// println!("{} seconds difference", x.diff(&y));
    /// assert_eq!(x.diff(&y), 31536000u64);
    /// ```
    fn diff<T: Time>(&self, other: &T) -> u64 
    where Self: Time {
        self.raw().abs_diff(other.raw()) / 1000
    }

    /// Get the difference between two times in milliseconds
    ///
    /// # Examples
    /// ```rust
    /// use thetime::{System, Ntp, Time, TimeDiff};
    /// let x = System::now();
    /// let y = Ntp::now();
    /// println!("{} milliseconds difference", x.diff_ms(&y));
    /// ```
    fn diff_ms<T: Time>(&self, other: &T) -> u64
    where Self: Time {
        self.raw().abs_diff(other.raw())
    }
}

/// Provides wrappers on string std types to parse into time structs
pub trait StrTime {
    /// Parse a string into a time struct of choice
    ///
    /// # Examples
    /// ```rust
    /// use thetime::{System, Ntp, Time, StrTime};
    /// println!("2017 - {}", "2017-01-01 00:00:00".parse_time::<System>("%Y-%m-%d %H:%M:%S"));
    /// println!("{}", "2017-01-01 00:00:00".parse_time::<System>("%Y-%m-%d %H:%M:%S").unix());
    /// assert_eq!("2017-01-01 00:00:00".parse_time::<System>("%Y-%m-%d %H:%M:%S").unix(), 1483228800);
    /// ```
    fn parse_time<T: Time>(&self, format: &str) -> T
    where
        Self: std::fmt::Display,
    {
        T::strptime(self, format)
    }
}

/// Provides wrappers on integer std types to parse into time structs, and also to pretty print timestamp integers
/// 
/// Note: If there is an error, the function will return the Unix epoch time for the struct of choice
pub trait IntTime: std::fmt::Display + Into<u64> {
    /// Convert an integer into a time struct of choice
    ///
    /// # Examples
    /// ```rust
    /// use thetime::{System, Ntp, Time, IntTime};
    /// let actual = format!("2017 - {:?}", 1483228800u64.from_unix::<System>());
    /// assert_eq!(actual, "2017 - System { inner_secs: 13127702400, inner_milliseconds: 400 }");
    /// ```
    fn from_unix<T: Time>(self) -> T {
        let unix: u64 = self.into();
        T::from_epoch(unix + 11644473600)
    }

    /// Convert an integer into a time struct of choice, from a Windows timestamp (100ns since `1601-01-01 00:00:00`)
    /// 
    /// # Examples
    /// ```rust
    /// use thetime::{System, Ntp, Time, IntTime};
    /// println!("2017 - {:#?}", 131277024000000000u64.from_windows_ns::<System>());
    /// assert_eq!(131277024000000000u64.from_windows_ns::<System>().strftime("%Y-%m-%d %H:%M:%S"), "2017-01-01 00:00:00");
    /// ```
    fn from_windows_ns<T: Time>(self) -> T {
        T::from_epoch(self.into() / 10_000_000)
    }

    /// Convert an integer into a time struct of choice, from a Mac OS timestamp (seconds since 1904-01-01 00:00:00)
    /// 
    /// # Examples
    /// ```rust
    /// use thetime::{System, Ntp, Time, IntTime};
    /// println!("2024 - {:#?}", 3787310789u64.from_mac_os::<System>());
    /// assert_eq!(3787310789u64.from_mac_os::<System>().strftime("%Y-%m-%d %H:%M:%S"), "2024-01-05 14:46:29");
    /// ```
    fn from_mac_os<T: Time>(self) -> T {
        let selfu64: u64 = self.into();
        let selfi64: i64 = selfu64 as i64;
        if 2082844800 >= selfu64 {
            return T::strptime("0", "%s");
        }
        let unix: i64 = selfi64 - 2082844800;
        T::from_epoch((unix + 11644473600) as u64)
    }

    /// Convert an integer into a time struct of choice, from a Mac OS Absolute timestamp (seconds since 2001-01-01 00:00:00)
    /// 
    /// # Examples
    /// ```rust
    /// use thetime::{System, Ntp, Time, IntTime};
    /// println!("2024 - {:#?}", 726158877u64.from_mac_os_cfa::<System>());
    /// assert_eq!(726158877u64.from_mac_os_cfa::<System>().strftime("%Y-%m-%d %H:%M:%S"), "2024-01-05 14:47:57");
    /// ```
    fn from_mac_os_cfa<T: Time>(self) -> T {
        let selfu64: u64 = self.into();
        let unix: i64 = (selfu64 as i64) + 978307200;
        T::from_epoch((unix + 11644473600) as u64)
    }

    /// Convert an integer into a time struct of choice, from a SAS 4GL timestamp (seconds since 1960-01-01 00:00:00)
    /// 
    /// # Examples
    /// ```rust
    /// use thetime::{System, Ntp, Time, IntTime};
    /// println!("2024 - {:#?}", 2020003754u64.from_sas_4gl::<System>());
    /// assert_eq!(2020003754u64.from_sas_4gl::<System>().strftime("%Y-%m-%d %H:%M:%S"), "2024-01-04");
    /// ```
    fn from_sas_4gl<T: Time>(self) -> T {
        let selfu64: u64 = self.into();
        let unix: i64 = (selfu64 as i64) - 315619200;
        T::from_epoch((unix + 11644473600) as u64)
    }

    /// Prints the time duration in a formatted string. Note that this only goes up to weeks, as years are rather subjective
    ///
    /// # Examples
    ///
    /// ```
    /// use thetime::IntTime;
    ///
    /// let duration = 3600u64;
    /// let formatted = duration.ts_print();
    /// assert_eq!(formatted, "0w 0d 1h 0m 0s");
    /// ```
    fn ts_print(self) -> String {
        let duration = chrono::Duration::seconds(self.into() as i64);
        format!("{}w {}d {}h {}m {}s", duration.num_weeks(), duration.num_days() % 7, duration.num_hours() % 24, duration.num_minutes() % 60, duration.num_seconds() % 60)
    }
}

/// implement the StrTime trait for `String` types
impl StrTime for str {}

/// implement the StrTime trait for `&str` types
impl StrTime for String {}

/// implement the IntTime trait for all integer types that implement conversion to u64
impl<T: std::fmt::Display + Into<u64>> IntTime for T {}



#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_system() {
        let x = System::now();
        println!("{}", x);
        println!("{}", x.unix_ms());
        println!("{}", x.strftime("%Y-%m-%d %H:%M:%S"));
    }

    #[test]
    fn test_ntp() {
        let x = Ntp::now();
        println!("{}", x);
        println!("{}", x.unix_ms());
        println!("{}", x.strftime("%Y-%m-%d %H:%M:%S"));
    }

    #[test]
    fn strptime() {
        let x = System::strptime("2015-02-18 23:16:09", "%Y-%m-%d %H:%M:%S");
        println!("2015 - {}", x);
        let x = Ntp::strptime("2021-01-01 00:00:00 +0000", "%Y-%m-%d %H:%M:%S %z");
        println!("2021 - {}", x);
        assert_eq!(x.unix(), 1609459200);
    }

    #[test]
    fn str_time() {
        let date2017 = "2017-01-01 00:00:00".parse_time::<System>("%Y-%m-%d %H:%M:%S");
        println!(
            "2017 - {}",
            date2017
        );
        assert_eq!(date2017.unix(), 1483228800);
    }

    #[test]
    fn time_diff() {
        let x = "2015-01-01 00:00:00".parse_time::<System>("%Y-%m-%d %H:%M:%S");
        let y = "2017-01-01 00:00:00".parse_time::<System>("%Y-%m-%d %H:%M:%S");
        println!("{} difference", y.diff(&x).ts_print());
        println!("{} milliseconds difference", x.diff_ms(&y));
        assert_eq!(x.diff(&y), 63158400u64);
    }

    #[test]
    fn int_time() {
        assert_eq!(1483228800u32.from_unix::<Ntp>().pretty(), "2017-01-01 00:00:00");
        assert_eq!(
            131277024000000000u64.from_windows_ns::<Ntp>().pretty(),
            "2017-01-01 00:00:00"

        );
        assert_eq!(3787228612u32.from_mac_os::<Ntp>().pretty(), "2024-01-04 15:56:52");
        assert_eq!(726158877u32.from_mac_os_cfa::<Ntp>().pretty(),"2024-01-05 14:47:57");
        assert_eq!(0u32.from_sas_4gl::<Ntp>().pretty(), "1960-01-01 00:00:00");
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
