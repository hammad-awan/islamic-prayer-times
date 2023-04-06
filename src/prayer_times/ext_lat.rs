use std::{cell::RefCell, collections::HashMap};

use chrono::Datelike;

use crate::{
    geo::{astro::TopAstroDay, coordinates::Coordinates, julian_day::JulianDay},
    prayer_times::{
        hours::{get_hours, HRS_PER_DAY, MIN_SEC_PER_HR_MIN},
        params::ExtremeLatitudeMethod,
    },
};

use super::{params::Params, Prayer, Weather};

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
    params: &Params,
    hours: HashMap<Prayer, Result<f64, ()>>,
    top_astro_day: &TopAstroDay,
    weather: Weather,
) -> HashMap<Prayer, Result<PrayerHour, ()>> {
    use ExtremeLatitudeMethod::*;

    let hours: HashMap<_, _> = HashMap::from_iter(
        hours
            .iter()
            .map(|x| (*x.0, RefCell::new(x.1.map(PrayerHour::new)))),
    );

    if can_adj(&hours, params.extreme_latitude_method) {
        match params.extreme_latitude_method {
            AngleBased => angle_based(params, &hours),
            NearestLatitudeAllPrayersAlways
            | NearestLatitudeFajrIshaAlways
            | NearestLatitudeFajrIshaInvalid => {
                adj_near_lat(params, &hours, top_astro_day, weather)
            }
            NearestGoodDayAllPrayersAlways | NearestGoodDayFajrIshaInvalid => {
                adj_near_good(params, &hours, top_astro_day, weather)
            }
            SeventhOfNightFajrIshaAlways
            | SeventhOfNightFajrIshaInvalid
            | SeventhOfDayFajrIshaAlways
            | SeventhOfDayFajrIshaInvalid
            | HalfOfNightFajrIshaAlways
            | HalfOfNightFajrIshaInvalid => adj_sev_half(params, &hours),
            MinutesFromMaghribFajrIshaAlways => adj_min_always(&hours),
            MinutesFromMaghribFajrIshaInvalid => adj_min_inv(params, &hours),
            _ => {}
        }
    }

    adj_for_int(params, &hours);

    HashMap::from_iter(hours.iter().map(|x| (*x.0, x.1.borrow().map(|y| y))))
}

fn can_adj(
    hours: &HashMap<Prayer, RefCell<Result<PrayerHour, ()>>>,
    ext_lat_meth: ExtremeLatitudeMethod,
) -> bool {
    ext_lat_meth != ExtremeLatitudeMethod::None
        && (has_inv_hours(hours) || is_ext_lat_always(ext_lat_meth))
}

fn has_inv_hours(hours: &HashMap<Prayer, RefCell<Result<PrayerHour, ()>>>) -> bool {
    hours.iter().any(|x| x.1.borrow().is_err())
}

fn is_ext_lat_always(ext_lat_meth: ExtremeLatitudeMethod) -> bool {
    use ExtremeLatitudeMethod::*;

    matches!(
        ext_lat_meth,
        NearestLatitudeAllPrayersAlways
            | NearestLatitudeFajrIshaAlways
            | NearestGoodDayAllPrayersAlways
            | SeventhOfNightFajrIshaAlways
            | SeventhOfDayFajrIshaAlways
            | HalfOfNightFajrIshaAlways
            | MinutesFromMaghribFajrIshaAlways
    )
}

fn angle_based(params: &Params, hours: &HashMap<Prayer, RefCell<Result<PrayerHour, ()>>>) {
    use Prayer::*;

    if hours[&Shurooq].borrow().is_ok() && hours[&Maghrib].borrow().is_ok() {
        let shur_hour = hours[&Shurooq].borrow().unwrap().value;
        let magh_hour = hours[&Maghrib].borrow().unwrap().value;
        let portion = HRS_PER_DAY - magh_hour + shur_hour;
        let ratio = 1. / MIN_SEC_PER_HR_MIN;
        let fajr_diff = ratio * params.angles[&Fajr] * portion;
        let isha_diff = ratio * params.angles[&Isha] * portion;
        *hours[&Fajr].borrow_mut() = Ok(PrayerHour::new_extreme(shur_hour - fajr_diff));
        *hours[&Isha].borrow_mut() = Ok(PrayerHour::new_extreme(magh_hour + isha_diff));
    }
}

