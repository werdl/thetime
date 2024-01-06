/// re-exported for easier access (no `use thetime::ntp::Ntp;`, just `use thetime::Ntp;`)
pub mod ntp;

/// re-exported for easier access (no `use thetime::system::System;`, just `use thetime::System;`)
pub mod system;

/// Timezones - a list of common timezones
/// Note: some names clash, examples Arabia Standard Time (AST) and Atlantic Standard Time (ATST), so we lengthen as shown above
/// 
/// # Examples
/// ```
/// use thetime::timezones;
/// println!("{}", timezones::UTC);
/// println!("{}", timezones::BST);
/// ```

pub mod timezones;


/// export the ntp file for easier access
pub use ntp::*;

/// export the system file for easier access
pub use system::*;

extern crate time;
/// Reference time
pub const REF_TIME_1970: u64 = 2208988800;

/// Offset between 1601 and 1970
pub const OFFSET_1601: u64 = 11644473600;

/// Magic number for SAS 4GL (offset betwenn 1960 and 1970)
pub const MAGIC_SAS_4GL: i64 = 315619200;

/// Magic number for Macos epoch (offset between 1904 and 1970)
pub const MAGIC_MAC_OS: i64 = 2082844800;

/// Magic number for Macos Absolute epoch (offset between 2001 and 1970)
pub const MAGIC_MAC_OS_CFA: i64 = 978307200;
/// Returns the current time in seconds since Unix epoch
///
/// # Examples
/// ```rust
/// use thetime::now;
/// println!("{} seconds since Unix epoch", now());
/// ```
pub fn now() -> i64 {
    Ntp::now().unix()
}

/// An enum to represent whether a time is in the past, present or future
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum RelativeTime {
    Past,
    Present,
    Future
}

impl core::fmt::Display for RelativeTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RelativeTime::Past => write!(f, "past"),
            RelativeTime::Present => write!(f, "present"),
            RelativeTime::Future => write!(f, "future"),
        }
    }
}

