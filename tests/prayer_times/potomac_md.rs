use chrono::{NaiveDate, NaiveTime};
use islamic_prayer_times::{
    geo::coordinates::{Coordinates, Elevation, Gmt, Latitude, Longitude},
    prayer_times::{
        params::{AsrShadowRatio, ExtremeLatitudeMethod, Method, Params},
        prayer_times_dt, prayer_times_dt_rng, DateRange, Location, Prayer,
    },
};

#[test]
fn test_potomac_md_default_params() {
    // Arrange
    let params = Params::new(Method::Isna);
    let latitude = Latitude::new(39.0181651).unwrap();
    let longitude = Longitude::new(-77.2085914).unwrap();
    let elevation = Elevation::default();
    let coords = Coordinates::new(latitude, longitude, elevation);
    let gmt = Gmt::new(-5.).unwrap();
    let location = Location { coords, gmt };
    let start_date = NaiveDate::from_ymd_opt(2023, 2, 6).unwrap();
    let middle_date = NaiveDate::from_ymd_opt(2023, 2, 7).unwrap();
    let end_date = NaiveDate::from_ymd_opt(2023, 2, 8).unwrap();
    let date_range = DateRange::new(start_date, end_date);

    // Act
    let pts_by_day = prayer_times_dt_rng(&params, location, date_range);

    // Assert
    assert_eq!(3, pts_by_day.len());
    assert_eq!(true, pts_by_day.contains_key(&start_date));
    assert_eq!(true, pts_by_day.contains_key(&middle_date));
    assert_eq!(true, pts_by_day.contains_key(&end_date));

    let pts = pts_by_day.get(&start_date).unwrap();
    let imsaak = pts.get(&Prayer::Imsaak).unwrap().unwrap();
    assert_eq!(NaiveTime::from_hms_opt(5, 48, 0).unwrap(), imsaak.time);
    assert_eq!(false, imsaak.extreme);
    let fajr = pts.get(&Prayer::Fajr).unwrap().unwrap();
    assert_eq!(NaiveTime::from_hms_opt(5, 56, 0).unwrap(), fajr.time);
    assert_eq!(false, fajr.extreme);
    let shurooq = pts.get(&Prayer::Shurooq).unwrap().unwrap();
    assert_eq!(NaiveTime::from_hms_opt(7, 10, 0).unwrap(), shurooq.time);
    assert_eq!(false, shurooq.extreme);
    let dhuhr = pts.get(&Prayer::Dhuhr).unwrap().unwrap();
    assert_eq!(NaiveTime::from_hms_opt(12, 23, 0).unwrap(), dhuhr.time);
    assert_eq!(false, dhuhr.extreme);
    let asr = pts.get(&Prayer::Asr).unwrap().unwrap();
    assert_eq!(NaiveTime::from_hms_opt(15, 12, 0).unwrap(), asr.time);
    assert_eq!(false, asr.extreme);
    let maghrib = pts.get(&Prayer::Maghrib).unwrap().unwrap();
    assert_eq!(NaiveTime::from_hms_opt(17, 36, 0).unwrap(), maghrib.time);
    assert_eq!(false, maghrib.extreme);
    let isha = pts.get(&Prayer::Isha).unwrap().unwrap();
    assert_eq!(NaiveTime::from_hms_opt(18, 50, 0).unwrap(), isha.time);
    assert_eq!(false, isha.extreme);

    let pts = pts_by_day.get(&middle_date).unwrap();
    let imsaak = pts.get(&Prayer::Imsaak).unwrap().unwrap();
    assert_eq!(NaiveTime::from_hms_opt(5, 47, 0).unwrap(), imsaak.time);
    assert_eq!(false, imsaak.extreme);
    let fajr = pts.get(&Prayer::Fajr).unwrap().unwrap();
    assert_eq!(NaiveTime::from_hms_opt(5, 55, 0).unwrap(), fajr.time);
    assert_eq!(false, fajr.extreme);
    let shurooq = pts.get(&Prayer::Shurooq).unwrap().unwrap();
    assert_eq!(NaiveTime::from_hms_opt(7, 9, 0).unwrap(), shurooq.time);
    assert_eq!(false, shurooq.extreme);
    let dhuhr = pts.get(&Prayer::Dhuhr).unwrap().unwrap();
    assert_eq!(NaiveTime::from_hms_opt(12, 23, 0).unwrap(), dhuhr.time);
    assert_eq!(false, dhuhr.extreme);
    let asr = pts.get(&Prayer::Asr).unwrap().unwrap();
    assert_eq!(NaiveTime::from_hms_opt(15, 13, 0).unwrap(), asr.time);
    assert_eq!(false, asr.extreme);
    let maghrib = pts.get(&Prayer::Maghrib).unwrap().unwrap();
    assert_eq!(NaiveTime::from_hms_opt(17, 37, 0).unwrap(), maghrib.time);
    assert_eq!(false, maghrib.extreme);
    let isha = pts.get(&Prayer::Isha).unwrap().unwrap();
    assert_eq!(NaiveTime::from_hms_opt(18, 51, 0).unwrap(), isha.time);
    assert_eq!(false, isha.extreme);

    let pts = pts_by_day.get(&end_date).unwrap();
    let imsaak = pts.get(&Prayer::Imsaak).unwrap().unwrap();
    assert_eq!(NaiveTime::from_hms_opt(5, 46, 0).unwrap(), imsaak.time);
    assert_eq!(false, imsaak.extreme);
    let fajr = pts.get(&Prayer::Fajr).unwrap().unwrap();
    assert_eq!(NaiveTime::from_hms_opt(5, 54, 0).unwrap(), fajr.time);
    assert_eq!(false, fajr.extreme);
    let shurooq = pts.get(&Prayer::Shurooq).unwrap().unwrap();
    assert_eq!(NaiveTime::from_hms_opt(7, 8, 0).unwrap(), shurooq.time);
    assert_eq!(false, shurooq.extreme);
    let dhuhr = pts.get(&Prayer::Dhuhr).unwrap().unwrap();
    assert_eq!(NaiveTime::from_hms_opt(12, 23, 0).unwrap(), dhuhr.time);
    assert_eq!(false, dhuhr.extreme);
    let asr = pts.get(&Prayer::Asr).unwrap().unwrap();
    assert_eq!(NaiveTime::from_hms_opt(15, 14, 0).unwrap(), asr.time);
    assert_eq!(false, asr.extreme);
    let maghrib = pts.get(&Prayer::Maghrib).unwrap().unwrap();
    assert_eq!(NaiveTime::from_hms_opt(17, 38, 0).unwrap(), maghrib.time);
    assert_eq!(false, maghrib.extreme);
    let isha = pts.get(&Prayer::Isha).unwrap().unwrap();
    assert_eq!(NaiveTime::from_hms_opt(18, 52, 0).unwrap(), isha.time);
    assert_eq!(false, isha.extreme);
}

