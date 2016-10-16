# rust-stopwatch
This is a simple module used to time things in Rust.

[![Build Status](https://travis-ci.org/ellisonch/rust-stopwatch.svg?branch=master)](https://travis-ci.org/ellisonch/rust-stopwatch)

## Usage
To use, add the following line to `Cargo.toml` under `[dependencies]`:
```toml

stopwatch = "0.0.7"
```
or alternatively,
```toml
stopwatch = { git = "https://github.com/ellisonch/rust-stopwatch.git" }
```

## Example
```rust
extern crate stopwatch;
use stopwatch::{Stopwatch};
// ...
let sw = Stopwatch::start_new();
// do something that takes some time
println!("Thing took {}ms", sw.elapsed_ms());
```

## Methods
```rust
fn new() -> Stopwatch
fn start_new() -> Stopwatch
fn start(&mut self)
fn stop(&mut self)
fn reset(&mut self)
fn restart(&mut self)
fn is_running(&self) -> bool
fn elapsed(&self) -> Duration
fn elapsed_ms(&self) -> i64
```
