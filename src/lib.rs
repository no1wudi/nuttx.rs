#![no_std]

mod error;
mod sys;

#[macro_use]
mod macros;

#[allow(non_camel_case_types)]
#[allow(dead_code)]
mod libc;

pub mod io;

use core::panic::PanicInfo;

pub use core::format_args;

#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    println!("{}", _panic);
    loop {}
}
