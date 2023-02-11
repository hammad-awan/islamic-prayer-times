use std::{cell::RefCell, collections::HashMap};

use chrono::Datelike;

use crate::{
    geo::{astro::TopAstroDay, coordinates::Coordinates, julian_day::JulianDay},
    prayer_times::{hours::get_hours, params::ExtremeLatitudeMethod},
};

use super::{
    params::{Params, Weather},
    Prayer,
};

#[derive(Debug, Clone, Copy)]
pub struct PrayerHour {
    pub value: f64,
    pub extreme: bool,
}

impl PrayerHour {
    fn new(hour: f64) -> Self {
        Self {
            value: hour,
            extreme: false,
        }
    }

    fn new_extreme(hour: f64) -> Self {
        Self {
            value: hour,
            extreme: true,
        }
    }
}

pub fn adj_for_ext_lat(
    hours: HashMap<Prayer, Result<f64, ()>>,
    params: &Params,
    top_astro_day: &TopAstroDay,
    weather: Weather,
) -> HashMap<Prayer, Result<PrayerHour, ()>> {
    use ExtremeLatitudeMethod::*;
    use Prayer::*;

    if !can_adj(&hours, params.ext_lat_method) {
        return HashMap::from_iter(
            hours
                .iter()
                .map(|x| (*x.0, x.1.map(|y| PrayerHour::new(y)))),
        );
    }

    let hours: HashMap<_, _> = HashMap::from_iter(
        hours
            .iter()
            .map(|x| (*x.0, RefCell::new(x.1.map(|y| PrayerHour::new(y))))),
    );

    match params.ext_lat_method {
        NearestLatitudeAllPrayersAlways
        | NearestLatitudeFajrIshaAlways
        | NearestLatitudeFajrIshaInvalid => adj_near_lat(&hours, params, top_astro_day, weather),
        NearestGoodDayAllPrayersAlways | NearestGoodDayFajrIshaInvalid => {
            adj_near_good(&hours, params, top_astro_day, weather)
        }
        SeventhOfNightFajrIshaAlways
        | SeventhOfNightFajrIshaInvalid
        | SeventhOfDayFajrIshaAlways
        | SeventhOfDayFajrIshaInvalid
        | HalfOfNightFajrIshaAlways
        | HalfOfNightFajrIshaInvalid => adj_sev_half(&hours, params),
        MinutesFromMaghribFajrIshaAlways => adj_min_always(&hours),
        MinutesFromMaghribFajrIshaInvalid => adj_min_inv(&hours, params),
        _ => {}
    }

    if params.ext_lat_method != MinutesFromMaghribFajrIshaInvalid
        && params.ext_lat_method != HalfOfNightFajrIshaInvalid
        && params.ext_lat_method != HalfOfNightFajrIshaAlways
    {
        if params.intervals[&Fajr] != 0. {
            *hours[&Fajr].borrow_mut() = hours[&Shurooq].borrow().map(|mut x| {
                x.value -= params.intervals[&Fajr] / 60.;
                x
            });
        }

        if params.intervals[&Isha] != 0. {
            *hours[&Isha].borrow_mut() = hours[&Maghrib].borrow().map(|mut x| {
                x.value += params.intervals[&Isha] / 60.;
                x
            });
        }
    }

    HashMap::from_iter(hours.iter().map(|x| (*x.0, x.1.borrow().map(|y| y))))
}

fn can_adj(
    prayer_hours: &HashMap<Prayer, Result<f64, ()>>,
    ext_lat_meth: ExtremeLatitudeMethod,
) -> bool {
    ext_lat_meth != ExtremeLatitudeMethod::None
        && (has_inv_hours(prayer_hours) || is_always_ext_lat(ext_lat_meth))
}

fn has_inv_hours(prayer_hours: &HashMap<Prayer, Result<f64, ()>>) -> bool {
    prayer_hours.into_iter().any(|x| x.1.is_err())
}

fn is_always_ext_lat(ext_lat_meth: ExtremeLatitudeMethod) -> bool {
    use ExtremeLatitudeMethod::*;

    match ext_lat_meth {
        NearestLatitudeAllPrayersAlways
        | NearestLatitudeFajrIshaAlways
        | NearestGoodDayAllPrayersAlways
        | SeventhOfNightFajrIshaAlways
        | SeventhOfDayFajrIshaAlways
        | HalfOfNightFajrIshaAlways
        | MinutesFromMaghribFajrIshaAlways => true,
        _ => false,
    }
}

