use core::ops::Add;
use core::ops::AddAssign;

pub struct Duration(u64);

impl Duration {
    pub fn new(secs: u64, nanos: u32) -> Duration {
        let sec = secs * 1000_000_000u64 + nanos as u64;
        Duration(sec)
    }

    pub fn from_secs(secs: u64) -> Duration {
        let sec = secs * 1000_000_000u64;
        Duration(sec)
    }

    pub fn from_millis(millis: u64) -> Duration {
        let sec = millis * 1000_000u64;
        Duration(sec)
    }

    pub fn as_secs(&self) -> u64 {
        self.0 / 1000_000_000u64
    }

    pub fn subsec_nanos(&self) -> u64 {
        self.0 % 1000_000_000u64
    }
}

impl Add for Duration {
    type Output = Duration;

    fn add(self, other: Duration) -> Duration {
        Duration(self.0 + other.0)
    }
}

impl AddAssign for Duration {
    fn add_assign(&mut self, other: Duration) {
        self.0 = self.0 + other.0;
    }
}