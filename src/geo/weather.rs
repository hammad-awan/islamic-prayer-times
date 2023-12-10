//! Weather information types.
//!

use std::ops::RangeInclusive;

use serde::{Deserialize, Serialize};

use crate::{Bounded, OutOfRangeError};

/// An atmospheric pressure in millibars.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Pressure(f64);

impl Bounded<f64> for Pressure {
    fn range() -> RangeInclusive<f64> {
        100. ..=1050.
    }

    fn new(value: f64) -> Self {
        Self(value)
    }
}

impl From<Pressure> for f64 {
    fn from(value: Pressure) -> Self {
        value.0
    }
}

impl TryFrom<f64> for Pressure {
    type Error = OutOfRangeError<f64>;

    fn try_from(value: f64) -> Result<Self, Self::Error> {
        <Self as Bounded<f64>>::try_from(value)
    }
}

/// An outside temperature in degrees Celcius.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Temperature(f64);

impl Bounded<f64> for Temperature {
    fn range() -> RangeInclusive<f64> {
        -90. ..=57.
    }

    fn new(value: f64) -> Self {
        Self(value)
    }
}

impl From<Temperature> for f64 {
    fn from(value: Temperature) -> Self {
        value.0
    }
}

impl TryFrom<f64> for Temperature {
    type Error = OutOfRangeError<f64>;

    fn try_from(value: f64) -> Result<Self, Self::Error> {
        <Self as Bounded<f64>>::try_from(value)
    }
}

/// Current weather as specified by [`Pressure`] and [`Temperature`].
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Weather {
    /// Atmospheric pressure
    pub pressure: Pressure,
    /// Outside temperature
    pub temperature: Temperature,
}

impl Default for Weather {
    fn default() -> Self {
        Self {
            pressure: <Pressure as TryFrom<f64>>::try_from(1010.).unwrap(),
            temperature: <Temperature as TryFrom<f64>>::try_from(14.).unwrap(),
        }
    }
}
