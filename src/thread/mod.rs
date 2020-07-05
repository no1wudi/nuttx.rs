use crate::libc;
use crate::time::Duration;

pub fn sleep(dur: Duration) {
    unsafe {
        libc::usleep(((dur.as_secs() * 1000_000_000 + dur.subsec_nanos()) / 1000) as i32);
    }
}

pub fn yield_now() {
    unsafe {
        libc::pthread_yield();
    }
}
