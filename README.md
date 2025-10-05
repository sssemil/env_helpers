# env_helpers

[![Crates.io](https://img.shields.io/crates/v/env_helpers?style=flat-square)](https://crates.io/crates/env_helpers)

A basic crate for working with env variables in a typed manner.

Example:
```rust
use env_helpers::get_env;
use std::path::PathBuf;

fn main() {
    let home_path: PathBuf = get_env("HOME");
    println!("{:?}", home_path);
}
```