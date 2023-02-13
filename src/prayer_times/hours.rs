use std::{collections::HashMap, ops::Rem};

use chrono::NaiveTime;

use crate::{
    angle::{LimitAngle, DEG_IN_CIRCLE},
    geo::astro::TopAstroDay,
    prayer_times::params::{Params, RoundSeconds},
};

use super::{params::Weather, Prayer};

pub const MIN_SEC_PER_HR_MIN: f64 = 60.;

const DEGREES_TO_10_BASE: f64 = 0.066666666666666666;
const INVALID_TRIGGER: f64 = 1.;
const CENTER_OF_SUN_ANGLE: f64 = -0.83337;
const DEF_ROUND_SEC: f64 = 30.;
const AGGRESSIVE_ROUND_SEC: f64 = 1.;
const HRS_PER_DAY: f64 = 24.;

pub fn get_hours(
    params: &Params,
    top_astro_day: &TopAstroDay,
    weather: Weather,
) -> HashMap<Prayer, Result<f64, ()>> {
    use Prayer::*;

    let (shur_hour_res, dhuhr_hour, magh_hour_res) = get_shur_dhuhr_magh(top_astro_day, weather);
    let (fajr_hour_res, isha_hour_res) = get_fajr_isha(params, top_astro_day, dhuhr_hour);
    let asr_hour_res = get_asr(params, top_astro_day, dhuhr_hour);

    let mut hours = HashMap::new();
    hours.insert(Fajr, fajr_hour_res);
    hours.insert(Shurooq, shur_hour_res);
    hours.insert(Dhuhr, Ok(dhuhr_hour));
    hours.insert(Asr, asr_hour_res);
    hours.insert(Maghrib, magh_hour_res);
    hours.insert(Isha, isha_hour_res);
    hours
}

fn get_shur_dhuhr_magh(
    top_astro_day: &TopAstroDay,
    weather: Weather,
) -> (Result<f64, ()>, f64, Result<f64, ()>) {
    // Astronomical Algorithms pg. 100-104
    let ra_interp_deltas = get_ra_interp_deltas(top_astro_day);

    let m_0 = (top_astro_day.astro().ra()
        - f64::from(top_astro_day.coords().longitude)
        - top_astro_day.astro().sid_time())
        / DEG_IN_CIRCLE;
    let dhuhr_m_time = m_0.cap_angle_1();
    let dhuhr_hour_angle = get_hour_angle(top_astro_day, ra_interp_deltas, dhuhr_m_time);
    let dhuhr_delta_m = dhuhr_hour_angle / DEG_IN_CIRCLE;
    let dhuhr_hour = HRS_PER_DAY * (dhuhr_m_time - dhuhr_delta_m);

    let shur_magh_res = if let Ok(sm_m_0_adj) = get_shur_magh_m_0_adj(top_astro_day) {
        let dec_interp_deltas = get_dec_interp_deltas(top_astro_day);

        let shuhr_m_time = (m_0 - sm_m_0_adj).cap_angle_1();
        let shuhr_hour_angle = get_hour_angle(top_astro_day, ra_interp_deltas, shuhr_m_time);
        let shur_hour = get_shur_magh(
            top_astro_day,
            weather,
            dec_interp_deltas,
            shuhr_m_time,
            shuhr_hour_angle,
        );

        let magh_m_time = (m_0 + sm_m_0_adj).cap_angle_1();
        let magh_hour_angle = get_hour_angle(top_astro_day, ra_interp_deltas, magh_m_time);
        let magh_hour = get_shur_magh(
            top_astro_day,
            weather,
            dec_interp_deltas,
            magh_m_time,
            magh_hour_angle,
        );

        (Ok(shur_hour), Ok(magh_hour))
    } else {
        (Err(()), Err(()))
    };

    (shur_magh_res.0, dhuhr_hour, shur_magh_res.1)
}

fn get_ra_interp_deltas(top_astro_day: &TopAstroDay) -> (f64, f64) {
    let mut prev_ra = top_astro_day.prev_astro().ra();
    let mut next_ra = top_astro_day.next_astro().ra();
    let j = 350.;
    let k = 10.;
    if top_astro_day.astro().ra() > j && next_ra < k {
        next_ra += DEG_IN_CIRCLE;
    }
    if prev_ra > j && top_astro_day.astro().ra() < k {
        prev_ra = 0.;
    }
    let delta1 = next_ra - prev_ra;
    let delta2 = next_ra + prev_ra - 2. * top_astro_day.astro().ra();
    (delta1, delta2)
}

fn get_hour_angle(top_astro_day: &TopAstroDay, ra_interp_deltas: (f64, f64), val: f64) -> f64 {
    // Sidereal time at Greenwich
    let sid_time_gw = (top_astro_day.astro().sid_time() + 360.985647 * val).cap_angle_360();
    // Astronomical Algorithms pg. 24 (3.3)
    let ra_interp =
        top_astro_day.astro().ra() + val * (ra_interp_deltas.0 + ra_interp_deltas.1 * val) / 2.;
    (sid_time_gw + f64::from(top_astro_day.coords().longitude) - ra_interp).cap_angle_between_180()
}

