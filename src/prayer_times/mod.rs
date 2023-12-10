pub mod date;
pub mod params;

pub use date::*;
pub use params::*;

use serde::{Deserialize, Serialize};

mod ext_lat;
mod hours;

use std::{
    collections::{BTreeMap, HashMap},
    fmt::Display,
    sync::mpsc::channel,
    thread::{self},
};

use chrono::{NaiveDate, NaiveTime};

use crate::{
    geo::{astro::TopAstroDay, coordinates::Location, julian_day::JulianDay},
    prayer_times::{ext_lat::adj_for_ext_lat, hours::get_hours},
    Weather,
};

use self::{ext_lat::PrayerHour, hours::hour_to_time};

/// An enumeration of Islamic prayer and related times.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
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

/// A calculated Islamic [`Prayer`] time that is possibly considered extreme.
///
/// See [`params` module level documentation](params) for more information on extreme latitude
/// calculation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
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

/// Returns a [`B-tree`] of [`NaiveDate`] keys to a [`B-tree`] of [`Prayer`] keys to [`PrayerTime`] values
/// using the specified [`Params`] for a [`Location`] and [`DateRange`].
///
/// [`B-tree`]: std::collections::BTreeMap
///
/// # Examples
/// ```
/// # use std::error::Error;
/// use chrono::NaiveDate;
/// use islamic_prayer_times::*;
///
/// let params = Params::default();
/// let latitude = Latitude::try_from(39.)?;
/// let longitude = Longitude::try_from(-77.)?;
/// let elevation = Elevation::try_from(0.)?;
/// let coords = Coordinates::new(latitude, longitude, elevation);
/// let gmt = Gmt::try_from(-5.)?;
/// let location = Location { coords, gmt };
/// let start_date = NaiveDate::from_ymd_opt(2023, 1, 1).unwrap();
/// let end_date = NaiveDate::from_ymd_opt(2023, 1, 31).unwrap();
/// let date_range = DateRange::from(start_date..=end_date);
///
/// let prayer_times_rng = prayer_times_dt_rng(&params, location, &date_range);
/// let prayer_times_date = prayer_times_rng.get(&start_date).unwrap();
///
/// assert_eq!(31, prayer_times_rng.len());
/// assert_eq!(7, prayer_times_date.len());
/// # Ok::<(), Box<dyn Error>>(())
/// ```
pub fn prayer_times_dt_rng(
    params: &Params,
    location: Location,
    date_range: &DateRange,
) -> BTreeMap<NaiveDate, BTreeMap<Prayer, Result<PrayerTime, ()>>> {
    let mut times = BTreeMap::new();
    for date in date_range
        .start_date()
        .iter_days()
        .take(date_range.num_days())
    {
        let prayer_time = prayer_times_dt(params, location, date, None);
        times.insert(date, prayer_time);
    }
    times
}

/// Returns a [`B-tree`] of [`NaiveDate`] keys to a [`B-tree`] of [`Prayer`] keys to [`PrayerTime`] values
/// using the specified [`Params`] for a [`Location`] and [`DateRange`] while maximizing parallelism for a
/// (possibly) large number of calculations.
///
/// [`B-tree`]: std::collections::BTreeMap
///
pub fn prayer_times_dt_rng_block(
    params: &Params,
    location: Location,
    date_range: &DateRange,
    min_days_for_pll: usize,
) -> BTreeMap<NaiveDate, BTreeMap<Prayer, Result<PrayerTime, ()>>> {
    // Determine parallelism.
    let avail_pll = if let Ok(count) = thread::available_parallelism() {
        count.get()
    } else {
        1
    };
    let no_parallelism = date_range.num_days() / avail_pll < min_days_for_pll;

    // No parallelism.
    if avail_pll == 1 || no_parallelism {
        prayer_times_dt_rng(params, location, date_range)
    } else {
        // Maximize parallelism through threads that calculate partial time results.
        thread::scope(|s| {
            let (tx, rx) = channel();

            // Spawn thread to combine prayer times for each date range.
            let handle = s.spawn(move || {
                let mut times = BTreeMap::new();
                while let Ok(mut partial_times) = rx.recv() {
                    times.append(&mut partial_times);
                }
                times
            });

            // Spawn threads to calculate prayer times for each date range.
            let date_ranges = date_range.partition(avail_pll);
            for date_range in date_ranges {
                let tx = tx.clone();
                s.spawn(move || {
                    let partial_times = prayer_times_dt_rng(params, location, &date_range);
                    tx.send(partial_times).unwrap();
                });
            }

            // Close channel to terminate blocking channel receive loop.
            drop(tx);

            handle.join().unwrap()
        })
    }
}

/// Returns a [`B-tree`](std::collections::BTreeMap) of [`Prayer`] keys to [`PrayerTime`] values using the specified
/// [`Params`] for a [`Location`], [`NaiveDate`], and its (optional) current [`Weather`].
///
/// # Examples
///
/// ```
/// # use std::error::Error;
/// use chrono::NaiveDate;
/// use islamic_prayer_times::*;
///
/// let params = Params::default();
/// let latitude = Latitude::try_from(39.)?;
/// let longitude = Longitude::try_from(-77.)?;
/// let elevation = Elevation::try_from(0.)?;
/// let coords = Coordinates::new(latitude, longitude, elevation);
/// let gmt = Gmt::try_from(-5.)?;
/// let location = Location { coords, gmt };
/// let date = NaiveDate::from_ymd_opt(2023, 2, 6).unwrap();
///
/// let prayer_times = prayer_times_dt(&params, location, date, None);
///
/// assert_eq!(7, prayer_times.len());
/// # Ok::<(), Box<dyn Error>>(())
/// ```
pub fn prayer_times_dt(
    params: &Params,
    location: Location,
    date: NaiveDate,
    weather: Option<Weather>,
) -> BTreeMap<Prayer, Result<PrayerTime, ()>> {
    use Prayer::*;

    let weather = if let Some(weather) = weather {
        weather
    } else {
        Weather::default()
    };

    let julian_day = JulianDay::new(date, location.gmt);
    let top_astro_day = TopAstroDay::from_jd(julian_day, location.coords);
    let hours = get_hours_adj_ext(params, &top_astro_day, weather);
    let mut times = BTreeMap::from_iter(
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
