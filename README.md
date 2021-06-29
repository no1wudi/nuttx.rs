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

## Requirement

On ubuntu, install NuttX build dependencies by this command:
```bash
sudo apt install gcc-arm-none-eabi kconfig-frontends
```

And the glone nuttx and apps into your work space, like:

* ~/work
    * nx
        * apps
        * nuttx

## Build

First, you should setup NuttX's develop enviroment, and set the task entry to
main (or other you preferred), and then set the enviroment vairable:

* `NUTTX_SRC_DIR`
* `NUTTX_BOARD_DIR`
* `NUTTX_BOARD_LD`

```bash
export NUTTX_SRC_DIR=/path/to/nuttx (e.g. ~/work/nx/nuttx)
export NUTTX_BOARD_DIR=nuttx/boards/xxx (stm32f4discovery by default)
export NUTTX_BOARD_LD=ld.script (by default, in boards/scripts)
```

Add dependencies to your `Cargo.toml`:
```toml
[dependencies]
nuttx_rs = { git = "https://github.com/no1wudi/nuttx.rs.git" }
```

And in your application project, add build target in `.cargo/config.toml`:
```toml
[build]
rustflags = ["-C", "link-arg=-Tlink.ld"]
```

The link script `link.ld` you can get in NuttX's board config dir.
