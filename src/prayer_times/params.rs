//! Islamic prayer times calculation parameters.
//!
//! Type [`Params`] represents a set of configurable fields used in the calculation of Islamic prayer times:
//!
//! * [`Round Seconds`] represents how to round up a prayer time to the nearest minute when a specific
//! threshold of seconds is exceeded for a set of prayer times.
//! * [`Asr Shadow Ratio`] represents the Fiqh school to use when calculating Asr prayer time.
//! * [`Extreme Latitude Method`] represents how to adjust a prayer time when its conventional calculation
//! results in an invalid value due to an extreme latitude for a set of prayer times.
//! * [`Angles`] is a [`map`] of [`Prayer`] keys to angle values in degrees.
//! * [`Intervals`] is a [`map`] of [`Prayer`] keys to interval values in minutes.
//! * [`Minutes`] is a [`map`] of [`Prayer`] keys to minute values used to adjust a calculated
//! prayer time by the keyed value when it is not approximate enough.
//!
//! Type [`Params`] is instantiated using [`new`](Params::new) by providing it a Fiqh [`Method`]
//! to initialize an instance of the type to the desired Fiqh school values. Fiqh [`Method`]
//! [`None`](Method::None) represents the initial values set by the method and the [`Default`]
//! value of [`Params`] is instantiated using it. All other enumerations of [`Method`] effectively
//! override these values.
//!
//! # Initialization
//!
//! [`None`](Method::None)
//! * [`Round Seconds`] is set to [`Special Rounding`](RoundSeconds::SpecialRounding).
//! * [`Asr Shadow Ratio`] is set to [`Shafi`](AsrShadowRatio::Shafi).
//! * [`Extreme Latitude Method`] is set to [`Nearest Good Day Fajr Isha Invalid`](ExtremeLatitudeMethod::NearestGoodDayFajrIshaInvalid).
//! * [`Angles`], [`Intervals`], and [`Minutes`] [`map`]s have their [`Prayer`] values set to 0.
//!
//! [`Egyptian`](Method::Egyptian)
//! * [`Angles`] for [`Fajr`] is set to 20.
//! * [`Angles`] for [`Isha`] is set to 18.
//!
//! [`Egypt`](Method::Egypt)
//! * [`Angles`] for [`Fajr`] is set to 19.5.
//! * [`Angles`] for [`Isha`] is set to 17.5.
//!
//! [`Shafi`](Method::Shafi)
//! * [`Angles`] for [`Fajr`] is set to 18.
//! * [`Angles`] for [`Isha`] is set to 18.
//!
//! [`Hanafi`](Method::Hanafi)
//! * [`Angles`] for [`Fajr`] is set to 18.
//! * [`Angles`] for [`Isha`] is set to 18.
//! * [`Asr Shadow Ratio`] is set to [`Hanafi`](AsrShadowRatio::Hanafi).
//!
//! [`Isna`](Method::Isna)
//! * [`Angles`] for [`Fajr`] is set to 15.
//! * [`Angles`] for [`Isha`] is set to 15.
//!
//! [`Mwl`](Method::Mwl)
//! * [`Angles`] for [`Fajr`] is set to 18.
//! * [`Angles`] for [`Isha`] is set to 17.
//!
//! [`UmmAlQurra`](Method::UmmAlQurra)
//! * [`Angles`] for [`Fajr`] is set to 18.
//! * [`Intervals`] for [`Isha`] is set to 90 minutes after Maghrib prayer.
//!
//! [`FixedIsha`](Method::FixedIsha)
//! * [`Angles`] for [`Fajr`] is set to  19.5.
//! * [`Intervals`] for [`Isha`] is set to  90 minutes after Maghrib prayer.
//!
//! The above Fiqh [`Method`] names are used in a very tentative manner as none of their
//! respective organizations have been contacted to obtain the correct (or up-to-date)
//! values published by the them. Since all fields on the [`Params`] are public they can
//! be explicitly set if their values need to be adjusted.
//!
//! # Extreme Latitude Calculation
//!
//! At certain locations and times of the year, some prayer times do not occur or
//! are otherwise impossible to precisely calculate using conventional means.
//! The [`Extreme Latitude Method`] enumeration represents the method used to
//! adjust the prayer times if the calculation returns invalid value(s), which
//! generally occur at latitudes of 49 degrees or above. The general categories are:
//!
//! * Nearest Latitude (Aqrab Al-Bilaad): Calculate prayer times using a nearest [`Latitude`].
//! * Nearest Good Day (Aqrab Al-Ayyam): Determine the closest previous or next day
//! where [`Fajr`] and [`Isha`] prayer times are both valid.
//! * An amount of night or day: Unlike the above mentioned methods, the multiple
//! methods in this category have no proof in traditional Fiqh resources. These
//! methods were introduced by modern day Muslim scholars and scientists for
//! practical reasons only.
//! * Minutes from Shurooq/Maghrib: Use an interval time to calculate Fajr and Isha prayer
//! times. This will set their calculated values to those of Shurooq and Maghrib respectively,
//! then adjust them by minute vlaues found in their respective values in [`Intervals`].
//!  
//! [`Extreme Latitude Method`]: Params::extreme_latitude_method
//! [`map`]: std::collections::HashMap
//! [`Latitude`]: crate::geo::coordinates::Latitude
//! [`Round Seconds`]: RoundSeconds
//! [`Asr Shadow Ratio`]: AsrShadowRatio
//! [`Angles`]: Params::angles
//! [`Intervals`]: Params::intervals
//! [`Minutes`]: Params::minutes
//! [`Prayer`]: super::Prayer
//! [`Fajr`]: super::Prayer::Fajr
//! [`Imsaak`]: super::Prayer::Imsaak
//! [`Dhuhr`]: super::Prayer::Dhuhr
//! [`Asr`]: super::Prayer::Asr
//! [`Shurooq`]: super::Prayer::Shurooq
//! [`Maghrib`]: super::Prayer::Maghrib
//! [`Isha`]: super::Prayer::Isha

