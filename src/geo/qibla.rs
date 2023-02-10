use std::fmt::Display;

use super::coordinates::Coordinates;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Qibla {
    coords: Coordinates,
    degrees: f64,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Rotation {
    Cw,
    Ccw,
}

impl Display for Rotation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let rot_str = match self {
            &Rotation::Cw => "CW",
            &Rotation::Ccw => "CCW",
        };
        write!(f, "{}", rot_str)
    }
}

impl Qibla {
    const KAABA_LATITUDE: f64 = 21.423333;
    const KAABA_LONGITUDE: f64 = 39.823333;

    pub fn new(coords: Coordinates) -> Self {
        let lat_rads = f64::from(coords.latitude).to_radians();
        let x = f64::from(coords.longitude).to_radians() - Self::KAABA_LONGITUDE.to_radians();
        let y = lat_rads.cos() * Self::KAABA_LATITUDE.to_radians().tan() - lat_rads.sin() * x.cos();
        let degrees = x.sin().atan2(y).to_degrees();
        Self { coords, degrees }
    }

    pub fn coords(&self) -> Coordinates {
        self.coords
    }

    pub fn degrees(&self) -> f64 {
        self.degrees
    }

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
