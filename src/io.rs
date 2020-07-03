use crate::syscall;

pub fn reader() {
    unsafe {
        syscall::write(1, 0 as *const u8, 0);
    }
}
