use std::fmt::Display;

use crate::{
    angle::{PI_DEG, RIGHT_ANG_DEG},
    error::OutOfRange,
};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Gmt(f64);

impl Gmt {
    pub fn new(gmt: f64) -> Result<Gmt, OutOfRange> {
        if (-12. ..=12.).contains(&gmt) {
            Ok(Gmt(gmt))
        } else {
            Err(OutOfRange)
        }
    }
}

impl From<Gmt> for f64 {
    fn from(value: Gmt) -> Self {
        value.0
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

impl Latitude {
    pub fn new(degrees: f64) -> Result<Self, OutOfRange> {
        if (-RIGHT_ANG_DEG..=RIGHT_ANG_DEG).contains(&degrees) {
            Ok(Self(degrees))
        } else {
            Err(OutOfRange)
        }
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

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Longitude(f64);

impl Longitude {
    pub fn new(degrees: f64) -> Result<Self, OutOfRange> {
        if (-PI_DEG..=PI_DEG).contains(&degrees) {
            Ok(Self(degrees))
        } else {
            Err(OutOfRange)
        }
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

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Elevation(f64);

impl Elevation {
    pub const MAX: f64 = 8848.;
    pub const MIN: f64 = -420.;

    pub fn new(value: f64) -> Result<Self, OutOfRange> {
        if (Elevation::MIN..=Elevation::MAX).contains(&value) {
            Ok(Self(value))
        } else {
            Err(OutOfRange)
        }
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
