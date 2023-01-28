use crate::{
    angle::{Angle, LimitAngle},
    geo::{
        astro::{Astro, AstroDay},
        coordinates::{Coordinates, GeoAngle, Latitude, Longitude},
    },
};

use super::params::AsrShadowRatioMethod;

const DEGREES_TO_10_BASE: f64 = 0.066666666666666666;
const INVALID_TRIGGER: f64 = -0.999;
const REFRACTION_ALTITUDE: f64 = 0.0347;
const CENTER_OF_SUN_ANGLE: f64 = -0.83337;
const DEGREES_IN_CIRCLE: f64 = 360.;

#[derive(Debug, Clone, Copy)]
pub struct Weather {
    pressure: f64,
    temperature: f64,
}

impl Default for Weather {
    fn default() -> Self {
        Self {
            pressure: 1010.,
            temperature: 10.,
        }
    }
}

fn get_fajr_isha(
    latitude: Latitude,
    dec: Angle,
    fajr_angle: Angle,
    isha_angle: Angle,
) -> (Result<f64, ()>, Result<f64, ()>) {
    let c = latitude.angle().cos() * dec.cos();
    let s = latitude.angle().sin() * dec.sin();
    let fajr_hour = (-fajr_angle.sin() - s) / c;
    let isha_hour = (-isha_angle.sin() - s) / c;
    let fajr_hour = if fajr_hour <= INVALID_TRIGGER {
        Err(())
    } else {
        Ok(DEGREES_TO_10_BASE * fajr_hour.acos().to_degrees())
    };
    let isha_hour = if isha_hour <= INVALID_TRIGGER {
        Err(())
    } else {
        Ok(DEGREES_TO_10_BASE * isha_hour.acos().to_degrees())
    };

    (fajr_hour, isha_hour)
}

fn get_asr(latitude: Latitude, dec: Angle, madhab: AsrShadowRatioMethod) -> f64 {
    let madhab = madhab as u8 as f64;
    let k = (latitude.angle() - dec).tan();
    let mut asr_hour = madhab + k;
    if asr_hour < 1.0 || latitude.angle().degrees() < 0. {
        asr_hour = madhab - k;
    }
    let asr_hour = 1.5707963267948966 - asr_hour.atan();
    let asr_hour = asr_hour.sin() - latitude.angle().sin() * dec.sin();
    let asr_hour = asr_hour / (latitude.angle().cos() * dec.cos());
    let mut asr_hour = asr_hour.acos();
    if f64::is_nan(asr_hour) {
        asr_hour = 0.;
    }
    let asr_hour = DEGREES_TO_10_BASE * asr_hour.to_degrees();
    asr_hour
}

fn get_ra_deltas(astro_day: &AstroDay) -> (f64, f64) {
    let mut prev = astro_day.prev_astro().ra.degrees();
    let mut next = astro_day.next_astro().ra.degrees();
    let j = 350.;
    let k = 10.;
    if astro_day.astro().ra.degrees() > j && astro_day.next_astro().ra.degrees() < k {
        next += DEGREES_IN_CIRCLE;
    }
    if astro_day.prev_astro().ra.degrees() > j && astro_day.astro().ra.degrees() < k {
        prev = 0.;
    }
    let delta1 = next - prev;
    let delta2 = next + prev - 2. * astro_day.astro().ra.degrees();
    (delta1, delta2)
}

fn get_dec_deltas(astro_day: &AstroDay) -> (f64, f64) {
    let delta1 = astro_day.next_astro().dec.degrees() - astro_day.prev_astro().dec.degrees();
    let delta2 = astro_day.next_astro().dec.degrees() - 2. * astro_day.astro().dec.degrees()
        + astro_day.prev_astro().dec.degrees();
    (delta1, delta2)
}

fn get_ra_factor(longitude: Longitude, astro: &Astro, ra_deltas: (f64, f64), val: f64) -> f64 {
    let a = (astro.sid_time.degrees() + 360.985647 * val).cap_angle_360();
    let b = astro.ra.degrees() + val * (ra_deltas.0 + ra_deltas.1 * val) / 2.;
    (a + longitude.angle().degrees() - b).cap_angle_between_180()
}