use std::collections::HashMap;

use crate::geo::coordinates::Latitude;

use super::Prayer;

/// The `Method` type. See [the module level documentation](self) for more.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Method {
    /// None
    None,
    /// Egyptian General Authority of Survey
    ///
    /// Used in: Indonesia, Iraq, Jordan, Lebanon, Malaysia, Singapore, Syria, parts of Africa, parts of the United States
    Egyptian,
    /// Egyptian General Authority of Survey
    ///
    /// Used in: Egypt
    Egypt,
    /// University of Islamic Sciences, Karachi (Shafi)
    ///
    /// Used in: Iran, Kuwait, parts of Europe
    Shafi,
    /// University of Islamic Sciences, Karachi (Hanafi)
    ///
    /// Used in: Afghanistan, Bangladesh, India
    Hanafi,
    /// Islamic Society of North America
    ///
    /// Used in: Canada, parts of the UK, parts of the United States
    Isna,
    /// Muslim World League
    ///
    /// Used in: Canada, parts of the UK, parts of the United States
    Mwl,
    /// Umm Al-Qurra University
    ///
    /// Used in: Saudi Arabia
    UmmAlQurra,
    /// Fixed Ishaa Angle Interval
    ///
    /// Used in: Bahrain, Oman, Qatar, United Arab Emirates  
    FixedIsha,
}