fn adj_near_lat(
    params: &Params,
    hours: &HashMap<Prayer, RefCell<Result<PrayerHour, ()>>>,
    top_astro_day: &TopAstroDay,
    weather: Weather,
) {
    use ExtremeLatitudeMethod::*;
    use Prayer::*;

    let mut coords = top_astro_day.coords();
    coords.latitude = params.nearest_latitude;
    let adj_top_astro_day = top_astro_day.new_coords(coords);
    let adj_hours = get_hours(params, &adj_top_astro_day, weather);

    if let Ok(adj_hour) = adj_hours[&Fajr] {
        let mut hours_res = hours[&Fajr].borrow_mut();
        if params.extreme_latitude_method != NearestLatitudeFajrIshaInvalid || hours_res.is_err() {
            *hours_res = Ok(PrayerHour::new_extreme(adj_hour));
        }
    }

    if let Ok(adj_hour) = adj_hours[&Isha] {
        let mut hours_res = hours[&Isha].borrow_mut();
        if params.extreme_latitude_method != NearestLatitudeFajrIshaInvalid || hours_res.is_err() {
            *hours_res = Ok(PrayerHour::new_extreme(adj_hour));
        }
    }

    if params.extreme_latitude_method == NearestLatitudeAllPrayersAlways {
        *hours[&Shurooq].borrow_mut() = adj_hours[&Shurooq].map(PrayerHour::new_extreme);
        hours[&Dhuhr].borrow_mut().as_mut().unwrap().extreme = true;
        *hours[&Asr].borrow_mut() = adj_hours[&Asr].map(PrayerHour::new_extreme);
        *hours[&Maghrib].borrow_mut() = adj_hours[&Maghrib].map(PrayerHour::new_extreme);
    }
}

