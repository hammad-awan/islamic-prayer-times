use chrono::{NaiveDate, NaiveTime};
use islamic_prayer_times::{
    geo::coordinates::{Coordinates, Elevation, Gmt, Latitude, Longitude},
    prayer_times::{
        params::{ExtremeLatitudeMethod, Method, Params},
        times_dt, DateRange, Prayer, {times_dt_rng, Location},
    },
};

#[test]
fn test_potomac_md() {
    // Arrange
    let params = Params::new(Method::Isna);
    let latitude = Latitude::new(39.0181651).unwrap();
    let longitude = Longitude::new(-77.2085914).unwrap();
    let elevation = Elevation::new(0.).unwrap();
    let coords = Coordinates::new(latitude, longitude, elevation);
    let gmt = Gmt::new(-5.).unwrap();
    let location = Location { coords, gmt };
    let start_date = NaiveDate::from_ymd_opt(2023, 2, 6).unwrap();
    let middle_date = NaiveDate::from_ymd_opt(2023, 2, 7).unwrap();
    let end_date = NaiveDate::from_ymd_opt(2023, 2, 8).unwrap();
    let date_range = DateRange::new(start_date, end_date).unwrap();

    // Act
    let pts_by_day = times_dt_rng(&params, location, date_range);

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
}

#[test]
fn test_juneau_ak_default_params() {
    // Arrange
    let params = Params::new(Method::Isna);
    let latitude = Latitude::new(58.3019444).unwrap();
    let longitude = Longitude::new(-134.4197222).unwrap();
    let elevation = Elevation::new(87.).unwrap();
    let coords = Coordinates::new(latitude, longitude, elevation);
    let gmt = Gmt::new(-9.).unwrap();
    let location = Location { coords, gmt };
    let date = NaiveDate::from_ymd_opt(2023, 2, 6).unwrap();

    // Act
    let pts = times_dt(&params, location, date, None);

    // Assert
    let imsaak = pts.get(&Prayer::Imsaak).unwrap().unwrap();
    assert_eq!(NaiveTime::from_hms_opt(5, 47, 0).unwrap(), imsaak.time);
    assert_eq!(false, imsaak.extreme);
    let fajr = pts.get(&Prayer::Fajr).unwrap().unwrap();
    assert_eq!(NaiveTime::from_hms_opt(5, 58, 0).unwrap(), fajr.time);
    assert_eq!(false, fajr.extreme);
    let shurooq = pts.get(&Prayer::Shurooq).unwrap().unwrap();
    assert_eq!(NaiveTime::from_hms_opt(7, 51, 0).unwrap(), shurooq.time);
    assert_eq!(false, shurooq.extreme);
    let dhuhr = pts.get(&Prayer::Dhuhr).unwrap().unwrap();
    assert_eq!(NaiveTime::from_hms_opt(12, 12, 0).unwrap(), dhuhr.time);
    assert_eq!(false, dhuhr.extreme);
    let asr = pts.get(&Prayer::Asr).unwrap().unwrap();
    assert_eq!(NaiveTime::from_hms_opt(14, 3, 0).unwrap(), asr.time);
    assert_eq!(false, asr.extreme);
    let maghrib = pts.get(&Prayer::Maghrib).unwrap().unwrap();
    assert_eq!(NaiveTime::from_hms_opt(16, 33, 0).unwrap(), maghrib.time);
    assert_eq!(false, maghrib.extreme);
    let isha = pts.get(&Prayer::Isha).unwrap().unwrap();
    assert_eq!(NaiveTime::from_hms_opt(18, 25, 0).unwrap(), isha.time);
    assert_eq!(false, isha.extreme);
}

