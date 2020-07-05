use crate::error;
use core::fmt;
use core::result;

pub type Result<T> = result::Result<T, Error>;

pub struct Error {
    repr: Repr,
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.repr, f)
    }
}

enum Repr {
    Os(i32),
    Simple(ErrorKind),
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ErrorKind {
    NoError,
    /// An entity was not found, often a file.
    NotFound,
    /// The operation lacked the necessary privileges to complete.
    PermissionDenied,
    /// The connection was refused by the remote server.
    ConnectionRefused,
    /// The connection was reset by the remote server.
    ConnectionReset,
    /// The connection was aborted (terminated) by the remote server.
    ConnectionAborted,
    /// The network operation failed because it was not connected yet.
    NotConnected,
    /// A socket address could not be bound because the address is already in
    /// use elsewhere.
    AddrInUse,
    /// A nonexistent interface was requested or the requested address was not
    /// local.
    AddrNotAvailable,
    /// The operation failed because a pipe was closed.
    BrokenPipe,
    /// An entity already exists, often a file.
    AlreadyExists,
    /// The operation needs to block to complete, but the blocking operation was
    /// requested to not occur.
    WouldBlock,
    /// A parameter was incorrect.
    InvalidInput,
    /// Data not valid for the operation were encountered.
    InvalidData,
    /// The I/O operation's timeout expired, causing it to be canceled.
    TimedOut,
    /// An error returned when an operation could not be completed because a
    /// call to [`write`] returned [`Ok(0)`].
    WriteZero,
    /// This operation was interrupted.
    ///
    /// Interrupted operations can typically be retried.
    Interrupted,
    /// Any I/O error not part of this list.
    Other,
    /// An error returned when an operation could not be completed because an
    /// "end of file" was reached prematurely.
    UnexpectedEof,
}

impl ErrorKind {
    pub(crate) fn as_str(&self) -> &'static str {
        match *self {
            ErrorKind::NoError => "no error",
            ErrorKind::NotFound => "entity not found",
            ErrorKind::PermissionDenied => "permission denied",
            ErrorKind::ConnectionRefused => "connection refused",
            ErrorKind::ConnectionReset => "connection reset",
            ErrorKind::ConnectionAborted => "connection aborted",
            ErrorKind::NotConnected => "not connected",
            ErrorKind::AddrInUse => "address in use",
            ErrorKind::AddrNotAvailable => "address not available",
            ErrorKind::BrokenPipe => "broken pipe",
            ErrorKind::AlreadyExists => "entity already exists",
            ErrorKind::WouldBlock => "operation would block",
            ErrorKind::InvalidInput => "invalid input parameter",
            ErrorKind::InvalidData => "invalid data",
            ErrorKind::TimedOut => "timed out",
            ErrorKind::WriteZero => "write zero",
            ErrorKind::Interrupted => "operation interrupted",
            ErrorKind::Other => "other os error",
            ErrorKind::UnexpectedEof => "unexpected end of file",
        }
    }
}

impl From<ErrorKind> for Error {
    #[inline]
    fn from(kind: ErrorKind) -> Error {
        Error {
            repr: Repr::Simple(kind),
        }
    }
}

impl Error {
    pub fn new(kind: ErrorKind, error: i32) -> Error {
        if kind == ErrorKind::Other {
            Error {
                repr: Repr::Simple(kind),
            }
        } else {
            Error {
                repr: Repr::Os(error),
            }
        }
    }

    pub fn kind(&self) -> ErrorKind {
        match self.repr {
            Repr::Os(_) => ErrorKind::NoError,
            Repr::Simple(k) => k,
        }
    }
}

impl fmt::Debug for Repr {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Repr::Os(code) => fmt.debug_struct("Os").field("code", &code).finish(),
            Repr::Simple(kind) => fmt.debug_tuple("Kind").field(&kind).finish(),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.repr {
            Repr::Os(code) => write!(fmt, "(os error {})", code),
            Repr::Simple(kind) => write!(fmt, "{}", kind.as_str()),
        }
    }
}

impl error::Error for Error {
    fn cause(&self) -> Option<&dyn error::Error> {
        match self.repr {
            Repr::Os(..) => None,
            Repr::Simple(..) => None,
        }
    }

    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self.repr {
            Repr::Os(..) => None,
            Repr::Simple(..) => None,
        }
    }
}
