pub mod error;
pub mod geo;
pub mod hijri_date;
pub mod prayer_times;

use std::fmt::{Debug, Display};
use std::ops::RangeInclusive;
use std::str::FromStr;

pub use error::*;
pub use geo::*;
pub use hijri_date::*;
pub use prayer_times::*;

mod angle;

// A trait to implement on a type that is bounded within an inclusive range
// which provides a default implementation for try_from so it can be constructed
// using TryFrom<T>::try_from syntax from a type T.
trait Bounded<T>: Sized
where
    T: Debug + Display + PartialOrd,
{
    fn range() -> RangeInclusive<T>;

    fn try_from(value: T) -> Result<Self, OutOfRangeError<T>> {
        if Self::range().contains(&value) {
            Ok(Self::new(value))
        } else {
            Err(OutOfRangeError(Self::range()))
        }
    }

    fn new(value: T) -> Self;
}

// A trait to implement on a type which provides a default implentation for
// parse so that it can be constructed by parsing a string from a type T.
trait Parsable<T>
where
    T: FromStr,
    <T as FromStr>::Err: Display + Debug,
    Self: Sized + TryFrom<T>,
    <Self as TryFrom<T>>::Error: Display + Debug,
{
    fn parse(s: &str) -> std::result::Result<Self, ParseError> {
        let value = s.parse::<T>();
        if let Ok(value) = value {
            let t = Self::try_from(value);
            if let Ok(t) = t {
                Ok(t)
            } else {
                Err(ParseError(t.err().unwrap().to_string()))
            }
        } else {
            Err(ParseError(value.err().unwrap().to_string()))
        }
    }
}
