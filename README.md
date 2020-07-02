# NuttX.rs

![Rust](https://github.com/no1wudi/nuttx.rs/workflows/Rust/badge.svg)

## Overview

A Rust std library like wrapper for NuttX.

It's built on NuttX, with mature hardware support and POSIX compatible API.

We can use it just like Rust std library:
```rust

#[macro_use]
extern crate nuttx_rs;

#[no_mangle]
pub fn main() {
    println!("Hello from Rust");
}

```