fn get_shurooq_dhuhr_maghrib(
    coords: Coordinates,
    astro_day: &AstroDay,
    weather: Weather,
) -> (f64, Result<(f64, f64), ()>) {
    let ra_deltas = get_ra_deltas(astro_day);

    let dhuhr_factor =
        (astro_day.astro().ra - coords.longitude.angle() - astro_day.astro().sid_time).degrees()
            / DEGREES_IN_CIRCLE;
    let dhuhr_factor_cap = dhuhr_factor.cap_angle_1();
    let dhuhr_ra_factor = get_ra_factor(
        coords.longitude,
        astro_day.astro(),
        ra_deltas,
        dhuhr_factor_cap,
    );
    let dhuhr_hour = 24. * (dhuhr_factor_cap - dhuhr_ra_factor / DEGREES_IN_CIRCLE);

    let sh_m_res =
        if let Ok(sm_adj) = get_shurooq_maghrib_adj(coords.latitude, astro_day.astro().dec) {
            let dec_deltas = get_dec_deltas(astro_day);

            let shurooq_factor_cap = (dhuhr_factor - sm_adj).cap_angle_1();
            let shurooq_ra_factor = get_ra_factor(
                coords.longitude,
                astro_day.astro(),
                ra_deltas,
                shurooq_factor_cap,
            );
            let shurooq_hour = get_shurooq_maghrib(
                coords,
                astro_day.astro(),
                weather,
                dec_deltas,
                shurooq_factor_cap,
                shurooq_ra_factor,
            );

            let maghrib_factor_cap = (dhuhr_factor + sm_adj).cap_angle_1();
            let maghrib_ra_factor = get_ra_factor(
                coords.longitude,
                astro_day.astro(),
                ra_deltas,
                maghrib_factor_cap,
            );
            let maghrib_hour = get_shurooq_maghrib(
                coords,
                astro_day.astro(),
                weather,
                dec_deltas,
                maghrib_factor_cap,
                maghrib_ra_factor,
            );

            Ok((shurooq_hour, maghrib_hour))
        } else {
            Err(())
        };

    (dhuhr_hour, sh_m_res)
}

fn get_refraction(weather: Weather, sun_alt: f64) -> f64 {
    let w = weather.pressure / 1010. * (283. / (273. + weather.temperature));
    let s = 1. / (sun_alt + 7.31 / (sun_alt + 4.4)).to_radians().tan() + 0.0013515;
    let refraction = w * s / 60.;
    refraction
}

fn get_shurooq_maghrib_adj(latitude: Latitude, dec: Angle) -> Result<f64, ()> {
    let n = CENTER_OF_SUN_ANGLE.to_radians().sin() - latitude.angle().sin() * dec.sin();
    let d = latitude.angle().cos() * dec.cos();
    let r = n / d;
    if r <= -1.0 || r >= 1. {
        return Err(());
    }

    let adj = r.acos().to_degrees().cap_angle_180() / DEGREES_IN_CIRCLE;
    Ok(adj)
}

fn get_shurooq_maghrib(
    coords: Coordinates,
    astro: &Astro,
    weather: Weather,
    dec_deltas: (f64, f64),
    factor_cap: f64,
    ra_factor: f64,
) -> f64 {
    let dec_rads = (astro.dec.degrees()
        + factor_cap * (dec_deltas.0 + dec_deltas.1 * factor_cap) / 2.)
        .to_radians();
    let da_rads = (ra_factor - astro.dra.degrees()).to_radians();
    let mut sun_alt = (coords.latitude.angle().sin() * dec_rads.sin()
        + coords.latitude.angle().cos() * dec_rads.cos() * da_rads.cos())
    .asin()
    .to_degrees();
    sun_alt += get_refraction(weather, sun_alt);
    let hour = 24.
        * (factor_cap
            + (sun_alt - CENTER_OF_SUN_ANGLE
                + REFRACTION_ALTITUDE * f64::from(coords.elevation).powf(0.5))
                / (DEGREES_IN_CIRCLE
                    * dec_rads.cos()
                    * coords.latitude.angle().cos()
                    * da_rads.sin()));
    hour
}
