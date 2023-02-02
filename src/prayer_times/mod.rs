use std::collections::HashMap;

use chrono::{NaiveDate, NaiveTime};

use crate::geo::coordinates::Coordinates;

use self::params::{Params, Weather};

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

pub fn get_prayer_times(
    params: &Params,
    coords: Coordinates,
    date_range: DateRange,
) -> HashMap<NaiveDate, HashMap<Prayer, NaiveTime>> {
    todo!()
}

pub fn get_prayer_time(
    params: &Params,
    coords: Coordinates,
    date: NaiveDate,
    weather: Option<Weather>,
) -> HashMap<Prayer, NaiveTime> {
    todo!()
}

mod ext_lat;
mod hours;
mod params;