/// Implements the core functionality of the library
/// 
/// The conversion methods from struct to various timestamps do support negatives where needed (everything but `windows_ns` as it uses the same epoch as we do)
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
    fn unix(&self) -> i64;

    /// Get the time in milliseconds since Unix epoch
    ///
    /// # Examples
    /// ```rust
    /// use thetime::{System, Ntp, Time};
    /// println!("{} milliseconds since Unix epoch", System::now().unix_ms());
    /// println!("{} milliseconds since Unix epoch from pool.ntp.org", Ntp::now().unix_ms());
    /// ```
    fn unix_ms(&self) -> i64;

    /// Gets the time in nanoseconds (approximate) since Windows epoch (`1601-01-01 00:00:00`)
    ///
    /// # Examples
    /// ```rust
    /// use thetime::{System, Ntp, Time};
    /// println!("{} nanoseconds since Windows epoch", System::now().windows_ns());
    /// println!("{} nanoseconds since Windows epoch from pool.ntp.org", Ntp::now().windows_ns());
    /// ```
    fn windows_ns(&self) -> i64 {
        ((self.epoch() as f64) * 1e4) as i64
    }

    /// Gets the time in microseconds (approximate) since Webkit epoch (`1601-01-01 00:00:00`)
    /// 
    /// # Examples
    /// ```rust
    /// use thetime::{System, Ntp, Time};
    /// println!("{} microseconds since Webkit epoch", System::now().webkit());
    /// println!("{} microseconds since Webkit epoch from pool.ntp.org", Ntp::now().webkit());
    /// ```
    fn webkit(&self) -> i64 {
        ((self.epoch() as f64) * 1e3) as i64
    }

    /// Get the time in seconds since the Mac OS epoch (1904-01-01 00:00:00)
    ///
    /// # Examples
    /// ```rust
    /// use thetime::{System, Ntp, Time};
    /// println!("{} seconds since Mac OS epoch", System::now().mac_os());
    /// println!("{} seconds since Mac OS epoch from pool.ntp.org", Ntp::now().mac_os());
    /// ```
    fn mac_os(&self) -> i64 {
        self.unix() + MAGIC_MAC_OS
    }

    /// Get the time in seconds since the Mac OS Absolute epoch (2001-01-01 00:00:00)
    ///
    /// # Examples
    /// ```rust
    /// use thetime::{System, Ntp, Time};
    /// println!("{} seconds since Mac OS Absolute epoch", System::now().mac_os_cfa());
    /// println!("{} seconds since Mac OS Absolute epoch from pool.ntp.org", Ntp::now().mac_os_cfa());
    /// ```
    fn mac_os_cfa(&self) -> i64 {
        self.unix() - MAGIC_MAC_OS_CFA
    }

    /// Get the time in seconds since the SAS 4GL epoch (1960-01-01 00:00:00)
    ///
    /// # Examples
    /// ```rust
    /// use thetime::{System, Ntp, Time};
    /// println!("{} seconds since SAS 4GL epoch", System::now().sas_4gl());
    /// println!("{} seconds since SAS 4GL epoch from pool.ntp.org", Ntp::now().sas_4gl());
    /// ```
    fn sas_4gl(&self) -> i64 {
        self.unix() + MAGIC_SAS_4GL
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
    fn epoch(&self) -> i64 {
        self.unix_ms() + (OFFSET_1601 as i64 * 1000i64)
    }

    /// pretty print the time object
    ///
    /// # Examples
    /// ```rust
    /// use thetime::{System, Ntp, Time, StrTime};
    /// let date2017 = "2017-01-01 00:00:00".parse_time::<System>("%Y-%m-%d %H:%M:%S");
    /// println!("2017 - {}", date2017.pretty());
    /// assert_eq!(date2017.pretty(), "2017-01-01 00:00:00");
    /// ```
    fn pretty(&self) -> String {
        self.strftime("%Y-%m-%d %H:%M:%S")
    }

    /// Don't use this method, it's for internal use only (for instantiating structs from timestamps using the `1601-01-01 00:00:00` epoch)
    #[doc(hidden)]
    fn from_epoch(timestamp: u64) -> Self;

    /// Don't use this method, it's for internal use only (returns raw struct ms)
    #[doc(hidden)]
    fn raw(&self) -> u64;

    /// Returns the date formatted in ISO8601 format
    ///
    /// # Examples
    /// ```rust
    /// use thetime::{System, Ntp, Time};
    /// println!("{}", System::now().iso8601());
    /// println!("{}", Ntp::now().iso8601());
    /// ```
    fn iso8601(&self) -> String {
        self.strftime("%Y-%m-%d %H:%M:%S.") + &(self.raw() % 1000).to_string()
    }

    /// Returns the date formatted in RFC3339 format
    ///
    /// # Examples
    /// ```rust
    /// use thetime::{System, Ntp, Time};
    /// println!("{}", System::now().rfc3339());
    /// println!("{}", Ntp::now().rfc3339());
    /// ```
    fn rfc3339(&self) -> String {
        self.strftime("%Y-%m-%dT%H:%M:%S.") + &(self.raw() % 1000).to_string() + "Z"
    }

    /// internal only
    #[doc(hidden)]
    fn utc_offset(&self) -> i32;

    /// Gets the timezone offset in the format HH:MM
    /// 
    /// # Examples
    /// ```rust
    /// use thetime::{System, Ntp, Time};
    /// println!("{}", System::now().tz_offset());
    /// println!("{}", Ntp::now().tz_offset());
    /// ```
    fn tz_offset(&self) -> String {
        let offset = self.utc_offset();
        let sign = if offset < 0 { "-" } else { "+" };
        let offset = offset.abs();
        let hours = offset / 3600;
        let minutes = (offset % 3600) / 60;
        format!("{}{:02}:{:02}", sign, hours, minutes)
    }

    /// Changes the timezone offset of the time object, where `offset` is in the form "+|-[0-5][0-9]:[0-5][0-9]"
    /// Note that this change is relative to UTC, not the current timezone
    /// 
    /// # Examples
    /// ```rust
    /// use thetime::{System, Ntp, Time};
    /// println!("{}", System::now().change_tz("+01:00"));
    /// println!("{}", Ntp::now().change_tz("-01:00"));
    /// ```
    fn change_tz<T: ToString>(&self, offset: T) -> Self 
    where Self: Sized {
        let offset = offset.to_string();
        let offset_seconds = offset[1..3].parse::<i32>().unwrap() * 3600
            + offset[4..6].parse::<i32>().unwrap() * 60;

        let offset_seconds = if offset.starts_with('+') {
            offset_seconds
        } else {
            -offset_seconds
        };

        let duped_secs = offset_seconds;


        let utc_self = Self::from_epoch_offset((self.raw() as i64 + (self.utc_offset() as i64 * 1000i64)) as u64, 0);


        Self::from_epoch_offset((utc_self.raw() as i64 + (offset_seconds as i64 * 1000i64)) as u64, -duped_secs)
    }

    /// add an amount in seconds to a time object
    /// 
    /// # Examples
    /// ```rust
    /// use thetime::{System, Ntp, Time};
    /// println!("{}", System::now().add_seconds(3600));
    /// println!("{}", Ntp::now().add_seconds(3600));
    /// ```
    fn add_seconds(&self, duration: i64) -> Self
    where Self: Sized {
        Self::from_epoch((self.raw() as i64 + (duration * 1000)) as u64)
    }

    /// add an amount in minutes to a time object
    /// 
    /// # Examples
    /// ```rust
    /// use thetime::{System, Ntp, Time};
    /// println!("{}", System::now().add_minutes(60));
    /// println!("{}", Ntp::now().add_minutes(-60));
    /// ```
    fn add_minutes(&self, minutes: i64) -> Self
    where Self: Sized {
        self.add_seconds(minutes * 60)
    }

    /// add an amount in hours to a time object
    /// 
    /// # Examples
    /// ```rust
    /// use thetime::{System, Ntp, Time};
    /// println!("{}", System::now().add_minutes(60));
    /// println!("{}", Ntp::now().add_minutes(24));
    /// ```
    fn add_hours(&self, hours: i64) -> Self
    where Self: Sized {
        self.add_seconds(hours * 3600)
    }

    /// add an amount in days to a time object
    /// 
    /// # Examples
    /// ```rust
    /// use thetime::{System, Ntp, Time};
    /// println!("{}", System::now().add_days(7));
    /// println!("{}", Ntp::now().add_days(24));
    /// ```
    fn add_days(&self, days: i64) -> Self
    where Self: Sized {
        self.add_seconds(days * 86400)
    }

    /// add an amount in weeks to a time object
    /// we stop at weeks to avoid potential leap year confusion
    /// 
    /// # Examples
    /// ```rust
    /// use thetime::{System, Ntp, Time};
    /// println!("{}", System::now().add_weeks(7));
    /// println!("{}", Ntp::now().add_weeks(52));
    /// ```
    fn add_weeks(&self, weeks: i64) -> Self
    where Self: Sized {
        self.add_seconds(weeks * 604800)
    }


    /// determine whether a time object is in the past, present or future
    /// 
    /// # Examples
    /// ```rust
    /// use thetime::{System, Ntp, Time};
    /// let x = System::now();
    /// let y = Ntp::now();
    /// println!("{} is in the {}", x, x.past_future(&y));
    /// println!("{} is in the {}", y, y.past_future(&x));
    /// ```
    fn past_future<T: Time>(&self, other: &T) -> RelativeTime {
        if self.raw() < other.raw() {
            RelativeTime::Past
        } else if self.raw() > other.raw() {
            RelativeTime::Future
        } else {
            RelativeTime::Present
        }
    }

    /// add a duration to a time object
    /// 
    /// # Examples
    /// ```rust
    /// use thetime::{System, Ntp, Time, ImplsDuration};
    /// let x = System::now();
    /// println!("{}", x.add_duration(chrono::Duration::seconds(3600)));
    /// ```
    fn add_duration<T: ImplsDuration>(&self, duration: T) -> Self
        where Self: Sized {
        self.add_seconds(duration.num_seconds())
    }

    /// internal only
    #[doc(hidden)]
    fn from_epoch_offset(timestamp: u64, offset: i32) -> Self;
}