/// The `ExtremeLatitudeMethod` type. See [the module level documentation](self) for more.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ExtremeLatitudeMethod {
    /// No adjustment. Prayer times can be invalid.
    None,
    /// Apply a portion of the night based on the angles for Fajr and Isha prayer
    /// times only if their calculated prayer times are invalid.
    AngleBased,
    /// Apply the nearest `Latitude` to all prayer times always.
    NearestLatitudeAllPrayersAlways(Latitude),
    /// Apply the nearest `Latitude` to Fajr and Isha prayer times always.
    NearestLatitudeFajrIshaAlways(Latitude),
    /// Apply the nearest `Latitude` only to an invalid Fajr or Isha prayer time.
    NearestLatitudeFajrIshaInvalid(Latitude),
    /// Apply nearest good day prayer time to all prayer times always.
    NearestGoodDayAllPrayersAlways,
    /// Apply nearest good day prayer time only to an invalid Fajr or Isha prayer time.
    NearestGoodDayFajrIshaInvalid,
    /// Apply one-seventh of the night adjustment to Fajr and Isha prayer times always.
    SeventhOfNightFajrIshaAlways,
    /// Apply one-seventh of the night adjustment only to an invalid Fajr or Isha prayer time.
    SeventhOfNightFajrIshaInvalid,
    /// Apply one-seventh of the day adjustment to Fajr and Isha prayer times always.
    SeventhOfDayFajrIshaAlways,
    /// Apply one-seventh of the day adjustment only to an invalid Fajr or Isha prayer time.
    SeventhOfDayFajrIshaInvalid,
    /// Apply one-half of the night adjustment to Fajr and Isha prayer times always.
    HalfOfNightFajrIshaAlways,
    /// Apply one-half of the night adjustment only to an invalid Fajr or Isha prayer time.
    HalfOfNightFajrIshaInvalid,
    /// Apply [`Fajr`](super::Prayer::Fajr) and [`Isha`](super::Prayer::Isha) values of [`Intervals`](Params::intervals)
    /// to Fajr and Isha prayer times always, respectively.
    MinutesFromMaghribFajrIshaAlways,
    /// Apply [`Fajr`](super::Prayer::Fajr) and [`Isha`](super::Prayer::Isha) values of [`Intervals`](Params::intervals)
    /// only to and invalid Fajr or Isha prayer time, respectively.
    MinutesFromMaghribFajrIshaInvalid,
}

/// The `RoundSeconds` type. See [the module level documentation](self) for more.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RoundSeconds {
    /// No rounding
    None,
    /// Round a prayer time up to the nearest minute if its seconds are greater than
    /// or equal to 30.
    NormalRounding,
    /// Round a Fajr, Dhuhr, Asr, Maghrib, or Isha prayer time up to the nearest minute
    /// if its seconds are greater than or equal to 30.
    SpecialRounding,
    /// Round a Fajr, Dhuhr, Asr, Maghrib, or Isha prayer time up to the nearest minute
    /// if its seconds are greater than or equal to 1.
    AggressiveRounding,
}

/// The `AsrShadowRatio` type. See [the module level documentation](self) for more.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AsrShadowRatio {
    /// Shafi school for calculating Asr prayer time.
    Shafi = 1,
    /// Hanafi school for calculating Asr prayer time.
    Hanafi,
}

/// The `Params` type. See [the module level documentation](self) for more.
#[derive(Debug, Clone)]
pub struct Params {
    pub round_seconds: RoundSeconds,
    pub asr_shadow_ratio: AsrShadowRatio,
    pub extreme_latitude_method: ExtremeLatitudeMethod,
    pub angles: HashMap<Prayer, f64>,
    pub intervals: HashMap<Prayer, f64>,
    pub minutes: HashMap<Prayer, f64>,
}

impl Params {
    /// Default Imsaak angle value.
    pub const DEF_IMSAAK_ANGLE: f64 = 1.5;

    /// Creates a new [`Params`] with default values according to Fiqh [`Method`].
    /// See [the module level documentation](self) for more.
    ///
    /// # Examples
    ///
    /// ```
    /// use islamic_prayer_times::{Method, Params, Prayer};
    ///
    /// let params: Params = Params::new(Method::Isna);
    /// assert_eq!(15., params.angles[&Prayer::Fajr]);
    /// ```
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

        let mut minutes = HashMap::new();
        minutes.insert(Imsaak, 0.);
        minutes.insert(Fajr, 0.);
        minutes.insert(Shurooq, 0.);
        minutes.insert(Dhuhr, 0.);
        minutes.insert(Asr, 0.);
        minutes.insert(Maghrib, 0.);
        minutes.insert(Isha, 0.);

        match method {
            Method::None => {
                angles.insert(Fajr, 0.);
                angles.insert(Isha, 0.);
            }
            Egyptian => {
                angles.insert(Fajr, 20.);
                angles.insert(Isha, 18.);
            }
            Egypt => {
                angles.insert(Fajr, 19.5);
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
            round_seconds: SpecialRounding,
            asr_shadow_ratio,
            extreme_latitude_method: ExtremeLatitudeMethod::NearestGoodDayFajrIshaInvalid,
            angles,
            intervals,
            minutes,
        }
    }
}

impl Default for Params {
    fn default() -> Self {
        Self::new(Method::None)
    }
}