fn adj_near_good(
    params: &Params,
    hours: &HashMap<Prayer, RefCell<Result<PrayerHour, ()>>>,
    top_astro_day: &TopAstroDay,
    weather: Weather,
) {
    use ExtremeLatitudeMethod::*;
    use Prayer::*;

    let mut adj_hours = HashMap::new();
    let julian_day = top_astro_day.julian_day();
    for i in 0..=julian_day.date.ordinal() {
        if let Some(hour) = test_fajr_isha(
            params,
            top_astro_day.coords(),
            weather,
            julian_day.sub(i as u64),
        ) {
            adj_hours = hour;
            break;
        }

        if let Some(hour) = test_fajr_isha(
            params,
            top_astro_day.coords(),
            weather,
            julian_day.add(i as u64),
        ) {
            adj_hours = hour;
            break;
        }
    }

    if !adj_hours.is_empty() {
        if params.extreme_latitude_method == NearestGoodDayAllPrayersAlways {
            *hours[&Fajr].borrow_mut() = adj_hours[&Fajr].map(PrayerHour::new_extreme);
            *hours[&Shurooq].borrow_mut() = adj_hours[&Shurooq].map(PrayerHour::new_extreme);
            *hours[&Dhuhr].borrow_mut() = adj_hours[&Dhuhr].map(PrayerHour::new_extreme);
            *hours[&Asr].borrow_mut() = adj_hours[&Asr].map(PrayerHour::new_extreme);
            *hours[&Maghrib].borrow_mut() = adj_hours[&Maghrib].map(PrayerHour::new_extreme);
            *hours[&Isha].borrow_mut() = adj_hours[&Isha].map(PrayerHour::new_extreme);
        } else {
            // NearestGoodDayFajrIshaInvalid
            if hours[&Fajr].borrow().is_err() {
                *hours[&Fajr].borrow_mut() = adj_hours[&Fajr].map(PrayerHour::new_extreme);
            }

            if hours[&Isha].borrow().is_err() {
                *hours[&Isha].borrow_mut() = adj_hours[&Isha].map(PrayerHour::new_extreme);
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

fn adj_sev_half(params: &Params, hours: &HashMap<Prayer, RefCell<Result<PrayerHour, ()>>>) {
    use ExtremeLatitudeMethod::*;
    use Prayer::*;

    if hours[&Shurooq].borrow().is_ok() && hours[&Maghrib].borrow().is_ok() {
        let shur_hour = hours[&Shurooq].borrow().as_ref().unwrap().value;
        let magh_hour = hours[&Maghrib].borrow().as_ref().unwrap().value;
        let portion = match params.extreme_latitude_method {
            SeventhOfNightFajrIshaAlways | SeventhOfNightFajrIshaInvalid => {
                (HRS_PER_DAY - (magh_hour - shur_hour)) / 7.
            }
            SeventhOfDayFajrIshaAlways | SeventhOfDayFajrIshaInvalid => {
                (magh_hour - shur_hour) / 7.
            } // HalfOfNightFajrIshaAlways | HalfOfNightFajrIshaInvalid
            _ => (HRS_PER_DAY - magh_hour - shur_hour) * 0.5,
        };

        match params.extreme_latitude_method {
            SeventhOfNightFajrIshaAlways
            | SeventhOfDayFajrIshaAlways
            | HalfOfNightFajrIshaAlways => {
                if params.extreme_latitude_method == HalfOfNightFajrIshaAlways {
                    *hours[&Fajr].borrow_mut() = Ok(PrayerHour::new_extreme(
                        portion - params.intervals[&Fajr] / MIN_SEC_PER_HR_MIN,
                    ));
                    *hours[&Isha].borrow_mut() = Ok(PrayerHour::new_extreme(
                        portion + params.intervals[&Isha] / MIN_SEC_PER_HR_MIN,
                    ));
                } else {
                    // SeventhOfNightFajrIshaAlways | SeventhOfDayFajrIshaAlways
                    *hours[&Fajr].borrow_mut() = Ok(PrayerHour::new_extreme(shur_hour - portion));
                    *hours[&Isha].borrow_mut() = Ok(PrayerHour::new_extreme(magh_hour + portion));
                }
            }
            _ => {
                // SeventhOfNightFajrIshaInvalid | SeventhOfDayFajrIshaInvalid | HalfOfNightFajrIshaInvalid
                if hours[&Fajr].borrow().is_err() {
                    if params.extreme_latitude_method == HalfOfNightFajrIshaInvalid {
                        *hours[&Fajr].borrow_mut() = Ok(PrayerHour::new_extreme(
                            portion - params.intervals[&Fajr] / MIN_SEC_PER_HR_MIN,
                        ));
                    } else {
                        // SeventhOfNightFajrIshaInvalid | SeventhOfDayFajrIshaInvalid
                        *hours[&Fajr].borrow_mut() =
                            Ok(PrayerHour::new_extreme(shur_hour - portion));
                    }
                }

                if hours[&Isha].borrow().is_err() {
                    if params.extreme_latitude_method == HalfOfNightFajrIshaInvalid {
                        *hours[&Isha].borrow_mut() = Ok(PrayerHour::new_extreme(
                            portion + params.intervals[&Isha] / MIN_SEC_PER_HR_MIN,
                        ));
                    } else {
                        // SeventhOfNightFajrIshaInvalid | SeventhOfDayFajrIshaInvalid
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

    // Do nothing because this is implemented through fajr and isha intervals.
    *hours[&Fajr].borrow_mut() = hours[&Shurooq].borrow().map(|mut x| {
        x.extreme = true;
        x
    });

    *hours[&Isha].borrow_mut() = hours[&Maghrib].borrow().map(|mut x| {
        x.extreme = true;
        x
    });
}

fn adj_min_inv(params: &Params, hours: &HashMap<Prayer, RefCell<Result<PrayerHour, ()>>>) {
    use Prayer::*;

    if hours[&Fajr].borrow().is_err() {
        *hours[&Fajr].borrow_mut() = hours[&Shurooq].borrow().map(|mut x| {
            x.value -= params.intervals[&Fajr] / MIN_SEC_PER_HR_MIN;
            x.extreme = true;
            x
        });
    }

    if hours[&Isha].borrow().is_err() {
        *hours[&Isha].borrow_mut() = hours[&Maghrib].borrow().map(|mut x| {
            x.value += params.intervals[&Isha] / MIN_SEC_PER_HR_MIN;
            x.extreme = true;
            x
        });
    }
}

fn adj_for_int(params: &Params, hours: &HashMap<Prayer, RefCell<Result<PrayerHour, ()>>>) {
    use ExtremeLatitudeMethod::*;
    use Prayer::*;

    if params.extreme_latitude_method != MinutesFromMaghribFajrIshaInvalid
        && params.extreme_latitude_method != HalfOfNightFajrIshaInvalid
        && params.extreme_latitude_method != HalfOfNightFajrIshaAlways
    {
        if params.intervals[&Fajr] != 0. {
            let extreme = hours[&Fajr].borrow().unwrap().extreme;
            *hours[&Fajr].borrow_mut() = hours[&Shurooq].borrow().map(|mut x| {
                x.value -= params.intervals[&Fajr] / MIN_SEC_PER_HR_MIN;
                x.extreme = extreme;
                x
            });
        }

        if params.intervals[&Isha] != 0. {
            let extreme = hours[&Isha].borrow().unwrap().extreme;
            *hours[&Isha].borrow_mut() = hours[&Maghrib].borrow().map(|mut x| {
                x.value += params.intervals[&Isha] / MIN_SEC_PER_HR_MIN;
                x.extreme = extreme;
                x
            });
        }
    }
}