#[test]
fn test_juneau_ak_default_params_near_lat_all_prayers_always() {
    // Arrange
    let mut params = Params::new(Method::Isna);
    params.ext_lat_method = ExtremeLatitudeMethod::NearestLatitudeAllPrayersAlways;
    let latitude = Latitude::new(58.3019444).unwrap();
    let longitude = Longitude::new(-134.4197222).unwrap();
    let elevation = Elevation::new(87.).unwrap();
    let coords = Coordinates::new(latitude, longitude, elevation);
    let gmt = Gmt::new(-9.).unwrap();
    let location = Location { coords, gmt };
    let date = NaiveDate::from_ymd_opt(2023, 2, 6).unwrap();

    // Act
    let pts = times_dt(&params, location, date, None);

    // Assert
    let imsaak = pts.get(&Prayer::Imsaak).unwrap().unwrap();
    assert_eq!(NaiveTime::from_hms_opt(5, 50, 0).unwrap(), imsaak.time);
    assert_eq!(true, imsaak.extreme);
    let fajr = pts.get(&Prayer::Fajr).unwrap().unwrap();
    assert_eq!(NaiveTime::from_hms_opt(5, 51, 0).unwrap(), fajr.time);
    assert_eq!(true, fajr.extreme);
    let shurooq = pts.get(&Prayer::Shurooq).unwrap().unwrap();
    assert_eq!(NaiveTime::from_hms_opt(7, 19, 0).unwrap(), shurooq.time);
    assert_eq!(true, shurooq.extreme);
    let dhuhr = pts.get(&Prayer::Dhuhr).unwrap().unwrap();
    assert_eq!(NaiveTime::from_hms_opt(12, 12, 0).unwrap(), dhuhr.time);
    assert_eq!(true, dhuhr.extreme);
    let asr = pts.get(&Prayer::Asr).unwrap().unwrap();
    assert_eq!(NaiveTime::from_hms_opt(14, 38, 0).unwrap(), asr.time);
    assert_eq!(true, asr.extreme);
    let maghrib = pts.get(&Prayer::Maghrib).unwrap().unwrap();
    assert_eq!(NaiveTime::from_hms_opt(17, 5, 0).unwrap(), maghrib.time);
    assert_eq!(true, maghrib.extreme);
    let isha = pts.get(&Prayer::Isha).unwrap().unwrap();
    assert_eq!(NaiveTime::from_hms_opt(18, 32, 0).unwrap(), isha.time);
    assert_eq!(true, isha.extreme);
}

#[test]
fn test_juneau_ak_default_params_near_lat_fajr_isha_always() {
    // Arrange
    let mut params = Params::new(Method::Isna);
    params.ext_lat_method = ExtremeLatitudeMethod::NearestLatitudeFajrIshaAlways;
    let latitude = Latitude::new(58.3019444).unwrap();
    let longitude = Longitude::new(-134.4197222).unwrap();
    let elevation = Elevation::new(87.).unwrap();
    let coords = Coordinates::new(latitude, longitude, elevation);
    let gmt = Gmt::new(-9.).unwrap();
    let location = Location { coords, gmt };
    let date = NaiveDate::from_ymd_opt(2023, 2, 6).unwrap();

    // Act
    let pts = times_dt(&params, location, date, None);

    // Assert
    let imsaak = pts.get(&Prayer::Imsaak).unwrap().unwrap();
    assert_eq!(NaiveTime::from_hms_opt(5, 50, 0).unwrap(), imsaak.time);
    assert_eq!(true, imsaak.extreme);
    let fajr = pts.get(&Prayer::Fajr).unwrap().unwrap();
    assert_eq!(NaiveTime::from_hms_opt(5, 51, 0).unwrap(), fajr.time);
    assert_eq!(true, fajr.extreme);
    let shurooq = pts.get(&Prayer::Shurooq).unwrap().unwrap();
    assert_eq!(NaiveTime::from_hms_opt(7, 51, 0).unwrap(), shurooq.time);
    assert_eq!(false, shurooq.extreme);
    let dhuhr = pts.get(&Prayer::Dhuhr).unwrap().unwrap();
    assert_eq!(NaiveTime::from_hms_opt(12, 12, 0).unwrap(), dhuhr.time);
    assert_eq!(false, dhuhr.extreme);
    let asr = pts.get(&Prayer::Asr).unwrap().unwrap();
    assert_eq!(NaiveTime::from_hms_opt(14, 3, 0).unwrap(), asr.time);
    assert_eq!(false, asr.extreme);
    let maghrib = pts.get(&Prayer::Maghrib).unwrap().unwrap();
    assert_eq!(NaiveTime::from_hms_opt(16, 33, 0).unwrap(), maghrib.time);
    assert_eq!(false, maghrib.extreme);
    let isha = pts.get(&Prayer::Isha).unwrap().unwrap();
    assert_eq!(NaiveTime::from_hms_opt(18, 32, 0).unwrap(), isha.time);
    assert_eq!(true, isha.extreme);
}

