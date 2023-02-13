use core::fmt;

pub mod geo;
pub mod prayer_times;

mod angle;

#[derive(Debug, Clone, Copy)]
pub struct OutOfRange;

impl fmt::Display for OutOfRange {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Out of Range")
    }
}

impl std::error::Error for OutOfRange {}
