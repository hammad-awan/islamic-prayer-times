mod ext_lat;
mod hours;
pub mod params;

use std::collections::HashMap;

use chrono::{FixedOffset, NaiveDate, NaiveDateTime, NaiveTime, TimeZone};

use crate::{
    geo::{
        astro::TopAstroDay,
        coordinates::{Coordinates, Gmt},
        julian_day::JulianDay,
    },
    prayer_times::{ext_lat::adj_for_ext_lat, hours::get_hours},
};

use self::{
    hours::to_time,
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

pub fn get_prayer_times(
    params: &Params,
    location: &Location,
    date_range: DateRange,
) -> HashMap<NaiveDate, HashMap<Prayer, Result<PrayerTime, ()>>> {
    let days = ((date_range.end_date - date_range.start_date).num_days() + 1) as usize;

    let gmt_secs = (f64::from(location.gmt) * 3600.) as i32;
    let tz_offset = if gmt_secs >= 0 {
        FixedOffset::east_opt(gmt_secs)
    } else {
        FixedOffset::west_opt(gmt_secs)
    };

    let tz_offset = tz_offset.unwrap();

    let mut prayer_times = HashMap::new();
    //let prayer_times = HashMap::with_capacity(days as usize);
    for date in date_range.start_date.iter_days().take(days) {
        let julian_day = JulianDay::new(date, location.gmt);
        let top_astro_day = TopAstroDay::from_jd(julian_day, location.coords);

        let weather = Weather::default();
        let hours = get_hours(params, &top_astro_day, weather);
        let hours = adj_for_ext_lat(hours, params, &top_astro_day, weather);

        let prayer_time: HashMap<_, _> = HashMap::from_iter(hours.iter().map(|x| {
            (
                *x.0,
                x.1.map(|y| {
                    let time = to_time(params, y.value, *x.0);
                    let dt = NaiveDateTime::new(date, time);
                    let dt_w_tz = tz_offset.from_local_datetime(&dt).unwrap();

                    PrayerTime {
                        time: dt_w_tz.time(),
                        extreme: y.extreme,
                    }
                }),
            )
        }));

        prayer_times.insert(date, prayer_time);
    }

    prayer_times
}

pub fn get_prayer_time(
    params: &Params,
    location: &Location,
    date: NaiveDate,
    weather: Option<Weather>,
) -> HashMap<Prayer, NaiveTime> {
    todo!()
}
