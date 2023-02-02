use std::collections::HashMap;

use crate::{angle::Angle, geo::coordinates::Latitude};

use super::{ext_lat::ExtremeLatitudeMethod, Prayer};

pub enum Method {
    Egyptian,
    Shafi,
    Hanafi,
    Isna,
    Mwl,
    UmmAlQurra,
    FixedIsha,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RoundSecondsMethod {
    NoRounding,
    NormalRounding,
    SpecialRounding,
    AggressiveRounding,
}

#[derive(Debug, Clone, Copy)]
pub enum AsrShadowRatioMethod {
    Shafi = 1,
    Hanafi,
}

pub struct Params {
    pub method: Method,
    pub nearest_latitude: Latitude,
    pub round_seconds: RoundSecondsMethod,
    pub asr_shadow_ratio: AsrShadowRatioMethod,
    pub extreme_latitude: ExtremeLatitudeMethod,
    pub angles: HashMap<Prayer, Angle>,
    pub intervals: HashMap<Prayer, Angle>,
    pub minute_offsets: HashMap<Prayer, f64>,
}

impl Params {
    pub fn new(method: Method) -> Self {
        use ExtremeLatitudeMethod::*;
        use Method::*;
        use Prayer::*;
        use RoundSecondsMethod::*;

        let mut asr_shadow_ratio = AsrShadowRatioMethod::Shafi;
        let mut angles = HashMap::new();
        angles.insert(Imsaak, Angle::from_degrees(1.5));

        let mut intervals = angles.clone();
        intervals.insert(Fajr, Angle::from_degrees(0.));
        intervals.insert(Imsaak, Angle::from_degrees(0.));
        intervals.insert(Isha, Angle::from_degrees(0.));

        let mut minute_offsets = HashMap::new();
        minute_offsets.insert(Fajr, 0.);
        minute_offsets.insert(Shurooq, 0.);
        minute_offsets.insert(Dhuhr, 0.);
        minute_offsets.insert(Asr, 0.);
        minute_offsets.insert(Maghrib, 0.);
        minute_offsets.insert(Isha, 0.);
        minute_offsets.insert(Imsaak, 0.);

        match method {
            Egyptian => {
                angles.insert(Fajr, Angle::from_degrees(20.));
                angles.insert(Isha, Angle::from_degrees(18.));
            }
            Shafi => {
                angles.insert(Fajr, Angle::from_degrees(18.));
                angles.insert(Isha, Angle::from_degrees(18.));
            }
            Hanafi => {
                angles.insert(Fajr, Angle::from_degrees(18.));
                angles.insert(Isha, Angle::from_degrees(18.));
                asr_shadow_ratio = AsrShadowRatioMethod::Hanafi;
            }
            Isna => {
                angles.insert(Fajr, Angle::from_degrees(15.));
                angles.insert(Isha, Angle::from_degrees(15.));
            }
            Mwl => {
                angles.insert(Fajr, Angle::from_degrees(18.));
                angles.insert(Isha, Angle::from_degrees(17.));
            }
            UmmAlQurra => {
                angles.insert(Fajr, Angle::from_degrees(19.));
                angles.insert(Isha, Angle::from_degrees(0.));
                intervals
                    .entry(Isha)
                    .and_modify(|x| *x = Angle::from_degrees(90.));
            }
            FixedIsha => {
                angles.insert(Fajr, Angle::from_degrees(19.5));
                angles.insert(Isha, Angle::from_degrees(0.));
                intervals
                    .entry(Isha)
                    .and_modify(|x| *x = Angle::from_degrees(90.));
            }
        }

        Self {
            method,
            nearest_latitude: Latitude::new(48.5).unwrap(),
            round_seconds: SpecialRounding,
            asr_shadow_ratio,
            extreme_latitude: NearestGoodDayFajrIshaInvalid,
            angles,
            intervals,
            minute_offsets,
        }
    }
}

impl Default for Params {
    fn default() -> Self {
        Self::new(Method::Isna)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Pressure(f64);

impl Pressure {
    pub fn new(pressure: f64) -> Result<Self, ()> {
        // TODO: Add invariant.
        Ok(Self(pressure))
    }
}

impl From<Pressure> for f64 {
    fn from(value: Pressure) -> Self {
        value.0
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Temperature(f64);

impl Temperature {
    pub fn new(temperature: f64) -> Result<Self, ()> {
        // TODO: Add invariant.
        Ok(Self(temperature))
    }
}

impl From<Temperature> for f64 {
    fn from(value: Temperature) -> Self {
        value.0
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Weather {
    pub pressure: Pressure,
    pub temperature: Temperature,
}

impl Default for Weather {
    fn default() -> Self {
        Self {
            pressure: Pressure::new(1010.).unwrap(),
            temperature: Temperature::new(10.).unwrap(),
        }
    }
}