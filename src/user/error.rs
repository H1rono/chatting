use std::fmt;

// TODO
#[derive(Debug, Clone, Copy)]
pub struct Error;

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "user service error")
    }
}

impl std::error::Error for Error {}
