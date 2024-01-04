# thetime
## Simple Rust library for time
- builds on top of std, chrono and time
- very simple and intuitive to use
## Examples
```rust
use thetime::{System, Time};
println!("It has been {} seconds since 1 Jan 1970", System::now().unix());
```
