use std::collections::HashMap;

use chrono::NaiveTime;

use crate::{
    angle::LimitAngle,
    geo::{
        astro::{Astro, TopAstroDay},
        coordinates::{Coordinates, Latitude, Longitude},
    },
    prayer_times::params::{AsrShadowRatioMethod, Params, RoundSecondsMethod},
};

use super::{params::Weather, Prayer};

const DEGREES_TO_10_BASE: f64 = 0.066666666666666666;
const INVALID_TRIGGER: f64 = 1.;
const REFRACTION_ALTITUDE: f64 = 0.0347;
const CENTER_OF_SUN_ANGLE: f64 = -0.83337;
const DEGREES_IN_CIRCLE: f64 = 360.;
const DEF_ROUND_SEC: f64 = 30.;
const AGGRESSIVE_ROUND_SEC: f64 = 1.;
const SEC_MIN_FRACTION: f64 = 1. / 60.;

pub fn get_hours(
    params: &Params,
    top_astro_day: &TopAstroDay,
    weather: Weather,
) -> HashMap<Prayer, Result<f64, ()>> {
    use Prayer::*;

    let (shuhr_hour_res, dhuhr_hour, magh_hour_res) =
        get_shur_dhuhr_magh(top_astro_day.coords, top_astro_day, weather);

    let (fajr_hour_res, isha_hour_res) = get_fajr_isha(
        top_astro_day.coords.latitude,
        top_astro_day.astro().dec,
        params.angles[&Fajr],
        params.angles[&Isha],
        dhuhr_hour,
    );

    let asr_hour_res = get_asr(
        top_astro_day.coords.latitude,
        top_astro_day.astro().dec,
        params.asr_shadow_ratio,
        dhuhr_hour,
    );

    let mut hours = HashMap::new();
    hours.insert(Fajr, fajr_hour_res);
    hours.insert(Shurooq, shuhr_hour_res);
    hours.insert(Dhuhr, Ok(dhuhr_hour));
    hours.insert(Asr, asr_hour_res);
    hours.insert(Maghrib, magh_hour_res);
    hours.insert(Isha, isha_hour_res);

    hours
}

pub fn to_time(params: &Params, hour: f64, prayer: Prayer) -> NaiveTime {
    use Prayer::*;
    use RoundSecondsMethod::*;

    let mut hour = hour + params.minute_offsets[&prayer] / 60.;

    if hour < 0. {
        while hour < 0. {
            hour += 24.;
        }
    }

    let mut min = (hour - hour.floor()) * 60.;
    let mut sec = (min - min.floor()) * 60.;

    match params.round_seconds {
        NormalRounding => {
            adj_time(&mut hour, &mut min, &mut sec, DEF_ROUND_SEC);
        }
        SpecialRounding | AggressiveRounding => match prayer {
            Fajr | Dhuhr | Asr | Maghrib | Isha => {
                if params.round_seconds == SpecialRounding {
                    adj_time(&mut hour, &mut min, &mut sec, DEF_ROUND_SEC);
                } else {
                    adj_time(&mut hour, &mut min, &mut sec, AGGRESSIVE_ROUND_SEC);
                }
            }
            _ => sec = 0.,
        },
        _ => {}
    }

    NaiveTime::from_hms_opt(hour as u32, min as u32, sec as u32).unwrap()
}

fn adj_time(hour: &mut f64, min: &mut f64, sec: &mut f64, sec_cap: f64) {
    if *sec >= sec_cap {
        *hour += SEC_MIN_FRACTION;
    }

    *min = (*hour - hour.floor()) * 60.;
    *sec = 0.;
}

fn get_fajr_isha(
    latitude: Latitude,
    dec: f64,
    fajr_angle: f64,
    isha_angle: f64,
    dhuhr_hour: f64,
) -> (Result<f64, ()>, Result<f64, ()>) {
    let lat_rads = f64::from(latitude).to_radians();
    let dec_rads = dec.to_radians();
    let c = lat_rads.cos() * dec_rads.cos();
    let s = lat_rads.sin() * dec_rads.sin();
    let fajr_hour = ((-fajr_angle).to_radians().sin() - s) / c;
    let isha_hour = ((-isha_angle).to_radians().sin() - s) / c;
    let fajr_hour = if fajr_hour < -INVALID_TRIGGER || fajr_hour > INVALID_TRIGGER {
        Err(())
    } else {
        Ok(dhuhr_hour - DEGREES_TO_10_BASE * fajr_hour.acos().to_degrees())
    };
    let isha_hour = if isha_hour < -INVALID_TRIGGER || isha_hour > INVALID_TRIGGER {
        Err(())
    } else {
        Ok(DEGREES_TO_10_BASE * isha_hour.acos().to_degrees() + dhuhr_hour)
    };

    (fajr_hour, isha_hour)
}

fn get_asr(
    latitude: Latitude,
    dec: f64,
    madhab: AsrShadowRatioMethod,
    dhuhr_hour: f64,
) -> Result<f64, ()> {
    let madhab = madhab as u8 as f64;
    let lat = f64::from(latitude);
    let mut asr_hour = (lat - dec).to_radians();
    asr_hour = madhab + asr_hour.abs().tan();
    asr_hour = (1. / asr_hour).atan();
    let lat_rads = lat.to_radians();
    let dec_rads = dec.to_radians();
    asr_hour = asr_hour.sin() - lat_rads.sin() * dec_rads.sin();
    asr_hour = asr_hour / (lat_rads.cos() * dec_rads.cos());
    if asr_hour < -INVALID_TRIGGER || asr_hour > INVALID_TRIGGER {
        Err(())
    } else {
        Ok(DEGREES_TO_10_BASE * asr_hour.acos().to_degrees() + dhuhr_hour)
    }
}

