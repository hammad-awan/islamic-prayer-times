use std::fmt::Display;

use super::coordinates::{Coordinates, GeoAngle};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Qibla {
    coords: Coordinates,
    angle: f64,
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

    pub fn new(coords: Coordinates) -> Qibla {
        let x = coords.longitude.angle().radians() - Qibla::KAABA_LONGITUDE.to_radians();
        let y = coords.latitude.angle().cos() * Qibla::KAABA_LATITUDE.to_radians().tan()
            - coords.latitude.angle().sin() * x.cos();
        let angle = x.sin().atan2(y).to_degrees();
        Qibla { coords, angle }
    }

    pub fn coords(&self) -> Coordinates {
        self.coords
    }

    pub fn angle(&self) -> f64 {
        self.angle
    }

    pub fn rotation(&self) -> Rotation {
        if self.angle < 0. {
            Rotation::Cw
        } else {
            Rotation::Ccw
        }
    }
}

impl Display for Qibla {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:.1}° {}", self.angle.abs(), self.rotation())
    }
}
