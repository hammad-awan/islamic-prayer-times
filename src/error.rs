use core::fmt;

#[derive(Debug, Clone, Copy)]
pub struct OutOfRange;

impl fmt::Display for OutOfRange {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Out of Range")
    }
}

impl std::error::Error for OutOfRange {}

#[derive(Debug, Clone, Copy)]
pub struct ConversionError;

impl fmt::Display for ConversionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Conversion Error")
    }
}

impl std::error::Error for ConversionError {}