fn adj_near_lat(
    hours: &HashMap<Prayer, RefCell<Result<PrayerHour, ()>>>,
    params: &Params,
    top_astro_day: &TopAstroDay,
    weather: Weather,
) {
    use ExtremeLatitudeMethod::*;
    use Prayer::*;

    let mut coords = top_astro_day.coords();
    coords.latitude = params.near_lat;
    let adj_top_astro_day = top_astro_day.new_coords(coords);
    let adj_hours = get_hours(params, &adj_top_astro_day, weather);

    if let Ok(adj_hour) = adj_hours[&Fajr] {
        let mut hours_res = hours[&Fajr].borrow_mut();
        if params.ext_lat_method != NearestLatitudeFajrIshaInvalid || hours_res.is_err() {
            *hours_res = Ok(PrayerHour::new_extreme(adj_hour));
        }
    }
    if let Ok(adj_hour) = adj_hours[&Isha] {
        let mut hours_res = hours[&Isha].borrow_mut();
        if params.ext_lat_method != NearestLatitudeFajrIshaInvalid || hours_res.is_err() {
            *hours_res = Ok(PrayerHour::new_extreme(adj_hour));
        }
    }

    if params.ext_lat_method == NearestLatitudeAllPrayersAlways {
        *hours[&Shurooq].borrow_mut() =
            adj_hours[&Shurooq].map(|hour| PrayerHour::new_extreme(hour));
        hours[&Dhuhr].borrow_mut().as_mut().unwrap().extreme = true;
        *hours[&Asr].borrow_mut() = adj_hours[&Asr].map(|hour| PrayerHour::new_extreme(hour));
        *hours[&Maghrib].borrow_mut() =
            adj_hours[&Maghrib].map(|hour| PrayerHour::new_extreme(hour));
    }
}

fn adj_near_good(
    hours: &HashMap<Prayer, RefCell<Result<PrayerHour, ()>>>,
    params: &Params,
    top_astro_day: &TopAstroDay,
    weather: Weather,
) {
    use ExtremeLatitudeMethod::*;
    use Prayer::*;

    let mut adj_hours = HashMap::new();
    let julian_day = top_astro_day.julian_day();
    for i in 0..=julian_day.date.ordinal() {
        if let Some(x) = test_fajr_isha(
            params,
            top_astro_day.coords(),
            weather,
            julian_day.sub(i as u64),
        ) {
            adj_hours = x;
            break;
        }

        if let Some(x) = test_fajr_isha(
            params,
            top_astro_day.coords(),
            weather,
            julian_day.add(i as u64),
        ) {
            adj_hours = x;
            break;
        }
    }

    if !adj_hours.is_empty() {
        let dhuhr_hour = adj_hours[&Dhuhr].unwrap();
        if params.ext_lat_method == NearestGoodDayAllPrayersAlways {
            *hours[&Fajr].borrow_mut() = adj_hours[&Fajr].map(|hour| PrayerHour::new_extreme(hour));
            *hours[&Shurooq].borrow_mut() =
                adj_hours[&Shurooq].map(|hour| PrayerHour::new_extreme(hour));
            *hours[&Dhuhr].borrow_mut() = Ok(PrayerHour::new_extreme(dhuhr_hour));
            *hours[&Asr].borrow_mut() = adj_hours[&Asr].map(|hour| PrayerHour::new_extreme(hour));
            *hours[&Maghrib].borrow_mut() =
                adj_hours[&Maghrib].map(|hour| PrayerHour::new_extreme(hour));
            *hours[&Isha].borrow_mut() = adj_hours[&Isha].map(|hour| PrayerHour::new_extreme(hour));
        } else {
            if hours[&Fajr].borrow().is_err() {
                *hours[&Fajr].borrow_mut() =
                    adj_hours[&Fajr].map(|hour| PrayerHour::new_extreme(hour));
            }
            if hours[&Isha].borrow().is_err() {
                *hours[&Isha].borrow_mut() =
                    adj_hours[&Isha].map(|hour| PrayerHour::new_extreme(hour));
            }
        }
    }
}

