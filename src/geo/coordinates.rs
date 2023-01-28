use std::fmt::Display;

use crate::angle::Angle;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

pub trait GeoAngle {
    fn direction(&self) -> Direction;
    fn angle(&self) -> Angle;
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Latitude(Angle);

impl Latitude {
    pub fn new(degrees: f64) -> Result<Self, ()> {
        if degrees > 90. || degrees < -90. {
            Err(())
        } else {
            Ok(Self(Angle::from_degrees(degrees)))
        }
    }
}

impl GeoAngle for Latitude {
    fn direction(&self) -> Direction {
        if self.0.degrees() >= 0. {
            Direction::North
        } else {
            Direction::South
        }
    }

    fn angle(&self) -> Angle {
        self.0
    }
}

impl Display for Latitude {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {}",
            self.angle().degrees().round().abs(),
            if self.direction() == Direction::North {
                "N"
            } else {
                "S"
            }
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Longitude(Angle);

impl Longitude {
    pub fn new(degrees: f64) -> Result<Self, ()> {
        if degrees > 180. || degrees < -180. {
            Err(())
        } else {
            Ok(Self(Angle::from_degrees(degrees)))
        }
    }
}

impl GeoAngle for Longitude {
    fn direction(&self) -> Direction {
        if self.0.degrees() >= 0. {
            Direction::East
        } else {
            Direction::West
        }
    }

    fn angle(&self) -> Angle {
        self.0
    }
}

impl Display for Longitude {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {}",
            self.angle().degrees().round().abs(),
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

    pub fn new(value: f64) -> Result<Self, ()> {
        if value > Elevation::MAX || value < Elevation::MIN {
            Err(())
        } else {
            Ok(Self(value))
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