#[test]
fn test_potomac_md_default_params_hanafi() {
    // Arrange
    let mut params = Params::new(Method::Isna);
    params.asr_shadow_ratio = AsrShadowRatio::Hanafi;
    let latitude = Latitude::new(39.0181651).unwrap();
    let longitude = Longitude::new(-77.2085914).unwrap();
    let elevation = Elevation::default();
    let coords = Coordinates::new(latitude, longitude, elevation);
    let gmt = Gmt::new(-5.).unwrap();
    let location = Location { coords, gmt };
    let date = NaiveDate::from_ymd_opt(2023, 2, 6).unwrap();

    // Act
    let pts = prayer_times_dt(&params, location, date, None);

    // Assert
    let imsaak = pts.get(&Prayer::Imsaak).unwrap().unwrap();
    assert_eq!(NaiveTime::from_hms_opt(5, 48, 0).unwrap(), imsaak.time);
    assert_eq!(false, imsaak.extreme);
    let fajr = pts.get(&Prayer::Fajr).unwrap().unwrap();
    assert_eq!(NaiveTime::from_hms_opt(5, 56, 0).unwrap(), fajr.time);
    assert_eq!(false, fajr.extreme);
    let shurooq = pts.get(&Prayer::Shurooq).unwrap().unwrap();
    assert_eq!(NaiveTime::from_hms_opt(7, 10, 0).unwrap(), shurooq.time);
    assert_eq!(false, shurooq.extreme);
    let dhuhr = pts.get(&Prayer::Dhuhr).unwrap().unwrap();
    assert_eq!(NaiveTime::from_hms_opt(12, 23, 0).unwrap(), dhuhr.time);
    assert_eq!(false, dhuhr.extreme);
    let asr = pts.get(&Prayer::Asr).unwrap().unwrap();
    assert_eq!(NaiveTime::from_hms_opt(15, 54, 0).unwrap(), asr.time);
    assert_eq!(false, asr.extreme);
    let maghrib = pts.get(&Prayer::Maghrib).unwrap().unwrap();
    assert_eq!(NaiveTime::from_hms_opt(17, 36, 0).unwrap(), maghrib.time);
    assert_eq!(false, maghrib.extreme);
    let isha = pts.get(&Prayer::Isha).unwrap().unwrap();
    assert_eq!(NaiveTime::from_hms_opt(18, 50, 0).unwrap(), isha.time);
    assert_eq!(false, isha.extreme);
}

