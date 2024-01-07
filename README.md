# thetime
[![GitHub Actions][gh-image]][gh-checks]
[![crates.io][cratesio-image]][cratesio]
[![thetime on docs.rs][docsrs-image]][docsrs]

[gh-image]: https://github.com/werdl/thetime/actions/workflows/rust.yml/badge.svg
[gh-checks]: https://github.com/werdl/thetime/actions?query=workflow%20rust
[cratesio-image]: https://img.shields.io/crates/v/thetime.svg
[cratesio]: https://crates.io/crates/thetime
[docsrs-image]: https://docs.rs/thetime/badge.svg
[docsrs]: https://docs.rs/thetime
## Simple Rust library for time
- builds on top of std (almost exclusively `core`), chrono and time
- very simple and intuitive to use
- wraps some very useful functions that are usually buried deep in modules
- has extensive support for unusual epochs
- Fully supports without overflow, as the core data is stored as `u64`s, times since 01-01-1601, and up to, in my testing, "a+262143-01-01

## Features
### ntp
- Default: `true`
- includes: `Ntp` struct
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
use thetime::Tz;
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
