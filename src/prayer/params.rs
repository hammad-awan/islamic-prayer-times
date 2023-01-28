use std::collections::HashMap;

use crate::{angle::Angle, geo::coordinates::Latitude};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Prayer {
    Fajr,
    Shurooq,
    Dhuhr,
    Asr,
    Maghrib,
    Isha,
    Imsaak,
}

pub enum Method {
    Egyptian,
    Shafi,
    Hanafi,
    Isna,
    Mwl,
    UmmAlQurra,
    FixedIsha,
}

pub enum RoundSecondsMethod {
    NoRounding,
    NormalRounding,
    SpecialRounding,
    AggressiveRounding,
}

pub enum AsrShadowRatioMethod {
    Shafi = 1,
    Hanafi,
}

pub enum ExtremeLatitudeMethod {
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

pub struct Params {
    method: Method,
    nearest_latitude: Latitude,
    round_seconds: RoundSecondsMethod,
    asr_shadow_ratio: AsrShadowRatioMethod,
    extreme_latitude: ExtremeLatitudeMethod,
    angles: HashMap<Prayer, Angle>,
    intervals: HashMap<Prayer, Angle>,
    minute_offsets: HashMap<Prayer, f64>,
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
