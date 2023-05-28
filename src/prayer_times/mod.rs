pub mod params;

pub use params::*;

mod ext_lat;
mod hours;

use std::{
    collections::{BTreeMap, HashMap},
    fmt::Display,
    mem::swap,
    ops::RangeInclusive,
};

use chrono::{NaiveDate, NaiveTime};

use strum::EnumIter;

use crate::{
    error::OutOfRangeError,
    geo::{
        astro::TopAstroDay,
        coordinates::{Coordinates, Gmt},
        julian_day::JulianDay,
    },
    prayer_times::{ext_lat::adj_for_ext_lat, hours::get_hours},
    Bounded,
};

use self::{ext_lat::PrayerHour, hours::hour_to_time};

/// An enumeration of Islamic prayer and other related times.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, EnumIter)]
pub enum Prayer {
    /// Some minutes before Fajr
    Imsaak,
    /// Dawn
    Fajr,
    /// Sunrise
    Shurooq,
    /// Noon
    Dhuhr,
    /// Afternoon
    Asr,
    /// Sunset
    Maghrib,
    /// Night
    Isha,
}

impl Display for Prayer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// A simple date range.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DateRange {
    start_date: NaiveDate,
    end_date: NaiveDate,
}

impl DateRange {
    /// Creates a new `DateRange` with the specified start and end date.
    /// If the specified start date is after the end date, the start date
    /// is used as the end date and vice versa.
    ///
    /// # Examples
    ///
    /// ```
    /// use chrono::NaiveDate;
    /// use islamic_prayer_times::DateRange;
    ///
    /// let mut start_date = NaiveDate::from_ymd_opt(2023, 1, 1).unwrap();
    /// let mut end_date = NaiveDate::from_ymd_opt(2023, 1, 31).unwrap();
    /// let mut date_range = DateRange::new(start_date, end_date);
    ///
    /// assert_eq!(date_range.start_date(), start_date);
    /// assert_eq!(date_range.end_date(), end_date);
    ///
    /// let temp_date = start_date;
    /// start_date = end_date;
    /// end_date = temp_date;
    ///
    /// date_range = DateRange::new(start_date, end_date);
    ///
    /// assert_eq!(date_range.start_date(), end_date);
    /// assert_eq!(date_range.end_date(), start_date);
    /// ```
    pub fn new(mut start_date: NaiveDate, mut end_date: NaiveDate) -> Self {
        if start_date > end_date {
            swap(&mut start_date, &mut end_date);
        }

        Self {
            start_date,
            end_date,
        }
    }

    /// Returns the start date of the `DateRange`.
    pub fn start_date(&self) -> NaiveDate {
        self.start_date
    }

    /// Returns the end date of the `DateRange`.
    pub fn end_date(&self) -> NaiveDate {
        self.end_date
    }
}

impl Display for DateRange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} - {}", self.start_date, self.end_date)
    }
}

/// A location specified by geographical [`Coordinates`](super::geo::coordinates::Coordinates) and [`Gmt`](super::geo::coordinates::Gmt) time.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Location {
    /// Geographical coordinates of the location.
    pub coords: Coordinates,
    /// Greenwich Mean Time of the location.
    pub gmt: Gmt,
}

/// An atmospheric pressure in millibars.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Pressure(f64);

impl Bounded<f64> for Pressure {
    fn range() -> RangeInclusive<f64> {
        100. ..=1050.
    }

    fn new(value: f64) -> Self {
        Self(value)
    }
}

impl From<Pressure> for f64 {
    fn from(value: Pressure) -> Self {
        value.0
    }
}

impl TryFrom<f64> for Pressure {
    type Error = OutOfRangeError<f64>;

    fn try_from(value: f64) -> Result<Self, Self::Error> {
        <Self as Bounded<f64>>::try_from(value)
    }
}

/// An outside temperature in degrees Celcius.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Temperature(f64);

impl Bounded<f64> for Temperature {
    fn range() -> RangeInclusive<f64> {
        -90. ..=57.
    }

    fn new(value: f64) -> Self {
        Self(value)
    }
}

impl From<Temperature> for f64 {
    fn from(value: Temperature) -> Self {
        value.0
    }
}

impl TryFrom<f64> for Temperature {
    type Error = OutOfRangeError<f64>;

    fn try_from(value: f64) -> Result<Self, Self::Error> {
        <Self as Bounded<f64>>::try_from(value)
    }
}

/// Current weather as specified by [`Pressure`] and [`Temperature`].
#[derive(Debug, Clone, Copy)]
pub struct Weather {
    /// Atmospheric pressure
    pub pressure: Pressure,
    /// Outside temperature
    pub temperature: Temperature,
}

impl Default for Weather {
    fn default() -> Self {
        Self {
            pressure: <Pressure as TryFrom<f64>>::try_from(1010.).unwrap(),
            temperature: <Temperature as TryFrom<f64>>::try_from(14.).unwrap(),
        }
    }
}

/// A calculated Islamic [`Prayer`] time that is possibly considered extreme.
///
/// See [`params` module level documentation](params) for more information on extreme latitude
/// calculation.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PrayerTime {
    /// The daily prayer time.
    pub time: NaiveTime,
    /// An extreme latitude method was used to calculate the prayer time.
    pub extreme: bool,
}

impl Display for PrayerTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let fmt_time = self.time.format("%l:%M %p");
        if self.extreme {
            write!(f, "{} (extreme)", fmt_time)
        } else {
            write!(f, "{}", fmt_time)
        }
    }
}

