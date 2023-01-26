use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Direction {
    North,
    South,
    East,
    West
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Latitude(f64);

impl Latitude {
    pub fn new(value: f64) -> Result<Latitude, ()> {
        if value > 90. || value < -90. {
            Err(())
        } else {
            Ok(Latitude(value))
        }
    }

    pub fn direction(&self) -> Direction {
        if self.0 >= 0. {
            Direction::North
        }
        else {
            Direction:: South
        }
    }
}

impl Display for Latitude {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.0.round().abs(), if self.direction() == Direction::North { "N" } else { "S"})
    }
}

impl From<Latitude> for f64 {
    fn from(value: Latitude) -> Self {
        value.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Longitude(f64);

impl Longitude {
    pub fn new(value: f64) -> Result<Longitude, ()> {
        if value > 180. || value < -180. {
            Err(())
        } else {
            Ok(Longitude(value))
        }
    }

    pub fn direction(&self) -> Direction {
        if self.0 >= 0. {
            Direction::East
        }
        else {
            Direction:: West
        }
    }
}

impl Display for Longitude {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.0.round().abs(), if self.direction() == Direction::West { "W" } else { "E"})
    }
}

impl From<Longitude> for f64 {
    fn from(value: Longitude) -> Self {
        value.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Elevation(f64);

impl Elevation {
    pub const MAX: f64 = 8848.;
    pub const MIN: f64 = -420.;

    pub fn new(value: f64) -> Result<Elevation, ()> {
        if value > Elevation::MAX || value < Elevation::MIN {
            Err(())
        } else {
            Ok(Elevation(value))
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
    latitude: Latitude,
    longitude: Longitude,
    elevation: Elevation,
}

impl Coordinates {
    pub fn new(latitude: Latitude, longitude: Longitude, elevation: Elevation) -> Coordinates {
        Coordinates {
            latitude,
            longitude,
            elevation,
        }
    }

    pub fn latitude(&self) -> Latitude {
        self.latitude
    }

    pub fn longitude(&self) -> Longitude {
        self.longitude
    }

    pub fn elevation(&self) -> Elevation {
        self.elevation
    }
}

impl Display for Coordinates {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}, {}, {}", self.latitude, self.longitude, self.elevation)
    }
}