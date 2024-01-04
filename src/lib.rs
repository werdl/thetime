pub mod system;
pub mod ntp;

pub use system::*;
pub use ntp::*;

extern crate time;

pub fn now() -> u64 {
    Ntp::now().local().unix()
}

/// Implements the core functionality of the library
pub trait Time {
    /// Get current time, returning the relevent struct
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

    /// Format the time according to the given format string
    /// 
    /// # Examples
    /// ```rust
    /// use thetime::{System, Ntp, Time};
    /// println!("{}", System::now().strftime("%Y-%m-%d %H:%M:%S"));
    /// println!("{}", Ntp::now().strftime("%Y-%B-%d %H:%M:%S"));
    /// ```
    fn strftime(&self, format: &str) -> String;
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
    where Self: std::fmt::Display
    {
        T::strptime(self, format)
    }
}

/// Provides wrappers on integer std types to convert into time structs
pub trait IntTime: std::fmt::Display {
    /// Convert an integer into a time struct of choice
    /// 
    /// # Examples
    /// ```rust
    /// use thetime::{System, Ntp, Time, IntTime};
    /// println!("2017 - {:#?}", 1483228800.from_unix::<System>());
    /// ```
    fn from_unix<T: Time>(&self) -> T 
    where Self: std::fmt::Display {
        T::strptime(self.to_string(), "%s")
    }
}

impl StrTime for str {}
impl StrTime for String {}
impl<T: std::fmt::Display> IntTime for T {}



#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_system() {
        let x = System::now();
        println!("{}", x);
        let x = System::now();
        println!("{}", x.unix_ms());
        println!("{}", x.strftime("%Y-%m-%d %H:%M:%S"));
    }

    #[test]
    fn test_ntp() {
        let x = Ntp::now();
        println!("{}", x);
        let x = Ntp::now();
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
        println!("2017 - {}", "2017-01-01 00:00:00".parse_time::<System>("%Y-%m-%d %H:%M:%S"));
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
        println!("2017 - {:#?}", 1483228800.from_unix::<System>());
    }
}