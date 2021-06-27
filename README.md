# NuttX.rs

![Rust](https://github.com/no1wudi/nuttx.rs/workflows/Rust/badge.svg)

## Overview

A Rust std library like wrapper for NuttX.

It's built on NuttX, with mature hardware support and POSIX compatible API.

We can use it just like Rust std library:
```rust
#![no_std]
#![no_main]

#[macro_use]
extern crate nuttx_rs;

#[no_mangle]
pub fn main() {
    println!("Hello from Rust");
}

```

## Build

First, you should setup NuttX's develop enviroment, and set the task entry to
main (or other you preferred), and then set the enviroment vairable:

* `NUTTX_SRC_DIR`
* `NUTTX_BOARD_DIR`
* `NUTTX_BOARD_LD`

```bash
export NUTTX_SRC_DIR=/path/to/nuttx
export NUTTX_BOARD_DIR=/path/to/nuttx/boards/xxx (stm32f4discovery by default)
export NUTTX_BOARD_LD=ld.script (by default)
```

Add dependencies to your `Cargo.toml`:
```toml
[dependencies]
nuttx_rs = { git = "https://github.com/no1wudi/nuttx.rs.git" }
```

And in your application project, add build target in `.cargo/config.toml`:
```toml
[build]
rustflags = ["-C", "link-arg=-Tlink.x"]
```

The link script `link.x` you can get in NuttX's board config dir.
