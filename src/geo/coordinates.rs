use std::{fmt::Display, ops::RangeInclusive, str::FromStr};

use crate::{
    angle::{PI_DEG, RIGHT_ANG_DEG},
    error::{OutOfRangeError, ParseError},
    Bounded, Parsable,
};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Gmt(f64);

impl Bounded<f64> for Gmt {
    fn range() -> RangeInclusive<f64> {
        -12. ..=12.
    }

    fn new(value: f64) -> Self {
        Self(value)
    }
}

impl TryFrom<f64> for Gmt {
    type Error = OutOfRangeError<f64>;

    fn try_from(value: f64) -> Result<Self, Self::Error> {
        <Self as Bounded<f64>>::try_from(value)
    }
}

impl From<Gmt> for f64 {
    fn from(value: Gmt) -> Self {
        value.0
    }
}

impl Parsable<f64> for Gmt {}

impl FromStr for Gmt {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::parse(s)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

pub trait GeoAngle {
    fn direction(&self) -> Direction;
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Latitude(f64);

pub const NEAREST_LATITUDE: Latitude = Latitude::nearest_latitude();

impl Latitude {
    const fn nearest_latitude() -> Self {
        Self(48.5)
    }
}

impl Bounded<f64> for Latitude {
    fn range() -> RangeInclusive<f64> {
        -RIGHT_ANG_DEG..=RIGHT_ANG_DEG
    }

    fn new(value: f64) -> Self {
        Self(value)
    }
}

impl TryFrom<f64> for Latitude {
    type Error = OutOfRangeError<f64>;

    fn try_from(value: f64) -> Result<Self, Self::Error> {
        <Self as Bounded<f64>>::try_from(value)
    }
}

impl Default for Latitude {
    fn default() -> Self {
        Self(0.)
    }
}

impl GeoAngle for Latitude {
    fn direction(&self) -> Direction {
        if self.0 >= 0. {
            Direction::North
        } else {
            Direction::South
        }
    }
}

impl From<Latitude> for f64 {
    fn from(value: Latitude) -> Self {
        value.0
    }
}

impl Display for Latitude {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {}",
            self.0.round().abs(),
            if self.direction() == Direction::North {
                "N"
            } else {
                "S"
            }
        )
    }
}

impl Parsable<f64> for Latitude {}

impl FromStr for Latitude {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::parse(s)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Longitude(f64);

impl Bounded<f64> for Longitude {
    fn range() -> RangeInclusive<f64> {
        -PI_DEG..=PI_DEG
    }

    fn new(value: f64) -> Self {
        Self(value)
    }
}

impl TryFrom<f64> for Longitude {
    type Error = OutOfRangeError<f64>;

    fn try_from(value: f64) -> Result<Self, Self::Error> {
        <Self as Bounded<f64>>::try_from(value)
    }
}

impl Default for Longitude {
    fn default() -> Self {
        Self(0.)
    }
}

impl GeoAngle for Longitude {
    fn direction(&self) -> Direction {
        if self.0 >= 0. {
            Direction::East
        } else {
            Direction::West
        }
    }
}

impl From<Longitude> for f64 {
    fn from(value: Longitude) -> Self {
        value.0
    }
}

impl Display for Longitude {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {}",
            self.0.round().abs(),
            if self.direction() == Direction::West {
                "W"
            } else {
                "E"
            }
        )
    }
}

impl Parsable<f64> for Longitude {}

impl FromStr for Longitude {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::parse(s)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Elevation(f64);

impl Bounded<f64> for Elevation {
    fn range() -> RangeInclusive<f64> {
        -420. ..=8848.
    }

    fn new(value: f64) -> Self {
        Self(value)
    }
}

impl TryFrom<f64> for Elevation {
    type Error = OutOfRangeError<f64>;

    fn try_from(value: f64) -> Result<Self, Self::Error> {
        <Self as Bounded<f64>>::try_from(value)
    }
}

impl Default for Elevation {
    fn default() -> Self {
        Self(0.)
    }
}

impl From<Elevation> for f64 {
    fn from(value: Elevation) -> Self {
        value.0
    }
}

impl Display for Elevation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} meters", self.0.round())
    }
}

impl Parsable<f64> for Elevation {}

impl FromStr for Elevation {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::parse(s)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Coordinates {
    pub latitude: Latitude,
    pub longitude: Longitude,
    pub elevation: Elevation,
}

impl Coordinates {
    pub fn new(latitude: Latitude, longitude: Longitude, elevation: Elevation) -> Self {
        Self {
            latitude,
            longitude,
            elevation,
        }
    }
}

impl Display for Coordinates {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}, {}, {}",
            self.latitude, self.longitude, self.elevation
        )
    }
}
