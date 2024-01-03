pub mod system;
pub mod ntp;

use system::System;
use ntp::Ntp;

trait Time {
    /// Get current time, returning the relevent struct
    /// 
    /// # Examples
    /// ```rust
    /// use thetime::{System, Ntp};
    /// println!("{} says the system, but {} says the server", System::now(), Ntp::now());
    /// ```
    fn now() -> Self;

    /// Get the time in seconds since Unix epoch
    /// 
    /// # Examples
    /// ```rust
    /// use thetime::{System, Ntp};
    /// println!("{} seconds since Unix epoch", System::now().unix());
    /// println!("{} seconds since Unix epoch from pool.ntp.org", Ntp::now().unix());
    /// ```
    fn unix(&self) -> u64;

    /// Get the time in milliseconds since Unix epoch
    /// 
    /// # Examples
    /// ```rust
    /// use thetime::{System, Ntp};
    /// println!("{} milliseconds since Unix epoch", System::now().unix_ms());
    /// println!("{} milliseconds since Unix epoch from pool.ntp.org", Ntp::now().unix_ms());
    /// ```
    fn unix_ms(&self) -> f64;

    /// Format the time according to the given format string
    /// 
    /// # Examples
    /// ```rust
    /// use thetime::{System, Ntp};
    /// println!("{}", System::now().strftime("%Y-%m-%d %H:%M:%S"));
    /// println!("{}", Ntp::now().strftime("%Y-%B-%d %H:%M:%S"));
    /// ```
    fn strftime(&self, format: &str) -> String;
}

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
}