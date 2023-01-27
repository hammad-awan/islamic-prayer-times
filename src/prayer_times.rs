use crate::{
    angle::Angle,
    geo::coordinates::{Coordinates, GeoAngle},
};

const DEGREES_TO_10_BASE: f64 = 0.066666666666666666;
const INVALID_TRIGGER: f64 = -0.999;

pub enum Prayer {
    Fajr,
    Shurooq,
    Dhuhr,
    Asr,
    Maghrib,
    Isha,
    Imsaak,
}

pub struct PrayerHour {
    hour: f64,
    extreme: bool,
}

fn get_fajr_isha(
    coords: Coordinates,
    dec: Angle,
    fajr_angle: Angle,
    isha_angle: Angle,
) -> (Result<f64, ()>, Result<f64, ()>) {
    let x = coords.latitude.angle().cos() * dec.cos();
    let y = coords.latitude.angle().sin() * dec.sin();
    let fajr_z = (-fajr_angle.radians().sin() - y) / x;
    let isha_z = (-isha_angle.radians().sin() - y) / x;
    let fajr_hour = if fajr_z < INVALID_TRIGGER {
        Err(())
    } else {
        Ok(DEGREES_TO_10_BASE * fajr_z.acos().to_degrees())
    };
    let isha_hour = if isha_z < INVALID_TRIGGER {
        Err(())
    } else {
        Ok(DEGREES_TO_10_BASE * isha_z.acos().to_degrees())
    };

    (fajr_hour, isha_hour)
}
