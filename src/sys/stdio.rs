use crate::io;
use crate::libc;
use crate::sys::fd::FileDesc;
use core::fmt;
use core::mem::ManuallyDrop;

pub struct Stdin(());
pub struct Stdout(());
pub struct Stderr(());

impl Stdin {
    pub fn new() -> io::Result<Stdin> {
        Ok(Stdin(()))
    }
}

impl io::Read for Stdin {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        ManuallyDrop::new(FileDesc::new(libc::STDIN_FILENO)).read(buf)
    }
}

impl Stdout {
    pub fn new() -> io::Result<Stdout> {
        Ok(Stdout(()))
    }
}

impl io::Write for Stdout {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        ManuallyDrop::new(FileDesc::new(libc::STDOUT_FILENO)).write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
    fn write_fmt(&mut self, fmt: fmt::Arguments<'_>) -> io::Result<()> {
        ManuallyDrop::new(FileDesc::new(libc::STDOUT_FILENO)).write_fmt(fmt)
    }
}

impl Stderr {
    pub fn new() -> io::Result<Stderr> {
        Ok(Stderr(()))
    }
}

impl io::Write for Stderr {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        ManuallyDrop::new(FileDesc::new(libc::STDERR_FILENO)).write(buf)
    }
    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
    fn write_fmt(&mut self, fmt: fmt::Arguments<'_>) -> io::Result<()> {
        ManuallyDrop::new(FileDesc::new(libc::STDOUT_FILENO)).write_fmt(fmt)
    }
}
