use std::error::{Error as StdError};
use std::io::{Error as IoError, ErrorKind as IoErrorKind};

use void::{unreachable, Void};


quick_error! {
    /// A generic name resolution error
    ///
    /// It's designed to provide basic abstraction over error types and also
    /// provide as much information as possible by carrying original error
    #[derive(Debug)]
    pub enum Error {
        /// Couldn't parse a name before resolution
        ///
        /// It's expected that this error is permanent and is a failure of
        /// validating user input or the name in the configuration is invalid,
        /// but it's possible that some resolver have very specific
        /// requirements for names, so you might want to change resolver too.
        InvalidName(name: String, description: &'static str) {
            description("name that you are trying to resolve is invalid")
            display("name {:?} is invalid: {}", name, description)
        }
        /// Temporary name resolution error
        ///
        /// This means either name server returned this kind of error or
        /// we couldn't connect to a name server itself. It's safe to assume
        /// that you can retry name resolution in a moment
        TemporaryError(err: Box<StdError + Send + Sync>) {
            description("temporary name resolution error")
            display("temporary name resolution error: {}", err)
            cause(&**err)
        }
        /// We have sucessfully done name resolution but there is no such name
        NameNotFound {
            description("name not found")
            display("name not found")
        }
        /// The target resolver can only resolve host names and no default
        /// port is specified
        NoDefaultPort {
            description("the resolver can only resolve hostname to an IP, \
                address, so port must be specified to get full address")
        }
    }
}

impl Error {
    /// Wraps the error into `std::io::Error`.
    pub fn into_io(self) -> IoError {
        match self {
            Error::InvalidName(_, _) =>
                IoError::new(IoErrorKind::InvalidInput, self),
            Error::TemporaryError(_) =>
                IoError::new(IoErrorKind::Other, self),
            Error::NameNotFound =>
                IoError::new(IoErrorKind::NotFound, self),
            Error::NoDefaultPort =>
                IoError::new(IoErrorKind::NotFound, self),
        }
    }
}

impl From<Void> for Error {
    fn from(v: Void) -> Error {
        unreachable(v);
    }
}

#[test]
fn send_sync() {
    fn send_sync<T: Send+Sync>(_: T) {}
    send_sync(Error::NameNotFound);
}

#[test]
fn wrap_into_io() {
    assert_eq!(Error::InvalidName("foo".to_string(), "bar").into_io().kind(),
        IoErrorKind::InvalidInput);
    assert_eq!(Error::TemporaryError(Box::new(IoError::new(IoErrorKind::Other, "oh no!"))).into_io().kind(),
        IoErrorKind::Other);
    assert_eq!(Error::NameNotFound.into_io().kind(),
        IoErrorKind::NotFound);
}