#[test]
fn test_potomac_md_default_params_ang_bas() {
    // Arrange
    let mut params = Params::new(Method::Isna);
    params.extreme_latitude_method = ExtremeLatitudeMethod::AngleBased;
    let latitude = Latitude::new(39.0181651).unwrap();
    let longitude = Longitude::new(-77.2085914).unwrap();
    let elevation = Elevation::default();
    let coords = Coordinates::new(latitude, longitude, elevation);
    let gmt = Gmt::new(-5.).unwrap();
    let location = Location { coords, gmt };
    let date = NaiveDate::from_ymd_opt(2023, 2, 6).unwrap();

    // Act
    let pts = prayer_times_dt(&params, location, date, None);

    // Assert
    let imsaak = pts.get(&Prayer::Imsaak).unwrap().unwrap();
    assert_eq!(NaiveTime::from_hms_opt(5, 48, 0).unwrap(), imsaak.time);
    assert_eq!(false, imsaak.extreme);
    let fajr = pts.get(&Prayer::Fajr).unwrap().unwrap();
    assert_eq!(NaiveTime::from_hms_opt(5, 56, 0).unwrap(), fajr.time);
    assert_eq!(false, fajr.extreme);
    let shurooq = pts.get(&Prayer::Shurooq).unwrap().unwrap();
    assert_eq!(NaiveTime::from_hms_opt(7, 10, 0).unwrap(), shurooq.time);
    assert_eq!(false, shurooq.extreme);
    let dhuhr = pts.get(&Prayer::Dhuhr).unwrap().unwrap();
    assert_eq!(NaiveTime::from_hms_opt(12, 23, 0).unwrap(), dhuhr.time);
    assert_eq!(false, dhuhr.extreme);
    let asr = pts.get(&Prayer::Asr).unwrap().unwrap();
    assert_eq!(NaiveTime::from_hms_opt(15, 12, 0).unwrap(), asr.time);
    assert_eq!(false, asr.extreme);
    let maghrib = pts.get(&Prayer::Maghrib).unwrap().unwrap();
    assert_eq!(NaiveTime::from_hms_opt(17, 36, 0).unwrap(), maghrib.time);
    assert_eq!(false, maghrib.extreme);
    let isha = pts.get(&Prayer::Isha).unwrap().unwrap();
    assert_eq!(NaiveTime::from_hms_opt(18, 50, 0).unwrap(), isha.time);
    assert_eq!(false, isha.extreme);
}

#[test]
fn test_potomac_md_default_params_min_off() {
    use Prayer::*;

    // Arrange
    let mut params = Params::new(Method::Isna);
    *params.minutes.get_mut(&Imsaak).unwrap() = 1.;
    *params.minutes.get_mut(&Fajr).unwrap() = 1.;
    *params.minutes.get_mut(&Shurooq).unwrap() = 1.;
    *params.minutes.get_mut(&Dhuhr).unwrap() = 1.;
    *params.minutes.get_mut(&Asr).unwrap() = 1.;
    *params.minutes.get_mut(&Maghrib).unwrap() = 1.;
    *params.minutes.get_mut(&Isha).unwrap() = 1.;
    let latitude = Latitude::new(39.0181651).unwrap();
    let longitude = Longitude::new(-77.2085914).unwrap();
    let elevation = Elevation::default();
    let coords = Coordinates::new(latitude, longitude, elevation);
    let gmt = Gmt::new(-5.).unwrap();
    let location = Location { coords, gmt };
    let date = NaiveDate::from_ymd_opt(2023, 2, 6).unwrap();

    // Act
    let pts = prayer_times_dt(&params, location, date, None);

    // Assert
    let imsaak = pts.get(&Prayer::Imsaak).unwrap().unwrap();
    assert_eq!(NaiveTime::from_hms_opt(5, 49, 0).unwrap(), imsaak.time);
    assert_eq!(false, imsaak.extreme);
    let fajr = pts.get(&Prayer::Fajr).unwrap().unwrap();
    assert_eq!(NaiveTime::from_hms_opt(5, 57, 0).unwrap(), fajr.time);
    assert_eq!(false, fajr.extreme);
    let shurooq = pts.get(&Prayer::Shurooq).unwrap().unwrap();
    assert_eq!(NaiveTime::from_hms_opt(7, 11, 0).unwrap(), shurooq.time);
    assert_eq!(false, shurooq.extreme);
    let dhuhr = pts.get(&Prayer::Dhuhr).unwrap().unwrap();
    assert_eq!(NaiveTime::from_hms_opt(12, 24, 0).unwrap(), dhuhr.time);
    assert_eq!(false, dhuhr.extreme);
    let asr = pts.get(&Prayer::Asr).unwrap().unwrap();
    assert_eq!(NaiveTime::from_hms_opt(15, 13, 0).unwrap(), asr.time);
    assert_eq!(false, asr.extreme);
    let maghrib = pts.get(&Prayer::Maghrib).unwrap().unwrap();
    assert_eq!(NaiveTime::from_hms_opt(17, 37, 0).unwrap(), maghrib.time);
    assert_eq!(false, maghrib.extreme);
    let isha = pts.get(&Prayer::Isha).unwrap().unwrap();
    assert_eq!(NaiveTime::from_hms_opt(18, 51, 0).unwrap(), isha.time);
    assert_eq!(false, isha.extreme);
}
