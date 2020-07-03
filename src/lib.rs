#![no_std]

#[allow(dead_code)]
mod syscall;

pub mod io;

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    loop {}
}
