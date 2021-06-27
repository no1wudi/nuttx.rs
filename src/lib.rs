#![no_std]

mod error;
mod sys;

#[macro_use]
mod macros;

#[allow(non_camel_case_types)]
#[allow(dead_code)]
mod libc;

pub mod io;
pub mod thread;
pub mod time;

#[cfg(not(target_arch="x86_64"))]
use core::panic::PanicInfo;

pub use core::format_args;

#[cfg(not(target_arch="x86_64"))]
#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    println!("{}", _panic);
    loop {}
}
