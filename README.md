# thetime
## Simple Rust library for time
- builds on top of std, chrono and time
- very simple and intuitive to use
- wraps some very useful functions that are usually buried deep in modules
- has extensive support for unusual epochs
## Which traits you need
```rust
// Basic functionality
use thetime::{System, Ntp, Time};

// Diff functions
use thetime::{System, Ntp, Time, TimeDiff};

// String direct strptime
use thetime::{System, Ntp, Time, StrTime};

// Timestamp int conversion
use thetime::{System, Ntp, Time, IntTime}

// Timezones
use thetime::timezones;
```
## Utilities provided
- full docs at [docs.rs/thetime](https://docs.rs/thetime)
### List
- NTP server pinging
- System time grabbing
- time diff functions
- string to time structs
- timestamps as integers to time structs
- strptime and strftime
- convienent `now` method in the root for easy access
- various epochs
> - 01-01-1904 (MacOS)
> - 01-01-2001 (MacOS Absolute)
> - 01-01-1601 (Windows, measured in 100ns chunks)
> - 01-01-1960 (SAS 4GL)
> - 01-01-1601 (Webkit, measured in μs)
- Fully supports without overflow, as the core data is stored as seconds and milliseconds since 01-01-1601