#[test]
fn test_juneau_ak_default_params_near_lat_fajr_isha_inv() {
    // Arrange
    let mut params = Params::new(Method::Isna);
    params.ext_lat_method = ExtremeLatitudeMethod::NearestLatitudeFajrIshaInvalid;
    let latitude = Latitude::new(58.3019444).unwrap();
    let longitude = Longitude::new(-134.4197222).unwrap();
    let elevation = Elevation::new(87.).unwrap();
    let coords = Coordinates::new(latitude, longitude, elevation);
    let gmt = Gmt::new(-9.).unwrap();
    let location = Location { coords, gmt };
    let date = NaiveDate::from_ymd_opt(2023, 2, 6).unwrap();

    // Act
    let pts = times_dt(&params, location, date, None);

    // Assert
    let imsaak = pts.get(&Prayer::Imsaak).unwrap().unwrap();
    assert_eq!(NaiveTime::from_hms_opt(5, 47, 0).unwrap(), imsaak.time);
    assert_eq!(false, imsaak.extreme);
    let fajr = pts.get(&Prayer::Fajr).unwrap().unwrap();
    assert_eq!(NaiveTime::from_hms_opt(5, 58, 0).unwrap(), fajr.time);
    assert_eq!(false, fajr.extreme);
    let shurooq = pts.get(&Prayer::Shurooq).unwrap().unwrap();
    assert_eq!(NaiveTime::from_hms_opt(7, 51, 0).unwrap(), shurooq.time);
    assert_eq!(false, shurooq.extreme);
    let dhuhr = pts.get(&Prayer::Dhuhr).unwrap().unwrap();
    assert_eq!(NaiveTime::from_hms_opt(12, 12, 0).unwrap(), dhuhr.time);
    assert_eq!(false, dhuhr.extreme);
    let asr = pts.get(&Prayer::Asr).unwrap().unwrap();
    assert_eq!(NaiveTime::from_hms_opt(14, 3, 0).unwrap(), asr.time);
    assert_eq!(false, asr.extreme);
    let maghrib = pts.get(&Prayer::Maghrib).unwrap().unwrap();
    assert_eq!(NaiveTime::from_hms_opt(16, 33, 0).unwrap(), maghrib.time);
    assert_eq!(false, maghrib.extreme);
    let isha = pts.get(&Prayer::Isha).unwrap().unwrap();
    assert_eq!(NaiveTime::from_hms_opt(18, 25, 0).unwrap(), isha.time);
    assert_eq!(false, isha.extreme);
}

