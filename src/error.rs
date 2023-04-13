use core::fmt;

/// The error type which is returned when a value is out of range.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct OutOfRangeError;

impl fmt::Display for OutOfRangeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Out of Range")
    }
}

impl std::error::Error for OutOfRangeError {}

/// The error type which is returned when a value cannot be converted to a particular enumeration.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ConversionError;

impl fmt::Display for ConversionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Conversion Error")
    }
}

impl std::error::Error for ConversionError {}
