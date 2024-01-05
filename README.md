# thetime
## Simple Rust library for time
- builds on top of std, chrono and time
- very simple and intuitive to use
- wraps some very useful functions that are usually buried deep in modules
- has extensive support for unusual epochs
## Examples
```rust
use thetime::now;
println!("{} seconds since Unix epoch", now());
```

```rust
use thetime::{System, Ntp, Time};
println!("{} says the system, but {} says the server", System::now(), Ntp::now());
```

```rust
use thetime::{System, Ntp, Time};
println!("The time was {}", System::strptime("2015-01-18 23:16:09", "%Y-%m-%d %H:%M:%S"));
```

```rust
use thetime::{System, Ntp, Time};
println!("{} seconds since Unix epoch", System::now().unix());
println!("{} seconds since Unix epoch from pool.ntp.org", Ntp::now().unix());
```

```rust
use thetime::{System, Ntp, Time};
println!("{} milliseconds since Unix epoch", System::now().unix_ms());
println!("{} milliseconds since Unix epoch from pool.ntp.org", Ntp::now().unix_ms());
```

```rust
use thetime::{System, Ntp, Time};
println!("{} nanoseconds since Windows epoch", System::now().windows_ns());
println!("{} nanoseconds since Windows epoch from pool.ntp.org", Ntp::now().windows_ns());
```

```rust
use thetime::{System, Ntp, Time};
println!("{} microseconds since Webkit epoch", System::now().webkit());
println!("{} microseconds since Webkit epoch from pool.ntp.org", Ntp::now().webkit());
```

```rust
use thetime::{System, Ntp, Time};
println!("{} seconds since Mac OS epoch", System::now().mac_os());
println!("{} seconds since Mac OS epoch from pool.ntp.org", Ntp::now().mac_os());
```

```rust
use thetime::{System, Ntp, Time};
println!("{} seconds since Mac OS Absolute epoch", System::now().mac_os_cfa());
println!("{} seconds since Mac OS Absolute epoch from pool.ntp.org", Ntp::now().mac_os_cfa());
```

```rust
use thetime::{System, Ntp, Time};
println!("{} seconds since SAS 4GL epoch", System::now().sas_4gl());
println!("{} seconds since SAS 4GL epoch from pool.ntp.org", Ntp::now().sas_4gl());
```

```rust
use thetime::{System, Ntp, Time};
println!("{}", System::now().strftime("%Y-%m-%d %H:%M:%S"));
println!("{}", Ntp::now().strftime("%Y-%B-%d %H:%M:%S"));
```

```rust
use thetime::{System, Ntp, Time};
println!("{} milliseconds since the epoch we use", System::now().epoch());
println!("{} milliseconds since the epoch we use from pool.ntp.org", Ntp::now().epoch());
```

```rust
use thetime::{System, Ntp, Time, StrTime};
let date2017 = "2017-01-01 00:00:00".parse_time::<System>("%Y-%m-%d %H:%M:%S");
println!("2017 - {}", date2017.pretty());
assert_eq!(date2017.pretty(), "2017-01-01 00:00:00");
```

```rust
use thetime::{System, Ntp, Time};
println!("{}", System::now().iso8601());
println!("{}", Ntp::now().iso8601());
```

```rust
use thetime::{System, Ntp, Time};
println!("{}", System::now().rfc3339());
println!("{}", Ntp::now().rfc3339());
```

```rust
use thetime::{System, Ntp, Time, TimeDiff, StrTime, IntTime};
let x = "2018-01-01 00:00:00".parse_time::<System>("%Y-%m-%d %H:%M:%S");
let y = "2017-01-01 00:00:00".parse_time::<System>("%Y-%m-%d %H:%M:%S");
println!("{} seconds difference", x.diff(&y));
assert_eq!(x.diff(&y), 31536000u64);
```

```rust
use thetime::{System, Ntp, Time, TimeDiff};
let x = System::now();
let y = Ntp::now();
println!("{} milliseconds difference", x.diff_ms(&y));
```

```rust
use thetime::{System, Ntp, Time, StrTime};
println!("2017 - {}", "2017-01-01 00:00:00".parse_time::<System>("%Y-%m-%d %H:%M:%S"));
println!("{}", "2017-01-01 00:00:00".parse_time::<System>("%Y-%m-%d %H:%M:%S").unix());
assert_eq!("2017-01-01 00:00:00".parse_time::<System>("%Y-%m-%d %H:%M:%S").unix(), 1483228800);
```

```rust
use thetime::{System, Ntp, Time, StrTime};
println!("2017 - {}", "2017-01-01T00:00:00.000".strp_iso8601::<System>());
println!("{}", "2017-01-01T00:00:00.000".strp_iso8601::<System>().unix());
assert_eq!("2017-01-01T00:00:00.000".strp_iso8601::<System>().unix(), 1483228800);
```

```rust
use thetime::{System, Ntp, Time, StrTime};
println!("2017 - {}", "2017-01-01T00:00:00.000Z".strp_rf3339::<System>());
println!("{}", "2017-01-01T00:00:00.000Z".strp_rf3339::<System>().unix());
assert_eq!("2017-01-01T00:00:00.000Z".strp_rf3339::<System>().unix(), 1483228800);
```

```rust
use thetime::{System, Ntp, Time, IntTime};
let actual = format!("2017 - {:?}", 1483228800u64.unix::<System>());
assert_eq!(actual, "2017 - System { inner_secs: 13127702400, inner_milliseconds: 400 }");
```

```rust
use thetime::{System, Ntp, Time, IntTime};
println!("2017 - {:#?}", 13127702400000000u64.windows_ns::<System>());
assert_eq!(131277024000000000u64.windows_ns::<System>().strftime("%Y-%m-%d %H:%M:%S"), "2017-01-01 00:00:00");
```

```rust
use thetime::{System, Ntp, Time, IntTime};
println!("2017 - {:#?}", 13127702400000000u64.webkit::<System>());
assert_eq!(13127702400000000u64.webkit::<System>().strftime("%Y-%m-%d %H:%M:%S"), "2017-01-01 00:00:00");
```

```rust
use thetime::{System, Ntp, Time, IntTime};
println!("2024 - {:#?}", 3787310789u64.mac_os::<System>());
assert_eq!(3787310789u64.mac_os::<System>().strftime("%Y-%m-%d %H:%M:%S"), "2024-01-05 14:46:29");
```

```rust
use thetime::{System, Ntp, Time, IntTime};
println!("2024 - {:#?}", 726158877u64.mac_os_cfa::<System>());
assert_eq!(726158877u64.mac_os_cfa::<System>().strftime("%Y-%m-%d %H:%M:%S"), "2024-01-05 14:47:57");
```

```rust
use thetime::{System, Ntp, Time, IntTime};
println!("2024 - {:#?}", 2020003754u64.sas_4gl::<System>());
assert_eq!(2020003754u64.sas_4gl::<System>().strftime("%Y-%m-%d %H:%M:%S"), "2024-01-04 16:09:14");
```

```rust
use thetime::IntTime;
let duration = 3600u64;
let formatted = duration.ts_print();
assert_eq!(formatted, "0w 0d 1h 0m 0s");
```
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