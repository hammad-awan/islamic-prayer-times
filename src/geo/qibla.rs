//! The [`Qibla`] (Arabic: قِبْلَة, romanized: qiblah, lit. 'direction') is the direction towards
//! the Kaaba in the Sacred Mosque in Mecca, which is used by Muslims in various religious
//!  contexts, particularly the direction of prayer.

use std::fmt::Display;

use super::coordinates::Coordinates;

/// `Qibla` for geographical [`Coordinates`](super::coordinates::Coordinates).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Qibla {
    coords: Coordinates,
    degrees: f64,
}

/// An enumeration of rotation values.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Rotation {
    /// Clockwise
    Cw,
    /// Counterclockwise
    Ccw,
}

impl Display for Rotation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let rot_str = match self {
            Rotation::Cw => "CW",
            Rotation::Ccw => "CCW",
        };
        write!(f, "{}", rot_str)
    }
}

impl Qibla {
    const KAABA_LATITUDE: f64 = 21.423333;
    const KAABA_LONGITUDE: f64 = 39.823333;

    /// Constructs a new `Qibla` from geographical [`Coordinates`](super::coordinates::Coordinates).
    pub fn new(coords: Coordinates) -> Self {
        let lat_rads = f64::from(coords.latitude).to_radians();
        let x = f64::from(coords.longitude).to_radians() - Self::KAABA_LONGITUDE.to_radians();
        let y = lat_rads.cos() * Self::KAABA_LATITUDE.to_radians().tan() - lat_rads.sin() * x.cos();
        let degrees = x.sin().atan2(y).to_degrees();
        Self { coords, degrees }
    }

    /// Returns the `Qibla` geographical [`Coordinates`](super::coordinates::Coordinates) passed to [`Qibla::new`].
    pub fn coords(&self) -> Coordinates {
        self.coords
    }

    /// Returns the `Qibla` direction in degrees from North [-90. ..=90.].
    pub fn degrees(&self) -> f64 {
        self.degrees
    }

    /// Returns the `Qibla` [`Rotation`].
    pub fn rotation(&self) -> Rotation {
        if self.degrees < 0. {
            Rotation::Cw
        } else {
            Rotation::Ccw
        }
    }
}

impl Display for Qibla {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:.1}° {}", self.degrees.abs(), self.rotation())
    }
}