#[test]
fn test_juneau_ak_default_params_near_good_day_all_prayers_always() {
    // Arrange
    let mut params = Params::new(Method::Isna);
    params.ext_lat_method = ExtremeLatitudeMethod::NearestGoodDayAllPrayersAlways;
    let latitude = Latitude::new(58.3019444).unwrap();
    let longitude = Longitude::new(-134.4197222).unwrap();
    let elevation = Elevation::new(87.).unwrap();
    let coords = Coordinates::new(latitude, longitude, elevation);
    let gmt = Gmt::new(-9.).unwrap();
    let location = Location { coords, gmt };
    let date = NaiveDate::from_ymd_opt(2023, 2, 6).unwrap();

    // Act
    let pts = times_dt(&params, location, date, None);

    // Assert
    let imsaak = pts.get(&Prayer::Imsaak).unwrap().unwrap();
    assert_eq!(NaiveTime::from_hms_opt(5, 57, 0).unwrap(), imsaak.time);
    assert_eq!(true, imsaak.extreme);
    let fajr = pts.get(&Prayer::Fajr).unwrap().unwrap();
    assert_eq!(NaiveTime::from_hms_opt(5, 58, 0).unwrap(), fajr.time);
    assert_eq!(true, fajr.extreme);
    let shurooq = pts.get(&Prayer::Shurooq).unwrap().unwrap();
    assert_eq!(NaiveTime::from_hms_opt(7, 51, 0).unwrap(), shurooq.time);
    assert_eq!(true, shurooq.extreme);
    let dhuhr = pts.get(&Prayer::Dhuhr).unwrap().unwrap();
    assert_eq!(NaiveTime::from_hms_opt(12, 12, 0).unwrap(), dhuhr.time);
    assert_eq!(true, dhuhr.extreme);
    let asr = pts.get(&Prayer::Asr).unwrap().unwrap();
    assert_eq!(NaiveTime::from_hms_opt(14, 3, 0).unwrap(), asr.time);
    assert_eq!(true, asr.extreme);
    let maghrib = pts.get(&Prayer::Maghrib).unwrap().unwrap();
    assert_eq!(NaiveTime::from_hms_opt(16, 33, 0).unwrap(), maghrib.time);
    assert_eq!(true, maghrib.extreme);
    let isha = pts.get(&Prayer::Isha).unwrap().unwrap();
    assert_eq!(NaiveTime::from_hms_opt(18, 25, 0).unwrap(), isha.time);
    assert_eq!(true, isha.extreme);
}

#[test]
fn test_peurto_williams_cl_default_params() {
    // Arrange
    let params = Params::new(Method::Isna);
    let latitude = Latitude::new(-54.9352).unwrap();
    let longitude = Longitude::new(-67.6059).unwrap();
    let elevation = Elevation::new(0.).unwrap();
    let coords = Coordinates::new(latitude, longitude, elevation);
    let gmt = Gmt::new(-3.).unwrap();
    let location = Location { coords, gmt };
    let date = NaiveDate::from_ymd_opt(2023, 2, 6).unwrap();

    // Act
    let pts = times_dt(&params, location, date, None);

    // Assert
    let imsaak = pts.get(&Prayer::Imsaak).unwrap().unwrap();
    assert_eq!(NaiveTime::from_hms_opt(3, 20, 0).unwrap(), imsaak.time);
    assert_eq!(false, imsaak.extreme);
    let fajr = pts.get(&Prayer::Fajr).unwrap().unwrap();
    assert_eq!(NaiveTime::from_hms_opt(3, 43, 0).unwrap(), fajr.time);
    assert_eq!(false, fajr.extreme);
    let shurooq = pts.get(&Prayer::Shurooq).unwrap().unwrap();
    assert_eq!(NaiveTime::from_hms_opt(6, 3, 0).unwrap(), shurooq.time);
    assert_eq!(false, shurooq.extreme);
    let dhuhr = pts.get(&Prayer::Dhuhr).unwrap().unwrap();
    assert_eq!(NaiveTime::from_hms_opt(13, 44, 0).unwrap(), dhuhr.time);
    assert_eq!(false, dhuhr.extreme);
    let asr = pts.get(&Prayer::Asr).unwrap().unwrap();
    assert_eq!(NaiveTime::from_hms_opt(17, 52, 0).unwrap(), asr.time);
    assert_eq!(false, asr.extreme);
    let maghrib = pts.get(&Prayer::Maghrib).unwrap().unwrap();
    assert_eq!(NaiveTime::from_hms_opt(21, 24, 0).unwrap(), maghrib.time);
    assert_eq!(false, maghrib.extreme);
    let isha = pts.get(&Prayer::Isha).unwrap().unwrap();
    assert_eq!(NaiveTime::from_hms_opt(23, 46, 0).unwrap(), isha.time);
    assert_eq!(false, isha.extreme);
}
