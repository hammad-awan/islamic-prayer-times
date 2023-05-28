use core::fmt;
use std::{
    error::Error,
    fmt::{Debug, Display, Formatter, Result},
    ops::RangeInclusive,
};

/// The error type for when a value is out of range.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct OutOfRangeError<T: Debug + Display>(pub RangeInclusive<T>);

impl<T: Debug + Display> Display for OutOfRangeError<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Out Of Range Error [{} - {}]",
            self.0.start(),
            self.0.end()
        )
    }
}

impl<T: Debug + Display> Error for OutOfRangeError<T> {}

/// The error type for when a value cannot be converted to a particular enumeration.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ConversionError;

impl Display for ConversionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "Conversion Error")
    }
}

impl Error for ConversionError {}

/// The error type for when a value cannot be parsed from a string.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ParseError(pub String);

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.0)
    }
}

impl Error for ParseError {}