fn test_fajr_isha(
    params: &Params,
    coords: Coordinates,
    weather: Weather,
    julian_day: JulianDay,
) -> Option<HashMap<Prayer, Result<f64, ()>>> {
    use Prayer::*;

    let top_astro_day = TopAstroDay::from_jd(julian_day, coords);
    let hours = get_hours(params, &top_astro_day, weather);
    if hours[&Fajr].is_ok() && hours[&Isha].is_ok() {
        Some(hours)
    } else {
        None
    }
}

fn adj_sev_half(hours: &HashMap<Prayer, RefCell<Result<PrayerHour, ()>>>, params: &Params) {
    use ExtremeLatitudeMethod::*;
    use Prayer::*;

    if hours[&Shurooq].borrow().is_ok() && hours[&Maghrib].borrow().is_ok() {
        let shur_hour = hours[&Shurooq].borrow().as_ref().unwrap().value;
        let magh_hour = hours[&Maghrib].borrow().as_ref().unwrap().value;
        let portion = match params.ext_lat_method {
            SeventhOfNightFajrIshaAlways | SeventhOfNightFajrIshaInvalid => {
                (24. - (magh_hour - shur_hour)) / 7.
            }
            SeventhOfDayFajrIshaAlways | SeventhOfDayFajrIshaInvalid => {
                (magh_hour - shur_hour) / 7.
            }
            _ => (24. - (magh_hour - shur_hour)) * 0.5,
        };

        match params.ext_lat_method {
            SeventhOfNightFajrIshaAlways
            | SeventhOfDayFajrIshaAlways
            | HalfOfNightFajrIshaAlways => {
                if params.ext_lat_method == HalfOfNightFajrIshaAlways {
                    *hours[&Fajr].borrow_mut() = Ok(PrayerHour::new_extreme(
                        portion - params.intervals[&Fajr] / 60.,
                    ));
                    *hours[&Isha].borrow_mut() = Ok(PrayerHour::new_extreme(
                        portion + params.intervals[&Isha] / 60.,
                    ));
                } else {
                    *hours[&Fajr].borrow_mut() = Ok(PrayerHour::new_extreme(shur_hour - portion));
                    *hours[&Isha].borrow_mut() = Ok(PrayerHour::new_extreme(magh_hour + portion));
                }
            }
            _ => {
                if hours[&Fajr].borrow().is_err() {
                    if params.ext_lat_method == HalfOfNightFajrIshaInvalid {
                        *hours[&Fajr].borrow_mut() = Ok(PrayerHour::new_extreme(
                            portion - params.intervals[&Fajr] / 60.,
                        ));
                    } else {
                        *hours[&Fajr].borrow_mut() =
                            Ok(PrayerHour::new_extreme(shur_hour - portion));
                    }
                }
                if hours[&Isha].borrow().is_err() {
                    if params.ext_lat_method == HalfOfNightFajrIshaInvalid {
                        *hours[&Isha].borrow_mut() = Ok(PrayerHour::new_extreme(
                            portion + params.intervals[&Isha] / 60.,
                        ));
                    } else {
                        *hours[&Isha].borrow_mut() =
                            Ok(PrayerHour::new_extreme(magh_hour + portion));
                    }
                }
            }
        }
    }
}

fn adj_min_always(hours: &HashMap<Prayer, RefCell<Result<PrayerHour, ()>>>) {
    use Prayer::*;

    *hours[&Fajr].borrow_mut() = hours[&Shurooq].borrow().map(|mut x| {
        x.extreme = true;
        x
    });
    *hours[&Isha].borrow_mut() = hours[&Maghrib].borrow().map(|mut x| {
        x.extreme = true;
        x
    });
}

fn adj_min_inv(hours: &HashMap<Prayer, RefCell<Result<PrayerHour, ()>>>, params: &Params) {
    use Prayer::*;

    if hours[&Fajr].borrow().is_err() {
        *hours[&Fajr].borrow_mut() = hours[&Shurooq].borrow().map(|mut x| {
            x.value -= params.intervals[&Fajr] / 60.;
            x.extreme = true;
            x
        });
    }
    if hours[&Isha].borrow().is_err() {
        *hours[&Isha].borrow_mut() = hours[&Maghrib].borrow().map(|mut x| {
            x.value += params.intervals[&Isha] / 60.;
            x.extreme = true;
            x
        });
    }
}
