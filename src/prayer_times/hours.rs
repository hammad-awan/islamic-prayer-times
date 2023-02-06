use std::{collections::HashMap, ops::Rem};

use chrono::NaiveTime;

use crate::{
    angle::LimitAngle,
    geo::astro::TopAstroDay,
    prayer_times::params::{AsrShadowRatio, Params, RoundSeconds},
};

use super::{params::Weather, Prayer};

const DEGREES_TO_10_BASE: f64 = 0.066666666666666666;
const INVALID_TRIGGER: f64 = 1.;
const REFRACTION_ALTITUDE: f64 = 0.0347;
const CENTER_OF_SUN_ANGLE: f64 = -0.83337;
const DEGREES_IN_CIRCLE: f64 = 360.;
const DEF_ROUND_SEC: f64 = 30.;
const AGGRESSIVE_ROUND_SEC: f64 = 1.;
const SECS_PER_MIN: f64 = 60.;
const HRS_PER_DAY: f64 = 24.;

pub fn get_hours(
    params: &Params,
    top_astro_day: &TopAstroDay,
    weather: Weather,
) -> HashMap<Prayer, Result<f64, ()>> {
    use Prayer::*;

    let (shur_hour_res, dhuhr_hour, magh_hour_res) = get_shur_dhuhr_magh(top_astro_day, weather);

    let (fajr_hour_res, isha_hour_res) = get_fajr_isha(top_astro_day, params, dhuhr_hour);

    let asr_hour_res = get_asr(top_astro_day, params.asr_shadow_ratio, dhuhr_hour);

    let mut hours = HashMap::new();
    hours.insert(Fajr, fajr_hour_res);
    hours.insert(Shurooq, shur_hour_res);
    hours.insert(Dhuhr, Ok(dhuhr_hour));
    hours.insert(Asr, asr_hour_res);
    hours.insert(Maghrib, magh_hour_res);
    hours.insert(Isha, isha_hour_res);

    hours
}

pub fn to_time(params: &Params, hour: f64, prayer: Prayer) -> NaiveTime {
    use Prayer::*;
    use RoundSeconds::*;

    let mut hour = hour + params.minute_offsets[&prayer] / SECS_PER_MIN;

    if hour < 0. {
        while hour < 0. {
            hour += HRS_PER_DAY;
        }
    }

    let mut min = (hour - hour.floor()) * SECS_PER_MIN;
    let mut sec = (min - min.floor()) * SECS_PER_MIN;

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

    if hour >= HRS_PER_DAY {
        hour = hour.rem(HRS_PER_DAY);
    }

    NaiveTime::from_hms_opt(hour as u32, min as u32, sec as u32).unwrap()
}

fn adj_time(hour: &mut f64, min: &mut f64, sec: &mut f64, sec_cap: f64) {
    if *sec >= sec_cap {
        *hour += 1. / SECS_PER_MIN;
    }

    *min = (*hour - hour.floor()) * SECS_PER_MIN;
    *sec = 0.;
}

fn get_fajr_isha(
    top_astro_day: &TopAstroDay,
    params: &Params,
    dhuhr_hour: f64,
) -> (Result<f64, ()>, Result<f64, ()>) {
    use Prayer::*;

    let lat_rads = f64::from(top_astro_day.coords.latitude).to_radians();
    let dec_rads = top_astro_day.astro().dec.to_radians();
    let c = lat_rads.cos() * dec_rads.cos();
    let s = lat_rads.sin() * dec_rads.sin();
    let fajr_hour = ((-params.angles[&Fajr]).to_radians().sin() - s) / c;
    let isha_hour = ((-params.angles[&Isha]).to_radians().sin() - s) / c;
    let fajr_hour = if fajr_hour < -INVALID_TRIGGER || fajr_hour > INVALID_TRIGGER {
        Err(())
    } else {
        Ok(dhuhr_hour - DEGREES_TO_10_BASE * fajr_hour.acos().to_degrees())
    };
    let isha_hour = if isha_hour < -INVALID_TRIGGER || isha_hour > INVALID_TRIGGER {
        Err(())
    } else {
        Ok(dhuhr_hour + DEGREES_TO_10_BASE * isha_hour.acos().to_degrees())
    };

    (fajr_hour, isha_hour)
}

fn get_asr(
    top_astro_day: &TopAstroDay,
    madhab: AsrShadowRatio,
    dhuhr_hour: f64,
) -> Result<f64, ()> {
    let madhab = madhab as u8 as f64;
    let lat_rads = f64::from(top_astro_day.coords.latitude).to_radians();
    let dec_rads = top_astro_day.astro().dec.to_radians();
    let mut asr_hour = madhab + (lat_rads - dec_rads).abs().tan();
    asr_hour = (1. / asr_hour).atan();
    asr_hour = asr_hour.sin() - lat_rads.sin() * dec_rads.sin();
    asr_hour = asr_hour / (lat_rads.cos() * dec_rads.cos());
    if asr_hour < -INVALID_TRIGGER || asr_hour > INVALID_TRIGGER {
        Err(())
    } else {
        Ok(dhuhr_hour + DEGREES_TO_10_BASE * asr_hour.acos().to_degrees())
    }
}

