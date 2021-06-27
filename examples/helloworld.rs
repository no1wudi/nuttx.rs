#![no_std]
#![no_main]

#[macro_use]
extern crate nuttx_rs;

#[no_mangle]
pub fn main() {
    println!("Hello World from Rust");
}
