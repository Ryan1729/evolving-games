use std::error;
use std::fmt;
use std::result;

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub struct Error {
    pub line: u32,
    pub file: &'static str,
    pub kind: ErrorKind,
}

macro_rules! err {
    ($kind:expr) => {
        Err(Error {
            line: line!(),
            file: file!(),
            kind: $kind,
        })
    };
}

#[derive(Debug)]
pub enum ErrorKind {
    NotImplemented,
}
pub use ErrorKind::*;

impl fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} error occurred at line {} in {}",
            self.kind, self.line, self.file
        )
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        "https://github.com/rust-lang/rfcs/pull/2230"
    }
}
