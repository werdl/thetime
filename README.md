# thetime
## Simple Rust library for time
- builds on top of std and chrono
- very simple and intuitive to use
## Examples
```rust
use thetime::System;
println!("It has been {} seconds since 1 Jan 1970", System::now().unix());
```