/// Returns a [`B-tree`] of [`NaiveDate`](chrono::NaiveDate) keys to a [`map`] of [`Prayer`] keys to [`PrayerTime`] values
/// using the specified [`Params`](params::Params) for a [`Location`] and [`DateRange`].
///
/// [`B-tree`]: std::collections::BTreeMap
///
/// # Examples
///
/// ```
/// use chrono::NaiveDate;
/// use islamic_prayer_times::*;
///
/// let params = Params::default();
/// let latitude = Latitude::try_from(39.).unwrap();
/// let longitude = Longitude::try_from(-77.).unwrap();
/// let elevation = Elevation::try_from(0.).unwrap();
/// let coords = Coordinates::new(latitude, longitude, elevation);
/// let gmt = Gmt::try_from(-5.).unwrap();
/// let location = Location { coords, gmt };
/// let start_date = NaiveDate::from_ymd_opt(2023, 1, 1).unwrap();
/// let end_date = NaiveDate::from_ymd_opt(2023, 1, 31).unwrap();
/// let date_range = DateRange::new(start_date, end_date);
///
/// let prayer_times_rng = prayer_times_dt_rng(&params, location, date_range);
/// let prayer_times_date = prayer_times_rng.get(&start_date).unwrap();
///
/// assert_eq!(31, prayer_times_rng.len());
/// assert_eq!(7, prayer_times_date.len());
/// ```
pub fn prayer_times_dt_rng(
    params: &Params,
    location: Location,
    date_range: DateRange,
) -> BTreeMap<NaiveDate, HashMap<Prayer, Result<PrayerTime, ()>>> {
    let dur = date_range.end_date - date_range.start_date;
    let days = (dur.num_days() + 1) as usize;
    let mut times = BTreeMap::new();
    for date in date_range.start_date.iter_days().take(days) {
        let prayer_time = prayer_times_dt(params, location, date, None);
        times.insert(date, prayer_time);
    }

    times
}

/// Returns a [`map`](std::collections::HashMap) of [`Prayer`] keys to [`PrayerTime`] values using the specified
/// [`Params`](params::Params) for a [`Location`], [`date`](chrono::NaiveDate), and its (optional) current [`Weather`].
///
/// # Examples
///
/// ```
/// use chrono::NaiveDate;
/// use islamic_prayer_times::*;
///
/// let params = Params::default();
/// let latitude = Latitude::try_from(39.).unwrap();
/// let longitude = Longitude::try_from(-77.).unwrap();
/// let elevation = Elevation::try_from(0.).unwrap();
/// let coords = Coordinates::new(latitude, longitude, elevation);
/// let gmt = Gmt::try_from(-5.).unwrap();
/// let location = Location { coords, gmt };
/// let date = NaiveDate::from_ymd_opt(2023, 2, 6).unwrap();
///
/// let prayer_times = prayer_times_dt(&params, location, date, None);
///
/// assert_eq!(7, prayer_times.len());
/// ```
pub fn prayer_times_dt(
    params: &Params,
    location: Location,
    date: NaiveDate,
    weather: Option<Weather>,
) -> HashMap<Prayer, Result<PrayerTime, ()>> {
    use Prayer::*;

    let weather = if let Some(weather) = weather {
        weather
    } else {
        Weather::default()
    };

    let julian_day = JulianDay::new(date, location.gmt);
    let top_astro_day = TopAstroDay::from_jd(julian_day, location.coords);
    let hours = get_hours_adj_ext(params, &top_astro_day, weather);
    let mut times = HashMap::from_iter(
        hours
            .iter()
            .map(|x| (*x.0, x.1.map(|y| to_prayer_time(params, *x.0, y)))),
    );

    let imsaak = get_imsaak(params, &top_astro_day, weather);
    times.insert(Imsaak, imsaak);
    times
}

fn get_hours_adj_ext(
    params: &Params,
    top_astro_day: &TopAstroDay,
    weather: Weather,
) -> HashMap<Prayer, Result<PrayerHour, ()>> {
    let hours = get_hours(params, top_astro_day, weather);
    adj_for_ext_lat(params, hours, top_astro_day, weather)
}

fn get_imsaak(
    params: &Params,
    top_astro_day: &TopAstroDay,
    weather: Weather,
) -> Result<PrayerTime, ()> {
    use Prayer::*;

    let mut params_adj = params.clone();
    if params.intervals[&Fajr] != 0. {
        *params_adj.intervals.get_mut(&Fajr).unwrap() += if params.intervals[&Imsaak] == 0. {
            Params::DEF_IMSAAK_ANGLE
        } else {
            params.intervals[&Imsaak]
        };
    } else if params.intervals[&Imsaak] != 0. {
        *params_adj.minutes.get_mut(&Fajr).unwrap() -= params.intervals[&Imsaak];
    } else {
        *params_adj.angles.get_mut(&Fajr).unwrap() += params.angles[&Imsaak];
    }

    let mut hours = get_hours_adj_ext(&params_adj, top_astro_day, weather);
    if let Ok(hour) = hours[&Fajr] {
        if hour.extreme {
            params_adj = params.clone();
            *params_adj.minutes.get_mut(&Fajr).unwrap() -= if params.intervals[&Imsaak] == 0. {
                Params::DEF_IMSAAK_ANGLE
            } else {
                params.intervals[&Imsaak]
            };

            hours = get_hours_adj_ext(&params_adj, top_astro_day, weather);
        }
    }

    hours[&Fajr].map(|x| to_prayer_time(&params_adj, Fajr, x))
}

fn to_prayer_time(params: &Params, prayer: Prayer, prayer_hour: PrayerHour) -> PrayerTime {
    PrayerTime {
        time: hour_to_time(params, prayer, prayer_hour.value),
        extreme: prayer_hour.extreme,
    }
}
