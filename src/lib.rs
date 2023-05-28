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

trait Parsable<T>
where
    T: FromStr,
    <T as FromStr>::Err: Display + Debug,
    Self: Sized + TryFrom<T>,
    <Self as TryFrom<T>>::Error: Display + Debug,
{
    fn parse(s: &str) -> std::result::Result<Self, ParseError> {
        let value = s.parse::<T>();
        if value.is_err() {
            Err(ParseError(value.err().unwrap().to_string()))
        } else {
            let t = Self::try_from(value.unwrap());
            if t.is_err() {
                Err(ParseError(t.err().unwrap().to_string()))
            } else {
                Ok(t.unwrap())
            }
        }
    }
}
