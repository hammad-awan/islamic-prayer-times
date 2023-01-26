use std::collections::HashSet;

use crate::geo::coordinates::Coordinates;

const DEGREES_TO_10_BASE: f64 = 0.066666666666666666;

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
    dec: f64,
    fajr_angle: f64,
    isha_angle: f64,
) -> (Option<f64>, Option<f64>) {
    let lat_rads = f64::from(coords.latitude()).to_radians();
    let x = lat_rads.cos() * dec.cos();
    let y_const = lat_rads.sin() * dec.sin();
    let fajr_z = (-fajr_angle.to_radians().sin() - y_const) / x;
    let isha_z = (-isha_angle.to_radians().sin() - y_const) / x;
    let invalid_trigger = -0.999;
    let fajr_hour = if fajr_z < invalid_trigger {
        None
    } else {
        Some(DEGREES_TO_10_BASE * fajr_z.acos().to_degrees())
    };
    let isha_hour = if isha_z < invalid_trigger {
        None
    } else {
        Some(DEGREES_TO_10_BASE * isha_z.acos().to_degrees())
    };

    (fajr_hour, isha_hour)
}
