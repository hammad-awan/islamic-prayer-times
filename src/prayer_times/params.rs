use std::collections::HashMap;

use crate::{geo::coordinates::Latitude, OutOfRange};

use super::Prayer;

#[derive(Debug, Clone, Copy)]
pub enum Method {
    None,
    Egyptian,
    Shafi,
    Hanafi,
    Isna,
    Mwl,
    UmmAlQurra,
    FixedIsha,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ExtremeLatitudeMethod {
    None,
    AngleBased,
    NearestLatitudeAllPrayersAlways,
    NearestLatitudeFajrIshaAlways,
    NearestLatitudeFajrIshaInvalid,
    NearestGoodDayAllPrayersAlways,
    NearestGoodDayFajrIshaInvalid,
    SeventhOfNightFajrIshaAlways,
    SeventhOfNightFajrIshaInvalid,
    SeventhOfDayFajrIshaAlways,
    SeventhOfDayFajrIshaInvalid,
    HalfOfNightFajrIshaAlways,
    HalfOfNightFajrIshaInvalid,
    MinutesFromMaghribFajrIshaAlways,
    MinutesFromMaghribFajrIshaInvalid,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RoundSeconds {
    None,
    NormalRounding,
    SpecialRounding,
    AggressiveRounding,
}

#[derive(Debug, Clone, Copy)]
pub enum AsrShadowRatio {
    Shafi = 1,
    Hanafi,
}

#[derive(Debug, Clone)]
pub struct Params {
    pub method: Method,
    pub near_lat: Latitude,
    pub round_seconds: RoundSeconds,
    pub asr_shadow_ratio: AsrShadowRatio,
    pub ext_lat_method: ExtremeLatitudeMethod,
    pub angles: HashMap<Prayer, f64>,
    pub intervals: HashMap<Prayer, f64>,
    pub min_offsets: HashMap<Prayer, f64>,
}

impl Params {
    pub const DEF_IMSAAK_ANGLE: f64 = 1.5;

    pub fn new(method: Method) -> Self {
        use Method::*;
        use Prayer::*;
        use RoundSeconds::*;

        let mut asr_shadow_ratio = AsrShadowRatio::Shafi;

        let mut angles = HashMap::new();
        angles.insert(Imsaak, Self::DEF_IMSAAK_ANGLE);

        let mut intervals = angles.clone();
        intervals.insert(Fajr, 0.);
        intervals.insert(Imsaak, 0.);
        intervals.insert(Isha, 0.);

        let mut min_offsets = HashMap::new();
        min_offsets.insert(Fajr, 0.);
        min_offsets.insert(Shurooq, 0.);
        min_offsets.insert(Dhuhr, 0.);
        min_offsets.insert(Asr, 0.);
        min_offsets.insert(Maghrib, 0.);
        min_offsets.insert(Isha, 0.);
        min_offsets.insert(Imsaak, 0.);

        match method {
            Method::None => {
                angles.insert(Fajr, 0.);
                angles.insert(Isha, 0.);
            }
            Egyptian => {
                angles.insert(Fajr, 20.);
                angles.insert(Isha, 18.);
            }
            Shafi => {
                angles.insert(Fajr, 18.);
                angles.insert(Isha, 18.);
            }
            Hanafi => {
                angles.insert(Fajr, 18.);
                angles.insert(Isha, 18.);
                asr_shadow_ratio = AsrShadowRatio::Hanafi;
            }
            Isna => {
                angles.insert(Fajr, 15.);
                angles.insert(Isha, 15.);
            }
            Mwl => {
                angles.insert(Fajr, 18.);
                angles.insert(Isha, 17.);
            }
            UmmAlQurra => {
                angles.insert(Fajr, 19.);
                angles.insert(Isha, 0.);
                *intervals.get_mut(&Isha).unwrap() = 90.;
            }
            FixedIsha => {
                angles.insert(Fajr, 19.5);
                angles.insert(Isha, 0.);
                *intervals.get_mut(&Isha).unwrap() = 90.;
            }
        }

        Self {
            method,
            near_lat: Latitude::new(48.5).unwrap(),
            round_seconds: SpecialRounding,
            asr_shadow_ratio,
            ext_lat_method: ExtremeLatitudeMethod::NearestGoodDayFajrIshaInvalid,
            angles,
            intervals,
            min_offsets,
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
    pub fn new(pressure: f64) -> Result<Self, OutOfRange> {
        if (100. ..=1050.).contains(&pressure) {
            Ok(Self(pressure))
        } else {
            Err(OutOfRange)
        }
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
    pub fn new(temperature: f64) -> Result<Self, OutOfRange> {
        if (-90. ..=57.).contains(&temperature) {
            Ok(Self(temperature))
        } else {
            Err(OutOfRange)
        }
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
            temperature: Temperature::new(14.).unwrap(),
        }
    }
}