fn get_dec_interp_deltas(top_astro_day: &TopAstroDay) -> (f64, f64) {
    let delta1 = top_astro_day.next_astro().dec() - top_astro_day.prev_astro().dec();
    let delta2 = top_astro_day.next_astro().dec() - 2. * top_astro_day.astro().dec()
        + top_astro_day.prev_astro().dec();
    (delta1, delta2)
}

fn get_shur_magh_m_0_adj(top_astro_day: &TopAstroDay) -> Result<f64, ()> {
    // Astronomical Algorithms pg. 102 (15.1)
    let lat_rads = f64::from(top_astro_day.coords().latitude).to_radians();
    let dec_rads = top_astro_day.astro().dec().to_radians();
    let n = CENTER_OF_SUN_ANGLE.to_radians().sin() - lat_rads.sin() * dec_rads.sin();
    let d = lat_rads.cos() * dec_rads.cos();
    let r = n / d;
    if r < -INVALID_TRIGGER || r > INVALID_TRIGGER {
        return Err(());
    }

    // Astronomical Algorithms pg. 102 (15.2)
    let adj = r.acos().to_degrees().cap_angle_180() / DEG_IN_CIRCLE;
    Ok(adj)
}

fn get_shur_magh(
    top_astro_day: &TopAstroDay,
    weather: Weather,
    dec_interp_deltas: (f64, f64),
    m_time: f64,
    hour_angle: f64,
) -> f64 {
    // Astronomical Algorithms pg. 24 (3.3)
    let dec_interp_rads = (top_astro_day.astro().dec()
        + m_time * (dec_interp_deltas.0 + dec_interp_deltas.1 * m_time) / 2.)
        .to_radians();
    let lat_rads = f64::from(top_astro_day.coords().latitude).to_radians();
    // Astronomical Algorithms pg. 93 (13.6)
    // The subtraction to dra is not in the book but it's in the ported code.
    let hour_angle_rads = hour_angle.to_radians() - top_astro_day.astro().dra();
    // "Airless" altitude of sun
    let mut sun_alt = (lat_rads.sin() * dec_interp_rads.sin()
        + lat_rads.cos() * dec_interp_rads.cos() * hour_angle_rads.cos())
    .asin()
    .to_degrees();
    // Astronomical Algorithms pg. 105 (Apparent altitude h0)
    sun_alt += get_refraction(weather, sun_alt);
    // Astronomical Algorithms pg. 103
    let delta_m = (sun_alt - CENTER_OF_SUN_ANGLE)
        / (DEG_IN_CIRCLE * dec_interp_rads.cos() * lat_rads.cos() * hour_angle_rads.sin());
    HRS_PER_DAY * (m_time + delta_m)
}

fn get_refraction(weather: Weather, sun_alt: f64) -> f64 {
    // Astronomical Algorithms pg. 105-107
    // Astronomical Algorithms pg. 106 (16.4)
    let r = 1.02
        / ((sun_alt + (10.3 / (sun_alt + 5.11)))
            .to_radians()
            .tan()
            .to_degrees()
            + 0.0019279);
    // Astronomical Algorithms pg. 107
    let m = f64::from(weather.pressure) / 1010. * (283. / (273. + f64::from(weather.temperature)));
    m * r / 60.
}

fn get_fajr_isha(
    params: &Params,
    top_astro_day: &TopAstroDay,
    dhuhr_hour: f64,
) -> (Result<f64, ()>, Result<f64, ()>) {
    use Prayer::*;

    let lat_rads = f64::from(top_astro_day.coords().latitude).to_radians();
    let dec_rads = top_astro_day.astro().dec().to_radians();
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

fn get_asr(params: &Params, top_astro_day: &TopAstroDay, dhuhr_hour: f64) -> Result<f64, ()> {
    let madhab = params.asr_shadow_ratio as u8 as f64;
    let lat_rads = f64::from(top_astro_day.coords().latitude).to_radians();
    let dec_rads = top_astro_day.astro().dec().to_radians();
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

pub fn hour_to_time(params: &Params, prayer: Prayer, hour: f64) -> NaiveTime {
    use Prayer::*;
    use RoundSeconds::*;

    let mut hour = hour + params.min_offsets[&prayer] / MIN_SEC_PER_HR_MIN;

    if hour < 0. {
        while hour < 0. {
            hour += HRS_PER_DAY;
        }
    }

    let mut min = (hour - hour.floor()) * MIN_SEC_PER_HR_MIN;
    let mut sec = (min - min.floor()) * MIN_SEC_PER_HR_MIN;

    match params.round_seconds {
        NormalRounding => {
            round_secs(&mut hour, &mut min, &mut sec, DEF_ROUND_SEC);
        }
        SpecialRounding | AggressiveRounding => match prayer {
            Fajr | Dhuhr | Asr | Maghrib | Isha => {
                if params.round_seconds == SpecialRounding {
                    round_secs(&mut hour, &mut min, &mut sec, DEF_ROUND_SEC);
                } else {
                    round_secs(&mut hour, &mut min, &mut sec, AGGRESSIVE_ROUND_SEC);
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

fn round_secs(hour: &mut f64, min: &mut f64, sec: &mut f64, sec_cap: f64) {
    if *sec >= sec_cap {
        *hour += 1. / MIN_SEC_PER_HR_MIN;
    }

    *min = (*hour - hour.floor()) * MIN_SEC_PER_HR_MIN;
    *sec = 0.;
}
