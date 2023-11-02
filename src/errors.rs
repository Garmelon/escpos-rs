//! Custom errors

#[cfg(feature = "graphics")]
use image::ImageError;
use std::{borrow::Cow, cell::BorrowMutError, fmt, io, num::TryFromIntError};

/// Custom Result
pub type Result<T> = std::result::Result<T, PrinterError>;

/// Printer error
#[derive(Debug)]
pub enum PrinterError {
    Io(String),
    Input(String),
    Network(String),
}

impl fmt::Display for PrinterError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PrinterError::Io(ref err) => write!(f, "IO error: {err}"),
            PrinterError::Network(ref err) => write!(f, "Network error: {err}"),
            PrinterError::Input(ref err) => write!(f, "Input error: {err}"),
        }
    }
}

impl From<io::Error> for PrinterError {
    fn from(err: std::io::Error) -> PrinterError {
        PrinterError::Io(err.to_string())
    }
}

impl From<Cow<'_, str>> for PrinterError {
    fn from(value: Cow<'_, str>) -> Self {
        PrinterError::Io(value.into_owned())
    }
}

impl From<BorrowMutError> for PrinterError {
    fn from(err: BorrowMutError) -> Self {
        PrinterError::Io(err.to_string())
    }
}

impl From<TryFromIntError> for PrinterError {
    fn from(err: TryFromIntError) -> Self {
        PrinterError::Io(err.to_string())
    }
}

#[cfg(feature = "graphics")]
impl From<ImageError> for PrinterError {
    fn from(err: ImageError) -> Self {
        PrinterError::Io(err.to_string())
    }
}
