use crate::libc;
use crate::libc::c_int;

use crate::io;
use crate::io::{Error, ErrorKind};
use core::fmt;

#[derive(Debug)]
pub struct FileDesc {
    fd: c_int,
}

impl FileDesc {
    pub fn new(fd: c_int) -> FileDesc {
        FileDesc { fd }
    }
}

impl io::Read for FileDesc {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let ret: i32;
        unsafe {
            ret = libc::read(self.fd, buf.as_mut_ptr(), buf.len() as i32);
        };
        if ret < 0 {
            Err(io::Error::new(io::ErrorKind::InvalidInput, 0))
        } else {
            Ok(ret as usize)
        }
    }
}

impl io::Write for FileDesc {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let ret: i32;
        unsafe {
            ret = libc::write(self.fd, buf.as_ptr(), buf.len() as i32);
        };
        if ret < 0 {
            Err(io::Error::new(io::ErrorKind::InvalidInput, 0))
        } else {
            Ok(ret as usize)
        }
    }
    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }

    fn write_fmt(&mut self, fmt: fmt::Arguments<'_>) -> io::Result<()> {
        struct Adaptor<'a, T: ?Sized + 'a> {
            inner: &'a mut T,
            error: io::Result<()>,
        }

        impl<T: io::Write + ?Sized> fmt::Write for Adaptor<'_, T> {
            fn write_str(&mut self, s: &str) -> fmt::Result {
                match self.inner.write_all(s.as_bytes()) {
                    Ok(()) => Ok(()),
                    Err(e) => {
                        self.error = Err(e);
                        Err(fmt::Error)
                    }
                }
            }
        }

        let mut output = Adaptor {
            inner: self,
            error: Ok(()),
        };
        match fmt::write(&mut output, fmt) {
            Ok(()) => Ok(()),
            Err(..) => {
                // check if the error came from the underlying `Write` or not
                if output.error.is_err() {
                    output.error
                } else {
                    Err(Error::new(ErrorKind::Other, 0))
                }
            }
        }
    }
}
