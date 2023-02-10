pub mod params;

mod ext_lat;
mod hours;

use std::collections::HashMap;

use chrono::{NaiveDate, NaiveTime};

use crate::{
    geo::{
        astro::TopAstroDay,
        coordinates::{Coordinates, Gmt},
        julian_day::JulianDay,
    },
    prayer_times::{ext_lat::adj_for_ext_lat, hours::get_hours},
};

use self::{
    ext_lat::PrayerHour,
    hours::hour_to_time,
    params::{Params, Weather},
};

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

#[derive(Debug, Clone, Copy)]
pub struct DateRange {
    start_date: NaiveDate,
    end_date: NaiveDate,
}

impl DateRange {
    pub fn new(start_date: NaiveDate, end_date: NaiveDate) -> Result<Self, ()> {
        if start_date > end_date {
            Err(())
        } else {
            Ok(Self {
                start_date,
                end_date,
            })
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Location {
    pub coords: Coordinates,
    pub gmt: Gmt,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PrayerTime {
    pub time: NaiveTime,
    pub extreme: bool,
}

pub fn times_dt_rng(
    params: &Params,
    location: Location,
    date_range: DateRange,
) -> HashMap<NaiveDate, HashMap<Prayer, Result<PrayerTime, ()>>> {
    let dur = date_range.end_date - date_range.start_date;
    let days = (dur.num_days() + 1) as usize;
    let mut times = HashMap::with_capacity(days);
    for date in date_range.start_date.iter_days().take(days) {
        let prayer_time = times_dt(params, location, date, None);
        times.insert(date, prayer_time);
    }

    times
}

pub fn times_dt(
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
    let hours = get_hours(params, &top_astro_day, weather);
    adj_for_ext_lat(hours, params, &top_astro_day, weather)
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
        *params_adj.min_offsets.get_mut(&Fajr).unwrap() -= params.intervals[&Imsaak];
    } else {
        *params_adj.angles.get_mut(&Fajr).unwrap() += params.angles[&Imsaak];
    }

    let mut hours = get_hours_adj_ext(&params_adj, &top_astro_day, weather);
    if let Ok(hour) = hours[&Fajr] {
        if hour.extreme {
            params_adj = params.clone();
            *params_adj.min_offsets.get_mut(&Fajr).unwrap() -= if params.intervals[&Imsaak] == 0. {
                Params::DEF_IMSAAK_ANGLE
            } else {
                params.intervals[&Imsaak]
            };

            hours = get_hours_adj_ext(&params_adj, &top_astro_day, weather);
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