pub trait ImplsDuration {
    fn num_seconds(&self) -> i64;
}
impl ImplsDuration for chrono::Duration {
    fn num_seconds(&self) -> i64 {
        self.num_seconds()
    }
}
impl ImplsDuration for std::time::Duration {
    fn num_seconds(&self) -> i64 {
        self.as_secs() as i64
    }
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
    where
        Self: Time,
    {
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
    where
        Self: Time,
    {
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

    /// Parse a string into a time struct of choice, using the ISO8601 format
    ///
    /// # Examples
    /// ```rust
    /// use thetime::{System, Ntp, Time, StrTime};
    /// println!("2017 - {}", "2017-01-01T00:00:00.000".strp_iso8601::<System>());
    /// println!("{}", "2017-01-01T00:00:00.000".strp_iso8601::<System>().unix());
    /// assert_eq!("2017-01-01T00:00:00.000".strp_iso8601::<System>().unix(), 1483228800);
    /// ```
    fn strp_iso8601<T: Time>(&self) -> T
    where
        Self: std::fmt::Display,
    {
        T::strptime(self, "%Y-%m-%dT%H:%M:%S.%f")
    }

    /// Parse a string into a time struct of choice, using the RFC3339 format
    ///
    /// # Examples
    /// ```rust
    /// use thetime::{System, Ntp, Time, StrTime};
    /// println!("2017 - {}", "2017-01-01T00:00:00.000Z".strp_rf3339::<System>());
    /// println!("{}", "2017-01-01T00:00:00.000Z".strp_rf3339::<System>().unix());
    /// assert_eq!("2017-01-01T00:00:00.000Z".strp_rf3339::<System>().unix(), 1483228800);
    /// ```
    fn strp_rf3339<T: Time>(&self) -> T
    where
        Self: std::fmt::Display,
    {
        T::strptime(self, "%Y-%m-%dT%H:%M:%S.%fZ")
    }
}

/// Provides wrappers on integer std types to parse into time structs, and also to pretty print timestamp integers
///
/// Note: If there is an error, the function will return the Unix epoch time for the struct of choice
/// 
/// You can only convert from positive integers, as negative integers are not supported, as they cannot be represented in the time structs. While it would be possible to fix this, I don't think it is a needed feature at the moment.
pub trait IntTime: std::fmt::Display + Into<u64> {
    /// Convert an integer into a time struct of choice
    ///
    /// # Examples
    /// ```rust
    /// use thetime::{System, Ntp, Time, IntTime};
    /// assert_eq!(1483228800u32.unix::<Ntp>().pretty(), "2017-01-01 00:00:00");
    /// ```
    fn unix<T: Time>(self) -> T {
        let unix: u64 = self.into();
        T::from_epoch((unix + OFFSET_1601) * 1000)
    }

    /// Convert an integer into a time struct of choice, from a Windows timestamp (100ns since `1601-01-01 00:00:00`)
    ///
    /// # Examples
    /// ```rust
    /// use thetime::{System, Ntp, Time, IntTime};
    /// assert_eq!(131277024000000000u64.windows_ns::<Ntp>().pretty(),"2017-01-01 00:00:00");
    /// ```
    fn windows_ns<T: Time>(self) -> T {
        T::from_epoch(self.into() / (1e4 as u64))
    }

    /// Convert an integer into a time struct of choice, from a Webkit timestamp (microseconds since `1601-01-01 00:00:00`)
    /// 
    /// # Examples
    /// ```rust
    /// use thetime::{System, Ntp, Time, IntTime};
    /// println!("2017 - {:#?}", 13127702400000000u64.webkit::<System>());
    /// assert_eq!(13127702400000000u64.webkit::<System>().strftime("%Y-%m-%d %H:%M:%S"), "2017-01-01 00:00:00");
    /// ```
    fn webkit<T: Time>(self) -> T {
        T::from_epoch(self.into() / (1e3 as u64))
    }

    /// Convert an integer into a time struct of choice, from a Mac OS timestamp (seconds since 1904-01-01 00:00:00)
    ///
    /// # Examples
    /// ```rust
    /// use thetime::{System, Ntp, Time, IntTime};
    /// println!("2024 - {:#?}", 3787310789u64.mac_os::<System>());
    /// assert_eq!(3787310789u64.mac_os::<System>().strftime("%Y-%m-%d %H:%M:%S"), "2024-01-05 14:46:29");
    /// ```
    fn mac_os<T: Time>(self) -> T {
        let selfu64: u64 = self.into();
        let selfi64: i64 = selfu64 as i64;
        let unix: i64 = selfi64 - (MAGIC_MAC_OS as i64);
        T::from_epoch((unix + (OFFSET_1601 as i64)) as u64 * 1000)
    }

    /// Convert an integer into a time struct of choice, from a Mac OS Absolute timestamp (seconds since 2001-01-01 00:00:00)
    ///
    /// # Examples
    /// ```rust
    /// use thetime::{System, Ntp, Time, IntTime};
    /// println!("2024 - {:#?}", 726158877u64.mac_os_cfa::<System>());
    /// assert_eq!(726158877u64.mac_os_cfa::<System>().strftime("%Y-%m-%d %H:%M:%S"), "2024-01-05 14:47:57");
    /// ```
    fn mac_os_cfa<T: Time>(self) -> T {
        let selfu64: u64 = self.into();
        let unix: u64 = selfu64 + MAGIC_MAC_OS_CFA as u64;
        T::from_epoch((unix + OFFSET_1601) * 1000)
    }

    /// Convert an integer into a time struct of choice, from a SAS 4GL timestamp (seconds since 1960-01-01 00:00:00)
    ///
    /// # Examples
    /// ```rust
    /// use thetime::{System, Ntp, Time, IntTime};
    /// println!("2024 - {:#?}", 2020003754u64.sas_4gl::<System>());
    /// assert_eq!(2020003754u64.sas_4gl::<System>().strftime("%Y-%m-%d %H:%M:%S"), "2024-01-04 16:09:14");
    /// ```
    fn sas_4gl<T: Time>(self) -> T {
        let selfu64: u64 = self.into();
        let selfi64: i64 = selfu64 as i64;
        let unix: i64 = selfi64 - (MAGIC_SAS_4GL as i64);
        T::from_epoch((unix + (OFFSET_1601 as i64)) as u64 * 1000)
    }

    /// Prints the time duration in a formatted string. Note that this only goes up to weeks, as years are rather subjective
    ///
    /// # Examples
    /// ```rust
    /// use thetime::IntTime;
    /// let duration = 3600u64;
    /// let formatted = duration.ts_print();
    /// assert_eq!(formatted, "0w 0d 1h 0m 0s");
    /// ```
    fn ts_print(self) -> String {
        let duration = chrono::Duration::seconds(self.into() as i64);
        format!(
            "{}w {}d {}h {}m {}s",
            duration.num_weeks(),
            duration.num_days() % 7,
            duration.num_hours() % 24,
            duration.num_minutes() % 60,
            duration.num_seconds() % 60
        )
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
        println!("2017 - {}", date2017);
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
        assert_eq!(1483228800u32.unix::<Ntp>().pretty(), "2017-01-01 00:00:00");
        assert_eq!(
            131277024000000000u64.windows_ns::<Ntp>().pretty(),
            "2017-01-01 00:00:00"
        );
        assert_eq!(
            3787310789u64.mac_os::<Ntp>().pretty(),
            "2024-01-05 14:46:29"
        );
        assert_eq!(
            726158877u32.mac_os_cfa::<Ntp>().pretty(),
            "2024-01-05 14:47:57"
        );
        assert_eq!(0u32.sas_4gl::<Ntp>().pretty(), "1960-01-01 00:00:00");
    }

    #[test]
    fn windows_tests() {
        let x = System::now();
        println!("{} lots of 100ns since Windows epoch", x.windows_ns());
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

    #[test]
    fn rfc3339_iso8601() {
        let x = Ntp::now();
        println!("{}", x.iso8601());
        println!("{}", x.rfc3339());
    }

    #[test]
    fn strptime_rfc_and_iso() {
        let x = "2017-01-01T00:00:00.000".strp_iso8601::<System>();
        let y = "2017-01-01T00:00:00.000Z".strp_rf3339::<System>();
        assert_eq!(x.unix(), 1483228800);
        assert_eq!(y.unix(), 1483228800);
    }

    #[test]
    fn tz_tests() {
        let x = System::now();
        println!("{}", x.tz_offset());
        println!("{}", x);
        println!("{}", x.change_tz("+01:00"));
        println!("{}", x.change_tz("-01:00"));
    }

    #[test]
    fn test_add_seconds() {
        let x = System::now();
        println!("{}", x.add_seconds(3600));
    } 

    #[test]
    fn test_add_minshoursdaysweeks() {
        let x = System::now();
        println!("{}", x.add_minutes(60));
        println!("{}", x.add_hours(24));
        println!("{}", x.add_days(7));
        println!("{}", x.add_weeks(52));
    }

    #[test]
    fn long_ago_dates() {
        let x = System::from_epoch(0);
        println!("{}", x);
        println!("{}", x.unix());
        println!("{}", x.strftime("%Y-%m-%d %H:%M:%S"));
    }

    #[test]
    fn test_past_future() {
        let x = "2029-01-01T00:00:00.000".strp_iso8601::<System>();
        let y = Ntp::now();
        println!("{} is in the {}", x, x.past_future(&y));
        println!("{} is in the {}", y, y.past_future(&x));
    }

    #[test]
    fn test_add_duration() {
        let x = System::now();
        println!("{}", x.add_duration(std::time::Duration::from_secs(3600)));
        println!("{}", x.add_duration(chrono::Duration::seconds(3600)));
    }
}