fn get_ra_deltas(top_astro_day: &TopAstroDay) -> (f64, f64) {
    let mut prev_ra = top_astro_day.prev_astro().ra;
    let mut next_ra = top_astro_day.next_astro().ra;
    let j = 350.;
    let k = 10.;
    if top_astro_day.astro().ra > j && next_ra < k {
        next_ra += DEGREES_IN_CIRCLE;
    }
    if prev_ra > j && top_astro_day.astro().ra < k {
        prev_ra = 0.;
    }
    let delta1 = next_ra - prev_ra;
    let delta2 = next_ra + prev_ra - 2. * top_astro_day.astro().ra;
    (delta1, delta2)
}

fn get_dec_deltas(top_astro_day: &TopAstroDay) -> (f64, f64) {
    let delta1 = top_astro_day.next_astro().dec - top_astro_day.prev_astro().dec;
    let delta2 = top_astro_day.next_astro().dec - 2. * top_astro_day.astro().dec
        + top_astro_day.prev_astro().dec;
    (delta1, delta2)
}

fn get_ra_factor(top_astro_day: &TopAstroDay, ra_deltas: (f64, f64), val: f64) -> f64 {
    let sid_g = (top_astro_day.astro().sid_time + 360.985647 * val).cap_angle_360();
    let a = top_astro_day.astro().ra + val * (ra_deltas.0 + ra_deltas.1 * val) / 2.;
    (sid_g + f64::from(top_astro_day.coords.longitude) - a).cap_angle_between_180()
}

fn get_shur_dhuhr_magh(
    top_astro_day: &TopAstroDay,
    weather: Weather,
) -> (Result<f64, ()>, f64, Result<f64, ()>) {
    let ra_deltas = get_ra_deltas(top_astro_day);

    let dhuhr_factor = (top_astro_day.astro().ra
        - f64::from(top_astro_day.coords.longitude)
        - top_astro_day.astro().sid_time)
        / DEGREES_IN_CIRCLE;
    let dhuhr_factor_cap = dhuhr_factor.cap_angle_1();
    let dhuhr_ra_factor = get_ra_factor(top_astro_day, ra_deltas, dhuhr_factor_cap);
    let dhuhr_hour = HRS_PER_DAY * (dhuhr_factor_cap - dhuhr_ra_factor / DEGREES_IN_CIRCLE);

    let shur_magh_res = if let Ok(sm_adj) = get_shur_magh_adj(top_astro_day) {
        let dec_deltas = get_dec_deltas(top_astro_day);

        let shur_factor_cap = (dhuhr_factor - sm_adj).cap_angle_1();
        let shur_ra_factor = get_ra_factor(top_astro_day, ra_deltas, shur_factor_cap);
        let shur_hour = get_shur_magh(
            top_astro_day,
            weather,
            dec_deltas,
            shur_factor_cap,
            shur_ra_factor,
        );

        let magh_factor_cap = (dhuhr_factor + sm_adj).cap_angle_1();
        let magh_ra_factor = get_ra_factor(top_astro_day, ra_deltas, magh_factor_cap);
        let magh_hour = get_shur_magh(
            top_astro_day,
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
    let s = 1.02
        / ((sun_alt + (10.3 / (sun_alt + 5.11)))
            .to_radians()
            .tan()
            .to_degrees()
            + 0.0019279);
    let refraction = w * s / 60.;
    refraction
}

fn get_shur_magh_adj(top_astro_day: &TopAstroDay) -> Result<f64, ()> {
    let lat_rads = f64::from(top_astro_day.coords.latitude).to_radians();
    let dec_rads = top_astro_day.astro().dec.to_radians();
    let n = CENTER_OF_SUN_ANGLE.to_radians().sin() - lat_rads.sin() * dec_rads.sin();
    let d = lat_rads.cos() * dec_rads.cos();
    let r = n / d;
    if r < -INVALID_TRIGGER || r > INVALID_TRIGGER {
        return Err(());
    }

    let adj = r.acos().to_degrees().cap_angle_180() / DEGREES_IN_CIRCLE;
    Ok(adj)
}

fn get_shur_magh(
    top_astro_day: &TopAstroDay,
    weather: Weather,
    dec_deltas: (f64, f64),
    factor_cap: f64,
    ra_factor: f64,
) -> f64 {
    let dec_rads = (top_astro_day.astro().dec
        + factor_cap * (dec_deltas.0 + dec_deltas.1 * factor_cap) / 2.)
        .to_radians();
    let lat_rads = f64::from(top_astro_day.coords.latitude).to_radians();
    let th = ra_factor.to_radians() - top_astro_day.astro().dra;
    let mut sun_alt = (lat_rads.sin() * dec_rads.sin()
        + lat_rads.cos() * dec_rads.cos() * th.cos())
    .asin()
    .to_degrees();
    sun_alt += get_refraction(weather, sun_alt);
    let refr_factor = REFRACTION_ALTITUDE * f64::from(top_astro_day.coords.elevation).powf(0.5);
    let hour = HRS_PER_DAY
        * (factor_cap
            + (sun_alt - CENTER_OF_SUN_ANGLE + refr_factor)
                / (DEGREES_IN_CIRCLE * dec_rads.cos() * lat_rads.cos() * th.sin()));
    hour
}
