use std::fmt::Display;

use super::coordinates::Coordinates;

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
        let lon_rads = f64::from(coords.longitude()).to_radians();
        let lat_rads = f64::from(coords.latitude()).to_radians();
        let lon_kaaba_rads = lon_rads - Qibla::KAABA_LONGITUDE.to_radians();
        let y = lon_kaaba_rads.sin();
        let x = lat_rads.cos() * Qibla::KAABA_LATITUDE.to_radians().tan()
            - lat_rads.sin() * lon_kaaba_rads.cos();
        let angle = y.atan2(x).to_degrees();
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