fn get_ra_deltas(top_astro_day: &TopAstroDay) -> (f64, f64) {
    let mut prev = top_astro_day.prev_astro().ra;
    let mut next = top_astro_day.next_astro().ra;
    let j = 350.;
    let k = 10.;
    if top_astro_day.astro().ra > j && top_astro_day.next_astro().ra < k {
        next += DEGREES_IN_CIRCLE;
    }
    if top_astro_day.prev_astro().ra > j && top_astro_day.astro().ra < k {
        prev = 0.;
    }
    let delta1 = next - prev;
    let delta2 = next + prev - 2. * top_astro_day.astro().ra;
    (delta1, delta2)
}

fn get_dec_deltas(astro_day: &TopAstroDay) -> (f64, f64) {
    let delta1 = astro_day.next_astro().dec - astro_day.prev_astro().dec;
    let delta2 =
        astro_day.next_astro().dec - 2. * astro_day.astro().dec + astro_day.prev_astro().dec;
    (delta1, delta2)
}

fn get_ra_factor(longitude: Longitude, astro: &Astro, ra_deltas: (f64, f64), val: f64) -> f64 {
    let a = (astro.sid_time + 360.985647 * val).cap_angle_360();
    let b = astro.ra + val * (ra_deltas.0 + ra_deltas.1 * val) / 2.;
    (a + f64::from(longitude) - b).cap_angle_between_180()
}

fn get_shur_dhuhr_magh(
    coords: Coordinates,
    astro_day: &TopAstroDay,
    weather: Weather,
) -> (Result<f64, ()>, f64, Result<f64, ()>) {
    let ra_deltas = get_ra_deltas(astro_day);

    let dhuhr_factor =
        (astro_day.astro().ra - f64::from(coords.longitude) - astro_day.astro().sid_time)
            / DEGREES_IN_CIRCLE;
    let dhuhr_factor_cap = dhuhr_factor.cap_angle_1();
    let dhuhr_ra_factor = get_ra_factor(
        coords.longitude,
        astro_day.astro(),
        ra_deltas,
        dhuhr_factor_cap,
    );
    let dhuhr_hour = 24. * (dhuhr_factor_cap - dhuhr_ra_factor / DEGREES_IN_CIRCLE);

    let shur_magh_res =
        if let Ok(sm_adj) = get_shur_magh_adj(coords.latitude, astro_day.astro().dec) {
            let dec_deltas = get_dec_deltas(astro_day);

            let shur_factor_cap = (dhuhr_factor - sm_adj).cap_angle_1();
            let shur_ra_factor = get_ra_factor(
                coords.longitude,
                astro_day.astro(),
                ra_deltas,
                shur_factor_cap,
            );
            let shur_hour = get_shur_magh(
                coords,
                astro_day.astro(),
                weather,
                dec_deltas,
                shur_factor_cap,
                shur_ra_factor,
            );

            let magh_factor_cap = (dhuhr_factor + sm_adj).cap_angle_1();
            let magh_ra_factor = get_ra_factor(
                coords.longitude,
                astro_day.astro(),
                ra_deltas,
                magh_factor_cap,
            );
            let magh_hour = get_shur_magh(
                coords,
                astro_day.astro(),
                weather,
                dec_deltas,
                magh_factor_cap,
                magh_ra_factor,
            );

            (Ok(shur_hour), Ok(magh_hour))
        } else {
            (Err(()), Err(()))
        };

    (shur_magh_res.0, dhuhr_hour, shur_magh_res.1)
}

fn get_refraction(weather: Weather, sun_alt: f64) -> f64 {
    let w = f64::from(weather.pressure) / 1010. * (283. / (273. + f64::from(weather.temperature)));
    let s = 1. / (sun_alt + 7.31 / (sun_alt + 4.4)).to_radians().tan() + 0.0013515;
    let refraction = w * s / 60.;
    refraction
}

fn get_shur_magh_adj(latitude: Latitude, dec: f64) -> Result<f64, ()> {
    let lat_rads = f64::from(latitude).to_radians();
    let dec_rads = dec.to_radians();
    let n = CENTER_OF_SUN_ANGLE.to_radians().sin() - lat_rads.sin() * dec_rads.sin();
    let d = lat_rads.cos() * dec_rads.cos();
    let r = n / d;
    if r <= -1.0 || r >= 1. {
        return Err(());
    }

    let adj = r.acos().to_degrees().cap_angle_180() / DEGREES_IN_CIRCLE;
    Ok(adj)
}

fn get_shur_magh(
    coords: Coordinates,
    astro: &Astro,
    weather: Weather,
    dec_deltas: (f64, f64),
    factor_cap: f64,
    ra_factor: f64,
) -> f64 {
    let dec_rads =
        (astro.dec + factor_cap * (dec_deltas.0 + dec_deltas.1 * factor_cap) / 2.).to_radians();
    let lat_rads = f64::from(coords.latitude).to_radians();
    let da_rads = (ra_factor - astro.dra.to_degrees()).to_radians();
    let mut sun_alt = (lat_rads.sin() * dec_rads.sin()
        + lat_rads.cos() * dec_rads.cos() * da_rads.cos())
    .asin()
    .to_degrees();
    sun_alt += get_refraction(weather, sun_alt);
    let hour = 24.
        * (factor_cap
            + (sun_alt - CENTER_OF_SUN_ANGLE
                + REFRACTION_ALTITUDE * f64::from(coords.elevation).powf(0.5))
                / (DEGREES_IN_CIRCLE * dec_rads.cos() * lat_rads.cos() * da_rads.sin()));
    hour
}
