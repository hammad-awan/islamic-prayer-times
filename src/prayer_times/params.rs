use std::collections::HashMap;

use crate::geo::coordinates::Latitude;

use super::Prayer;

#[derive(Debug, Clone, Copy, PartialEq)]
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

#[derive(Debug, Clone, Copy, PartialEq)]
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
        min_offsets.insert(Imsaak, 0.);
        min_offsets.insert(Fajr, 0.);
        min_offsets.insert(Shurooq, 0.);
        min_offsets.insert(Dhuhr, 0.);
        min_offsets.insert(Asr, 0.);
        min_offsets.insert(Maghrib, 0.);
        min_offsets.insert(Isha, 0.);

        match method {
            Method::None => {
                angles.insert(Fajr, 0.);
                angles.insert(Isha, 0.);
            }
            Egyptian => {
                angles.insert(Fajr, 19.);
                angles.insert(Isha, 17.5);
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
                angles.insert(Fajr, 18.);
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
