pub const STDIN_FILENO: i32 = 0;
pub const STDOUT_FILENO: i32 = 1;
pub const STDERR_FILENO: i32 = 2;

extern "C" {
    pub fn write(fd: i32, buf: *const u8, nbytes: i32);
    pub fn read(fd: i32, buf: *mut u8, nbytes: i32);
}
