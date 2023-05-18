use chrono::{NaiveDate, NaiveTime};
use islamic_prayer_times::{
    prayer_times_dt, Coordinates, Elevation, ExtremeLatitudeMethod, Gmt, Latitude, Location,
    Longitude, Method, Params, Prayer, NEAREST_LATITUDE,
};

#[test]
fn test_juneau_ak_default_params() {
    // Arrange
    let params = Params::new(Method::Isna); // NearestGoodDayFajrIshaInvalid
    let latitude = Latitude::new(58.3019444).unwrap();
    let longitude = Longitude::new(-134.4197222).unwrap();
    let elevation = Elevation::new(87.).unwrap();
    let coords = Coordinates::new(latitude, longitude, elevation);
    let gmt = Gmt::new(-9.).unwrap();
    let location = Location { coords, gmt };
    let date = NaiveDate::from_ymd_opt(2022, 7, 6).unwrap();

    // Act
    let pts = prayer_times_dt(&params, location, date, None);

    // Assert
    let imsaak = pts.get(&Prayer::Imsaak).unwrap().unwrap();
    assert_eq!(NaiveTime::from_hms_opt(0, 15, 0).unwrap(), imsaak.time);
    assert_eq!(true, imsaak.extreme);
    let fajr = pts.get(&Prayer::Fajr).unwrap().unwrap();
    assert_eq!(NaiveTime::from_hms_opt(0, 16, 0).unwrap(), fajr.time);
    assert_eq!(true, fajr.extreme);
    let shurooq = pts.get(&Prayer::Shurooq).unwrap().unwrap();
    assert_eq!(NaiveTime::from_hms_opt(3, 2, 0).unwrap(), shurooq.time);
    assert_eq!(false, shurooq.extreme);
    let dhuhr = pts.get(&Prayer::Dhuhr).unwrap().unwrap();
    assert_eq!(NaiveTime::from_hms_opt(12, 3, 0).unwrap(), dhuhr.time);
    assert_eq!(false, dhuhr.extreme);
    let asr = pts.get(&Prayer::Asr).unwrap().unwrap();
    assert_eq!(NaiveTime::from_hms_opt(16, 38, 0).unwrap(), asr.time);
    assert_eq!(false, asr.extreme);
    let maghrib = pts.get(&Prayer::Maghrib).unwrap().unwrap();
    assert_eq!(NaiveTime::from_hms_opt(21, 2, 0).unwrap(), maghrib.time);
    assert_eq!(false, maghrib.extreme);
    let isha = pts.get(&Prayer::Isha).unwrap().unwrap();
    assert_eq!(NaiveTime::from_hms_opt(23, 51, 0).unwrap(), isha.time);
    assert_eq!(true, isha.extreme);
}

#[test]
fn test_juneau_ak_default_params_near_good_day_all_prayers_always() {
    // Arrange
    let mut params = Params::new(Method::Isna);
    params.extreme_latitude_method = ExtremeLatitudeMethod::NearestGoodDayAllPrayersAlways;
    let latitude = Latitude::new(58.3019444).unwrap();
    let longitude = Longitude::new(-134.4197222).unwrap();
    let elevation = Elevation::new(87.).unwrap();
    let coords = Coordinates::new(latitude, longitude, elevation);
    let gmt = Gmt::new(-9.).unwrap();
    let location = Location { coords, gmt };
    let date = NaiveDate::from_ymd_opt(2022, 7, 6).unwrap();

    // Act
    let pts = prayer_times_dt(&params, location, date, None);

    // Assert
    let imsaak = pts.get(&Prayer::Imsaak).unwrap().unwrap();
    assert_eq!(NaiveTime::from_hms_opt(0, 15, 0).unwrap(), imsaak.time);
    assert_eq!(true, imsaak.extreme);
    let fajr = pts.get(&Prayer::Fajr).unwrap().unwrap();
    assert_eq!(NaiveTime::from_hms_opt(0, 16, 0).unwrap(), fajr.time);
    assert_eq!(true, fajr.extreme);
    let shurooq = pts.get(&Prayer::Shurooq).unwrap().unwrap();
    assert_eq!(NaiveTime::from_hms_opt(4, 0, 0).unwrap(), shurooq.time);
    assert_eq!(true, shurooq.extreme);
    let dhuhr = pts.get(&Prayer::Dhuhr).unwrap().unwrap();
    assert_eq!(NaiveTime::from_hms_opt(12, 4, 0).unwrap(), dhuhr.time);
    assert_eq!(true, dhuhr.extreme);
    let asr = pts.get(&Prayer::Asr).unwrap().unwrap();
    assert_eq!(NaiveTime::from_hms_opt(16, 18, 0).unwrap(), asr.time);
    assert_eq!(true, asr.extreme);
    let maghrib = pts.get(&Prayer::Maghrib).unwrap().unwrap();
    assert_eq!(NaiveTime::from_hms_opt(20, 5, 0).unwrap(), maghrib.time);
    assert_eq!(true, maghrib.extreme);
    let isha = pts.get(&Prayer::Isha).unwrap().unwrap();
    assert_eq!(NaiveTime::from_hms_opt(23, 51, 0).unwrap(), isha.time);
    assert_eq!(true, isha.extreme);
}

#[test]
fn test_juneau_ak_default_params_near_lat_all_prayers_always() {
    // Arrange
    let mut params = Params::new(Method::Isna);
    params.extreme_latitude_method =
        ExtremeLatitudeMethod::NearestLatitudeAllPrayersAlways(NEAREST_LATITUDE);
    let latitude = Latitude::new(58.3019444).unwrap();
    let longitude = Longitude::new(-134.4197222).unwrap();
    let elevation = Elevation::new(87.).unwrap();
    let coords = Coordinates::new(latitude, longitude, elevation);
    let gmt = Gmt::new(-9.).unwrap();
    let location = Location { coords, gmt };
    let date = NaiveDate::from_ymd_opt(2022, 7, 6).unwrap();

    // Act
    let pts = prayer_times_dt(&params, location, date, None);

    // Assert
    let imsaak = pts.get(&Prayer::Imsaak).unwrap().unwrap();
    assert_eq!(NaiveTime::from_hms_opt(1, 47, 0).unwrap(), imsaak.time);
    assert_eq!(true, imsaak.extreme);
    let fajr = pts.get(&Prayer::Fajr).unwrap().unwrap();
    assert_eq!(NaiveTime::from_hms_opt(1, 48, 0).unwrap(), fajr.time);
    assert_eq!(true, fajr.extreme);
    let shurooq = pts.get(&Prayer::Shurooq).unwrap().unwrap();
    assert_eq!(NaiveTime::from_hms_opt(4, 3, 0).unwrap(), shurooq.time);
    assert_eq!(true, shurooq.extreme);
    let dhuhr = pts.get(&Prayer::Dhuhr).unwrap().unwrap();
    assert_eq!(NaiveTime::from_hms_opt(12, 3, 0).unwrap(), dhuhr.time);
    assert_eq!(true, dhuhr.extreme);
    let asr = pts.get(&Prayer::Asr).unwrap().unwrap();
    assert_eq!(NaiveTime::from_hms_opt(16, 18, 0).unwrap(), asr.time);
    assert_eq!(true, asr.extreme);
    let maghrib = pts.get(&Prayer::Maghrib).unwrap().unwrap();
    assert_eq!(NaiveTime::from_hms_opt(20, 1, 0).unwrap(), maghrib.time);
    assert_eq!(true, maghrib.extreme);
    let isha = pts.get(&Prayer::Isha).unwrap().unwrap();
    assert_eq!(NaiveTime::from_hms_opt(22, 17, 0).unwrap(), isha.time);
    assert_eq!(true, isha.extreme);
}

#[test]
fn test_juneau_ak_default_params_near_lat_fajr_isha_always() {
    // Arrange
    let mut params = Params::new(Method::Isna);
    params.extreme_latitude_method =
        ExtremeLatitudeMethod::NearestLatitudeFajrIshaAlways(NEAREST_LATITUDE);
    let latitude = Latitude::new(58.3019444).unwrap();
    let longitude = Longitude::new(-134.4197222).unwrap();
    let elevation = Elevation::new(87.).unwrap();
    let coords = Coordinates::new(latitude, longitude, elevation);
    let gmt = Gmt::new(-9.).unwrap();
    let location = Location { coords, gmt };
    let date = NaiveDate::from_ymd_opt(2022, 7, 6).unwrap();

    // Act
    let pts = prayer_times_dt(&params, location, date, None);

    // Assert
    let imsaak = pts.get(&Prayer::Imsaak).unwrap().unwrap();
    assert_eq!(NaiveTime::from_hms_opt(1, 47, 0).unwrap(), imsaak.time);
    assert_eq!(true, imsaak.extreme);
    let fajr = pts.get(&Prayer::Fajr).unwrap().unwrap();
    assert_eq!(NaiveTime::from_hms_opt(1, 48, 0).unwrap(), fajr.time);
    assert_eq!(true, fajr.extreme);
    let shurooq = pts.get(&Prayer::Shurooq).unwrap().unwrap();
    assert_eq!(NaiveTime::from_hms_opt(3, 2, 0).unwrap(), shurooq.time);
    assert_eq!(false, shurooq.extreme);
    let dhuhr = pts.get(&Prayer::Dhuhr).unwrap().unwrap();
    assert_eq!(NaiveTime::from_hms_opt(12, 3, 0).unwrap(), dhuhr.time);
    assert_eq!(false, dhuhr.extreme);
    let asr = pts.get(&Prayer::Asr).unwrap().unwrap();
    assert_eq!(NaiveTime::from_hms_opt(16, 38, 0).unwrap(), asr.time);
    assert_eq!(false, asr.extreme);
    let maghrib = pts.get(&Prayer::Maghrib).unwrap().unwrap();
    assert_eq!(NaiveTime::from_hms_opt(21, 2, 0).unwrap(), maghrib.time);
    assert_eq!(false, maghrib.extreme);
    let isha = pts.get(&Prayer::Isha).unwrap().unwrap();
    assert_eq!(NaiveTime::from_hms_opt(22, 17, 0).unwrap(), isha.time);
    assert_eq!(true, isha.extreme);
}

#[test]
fn test_juneau_ak_default_params_near_lat_fajr_isha_inv() {
    // Arrange
    let mut params = Params::new(Method::Isna);
    params.extreme_latitude_method =
        ExtremeLatitudeMethod::NearestLatitudeFajrIshaInvalid(NEAREST_LATITUDE);
    let latitude = Latitude::new(58.3019444).unwrap();
    let longitude = Longitude::new(-134.4197222).unwrap();
    let elevation = Elevation::new(87.).unwrap();
    let coords = Coordinates::new(latitude, longitude, elevation);
    let gmt = Gmt::new(-9.).unwrap();
    let location = Location { coords, gmt };
    let date = NaiveDate::from_ymd_opt(2022, 7, 6).unwrap();

    // Act
    let pts = prayer_times_dt(&params, location, date, None);

    // Assert
    let imsaak = pts.get(&Prayer::Imsaak).unwrap().unwrap();
    assert_eq!(NaiveTime::from_hms_opt(1, 47, 0).unwrap(), imsaak.time);
    assert_eq!(true, imsaak.extreme);
    let fajr = pts.get(&Prayer::Fajr).unwrap().unwrap();
    assert_eq!(NaiveTime::from_hms_opt(1, 48, 0).unwrap(), fajr.time);
    assert_eq!(true, fajr.extreme);
    let shurooq = pts.get(&Prayer::Shurooq).unwrap().unwrap();
    assert_eq!(NaiveTime::from_hms_opt(3, 2, 0).unwrap(), shurooq.time);
    assert_eq!(false, shurooq.extreme);
    let dhuhr = pts.get(&Prayer::Dhuhr).unwrap().unwrap();
    assert_eq!(NaiveTime::from_hms_opt(12, 3, 0).unwrap(), dhuhr.time);
    assert_eq!(false, dhuhr.extreme);
    let asr = pts.get(&Prayer::Asr).unwrap().unwrap();
    assert_eq!(NaiveTime::from_hms_opt(16, 38, 0).unwrap(), asr.time);
    assert_eq!(false, asr.extreme);
    let maghrib = pts.get(&Prayer::Maghrib).unwrap().unwrap();
    assert_eq!(NaiveTime::from_hms_opt(21, 2, 0).unwrap(), maghrib.time);
    assert_eq!(false, maghrib.extreme);
    let isha = pts.get(&Prayer::Isha).unwrap().unwrap();
    assert_eq!(NaiveTime::from_hms_opt(22, 17, 0).unwrap(), isha.time);
    assert_eq!(true, isha.extreme);
}

#[test]
fn test_juneau_ak_default_params_sev_day_fajr_isha_always() {
    // Arrange
    let mut params = Params::new(Method::Isna);
    params.extreme_latitude_method = ExtremeLatitudeMethod::SeventhOfDayFajrIshaAlways;
    let latitude = Latitude::new(58.3019444).unwrap();
    let longitude = Longitude::new(-134.4197222).unwrap();
    let elevation = Elevation::new(87.).unwrap();
    let coords = Coordinates::new(latitude, longitude, elevation);
    let gmt = Gmt::new(-9.).unwrap();
    let location = Location { coords, gmt };
    let date = NaiveDate::from_ymd_opt(2022, 7, 6).unwrap();

    // Act
    let pts = prayer_times_dt(&params, location, date, None);

    // Assert
    let imsaak = pts.get(&Prayer::Imsaak).unwrap().unwrap();
    assert_eq!(NaiveTime::from_hms_opt(0, 27, 0).unwrap(), imsaak.time);
    assert_eq!(true, imsaak.extreme);
    let fajr = pts.get(&Prayer::Fajr).unwrap().unwrap();
    assert_eq!(NaiveTime::from_hms_opt(0, 29, 0).unwrap(), fajr.time);
    assert_eq!(true, fajr.extreme);
    let shurooq = pts.get(&Prayer::Shurooq).unwrap().unwrap();
    assert_eq!(NaiveTime::from_hms_opt(3, 2, 0).unwrap(), shurooq.time);
    assert_eq!(false, shurooq.extreme);
    let dhuhr = pts.get(&Prayer::Dhuhr).unwrap().unwrap();
    assert_eq!(NaiveTime::from_hms_opt(12, 3, 0).unwrap(), dhuhr.time);
    assert_eq!(false, dhuhr.extreme);
    let asr = pts.get(&Prayer::Asr).unwrap().unwrap();
    assert_eq!(NaiveTime::from_hms_opt(16, 38, 0).unwrap(), asr.time);
    assert_eq!(false, asr.extreme);
    let maghrib = pts.get(&Prayer::Maghrib).unwrap().unwrap();
    assert_eq!(NaiveTime::from_hms_opt(21, 2, 0).unwrap(), maghrib.time);
    assert_eq!(false, maghrib.extreme);
    let isha = pts.get(&Prayer::Isha).unwrap().unwrap();
    assert_eq!(NaiveTime::from_hms_opt(23, 36, 0).unwrap(), isha.time);
    assert_eq!(true, isha.extreme);
}

#[test]
fn test_juneau_ak_default_params_sev_day_fajr_isha_inv() {
    // Arrange
    let mut params = Params::new(Method::Isna);
    params.extreme_latitude_method = ExtremeLatitudeMethod::SeventhOfDayFajrIshaInvalid;
    let latitude = Latitude::new(58.3019444).unwrap();
    let longitude = Longitude::new(-134.4197222).unwrap();
    let elevation = Elevation::new(87.).unwrap();
    let coords = Coordinates::new(latitude, longitude, elevation);
    let gmt = Gmt::new(-9.).unwrap();
    let location = Location { coords, gmt };
    let date = NaiveDate::from_ymd_opt(2022, 7, 6).unwrap();

    // Act
    let pts = prayer_times_dt(&params, location, date, None);

    // Assert
    let imsaak = pts.get(&Prayer::Imsaak).unwrap().unwrap();
    assert_eq!(NaiveTime::from_hms_opt(0, 27, 0).unwrap(), imsaak.time);
    assert_eq!(true, imsaak.extreme);
    let fajr = pts.get(&Prayer::Fajr).unwrap().unwrap();
    assert_eq!(NaiveTime::from_hms_opt(0, 29, 0).unwrap(), fajr.time);
    assert_eq!(true, fajr.extreme);
    let shurooq = pts.get(&Prayer::Shurooq).unwrap().unwrap();
    assert_eq!(NaiveTime::from_hms_opt(3, 2, 0).unwrap(), shurooq.time);
    assert_eq!(false, shurooq.extreme);
    let dhuhr = pts.get(&Prayer::Dhuhr).unwrap().unwrap();
    assert_eq!(NaiveTime::from_hms_opt(12, 3, 0).unwrap(), dhuhr.time);
    assert_eq!(false, dhuhr.extreme);
    let asr = pts.get(&Prayer::Asr).unwrap().unwrap();
    assert_eq!(NaiveTime::from_hms_opt(16, 38, 0).unwrap(), asr.time);
    assert_eq!(false, asr.extreme);
    let maghrib = pts.get(&Prayer::Maghrib).unwrap().unwrap();
    assert_eq!(NaiveTime::from_hms_opt(21, 2, 0).unwrap(), maghrib.time);
    assert_eq!(false, maghrib.extreme);
    let isha = pts.get(&Prayer::Isha).unwrap().unwrap();
    assert_eq!(NaiveTime::from_hms_opt(23, 36, 0).unwrap(), isha.time);
    assert_eq!(true, isha.extreme);
}

#[test]
fn test_juneau_ak_default_params_sev_night_fajr_isha_always() {
    // Arrange
    let mut params = Params::new(Method::Isna);
    params.extreme_latitude_method = ExtremeLatitudeMethod::SeventhOfNightFajrIshaAlways;
    let latitude = Latitude::new(58.3019444).unwrap();
    let longitude = Longitude::new(-134.4197222).unwrap();
    let elevation = Elevation::new(87.).unwrap();
    let coords = Coordinates::new(latitude, longitude, elevation);
    let gmt = Gmt::new(-9.).unwrap();
    let location = Location { coords, gmt };
    let date = NaiveDate::from_ymd_opt(2022, 7, 6).unwrap();

    // Act
    let pts = prayer_times_dt(&params, location, date, None);

    // Assert
    let imsaak = pts.get(&Prayer::Imsaak).unwrap().unwrap();
    assert_eq!(NaiveTime::from_hms_opt(2, 10, 0).unwrap(), imsaak.time);
    assert_eq!(true, imsaak.extreme);
    let fajr = pts.get(&Prayer::Fajr).unwrap().unwrap();
    assert_eq!(NaiveTime::from_hms_opt(2, 11, 0).unwrap(), fajr.time);
    assert_eq!(true, fajr.extreme);
    let shurooq = pts.get(&Prayer::Shurooq).unwrap().unwrap();
    assert_eq!(NaiveTime::from_hms_opt(3, 2, 0).unwrap(), shurooq.time);
    assert_eq!(false, shurooq.extreme);
    let dhuhr = pts.get(&Prayer::Dhuhr).unwrap().unwrap();
    assert_eq!(NaiveTime::from_hms_opt(12, 3, 0).unwrap(), dhuhr.time);
    assert_eq!(false, dhuhr.extreme);
    let asr = pts.get(&Prayer::Asr).unwrap().unwrap();
    assert_eq!(NaiveTime::from_hms_opt(16, 38, 0).unwrap(), asr.time);
    assert_eq!(false, asr.extreme);
    let maghrib = pts.get(&Prayer::Maghrib).unwrap().unwrap();
    assert_eq!(NaiveTime::from_hms_opt(21, 2, 0).unwrap(), maghrib.time);
    assert_eq!(false, maghrib.extreme);
    let isha = pts.get(&Prayer::Isha).unwrap().unwrap();
    assert_eq!(NaiveTime::from_hms_opt(21, 53, 0).unwrap(), isha.time);
    assert_eq!(true, isha.extreme);
}

#[test]
fn test_juneau_ak_default_params_sev_night_fajr_isha_inv() {
    // Arrange
    let mut params = Params::new(Method::Isna);
    params.extreme_latitude_method = ExtremeLatitudeMethod::SeventhOfNightFajrIshaInvalid;
    let latitude = Latitude::new(58.3019444).unwrap();
    let longitude = Longitude::new(-134.4197222).unwrap();
    let elevation = Elevation::new(87.).unwrap();
    let coords = Coordinates::new(latitude, longitude, elevation);
    let gmt = Gmt::new(-9.).unwrap();
    let location = Location { coords, gmt };
    let date = NaiveDate::from_ymd_opt(2022, 7, 6).unwrap();

    // Act
    let pts = prayer_times_dt(&params, location, date, None);

    // Assert
    let imsaak = pts.get(&Prayer::Imsaak).unwrap().unwrap();
    assert_eq!(NaiveTime::from_hms_opt(2, 10, 0).unwrap(), imsaak.time);
    assert_eq!(true, imsaak.extreme);
    let fajr = pts.get(&Prayer::Fajr).unwrap().unwrap();
    assert_eq!(NaiveTime::from_hms_opt(2, 11, 0).unwrap(), fajr.time);
    assert_eq!(true, fajr.extreme);
    let shurooq = pts.get(&Prayer::Shurooq).unwrap().unwrap();
    assert_eq!(NaiveTime::from_hms_opt(3, 2, 0).unwrap(), shurooq.time);
    assert_eq!(false, shurooq.extreme);
    let dhuhr = pts.get(&Prayer::Dhuhr).unwrap().unwrap();
    assert_eq!(NaiveTime::from_hms_opt(12, 3, 0).unwrap(), dhuhr.time);
    assert_eq!(false, dhuhr.extreme);
    let asr = pts.get(&Prayer::Asr).unwrap().unwrap();
    assert_eq!(NaiveTime::from_hms_opt(16, 38, 0).unwrap(), asr.time);
    assert_eq!(false, asr.extreme);
    let maghrib = pts.get(&Prayer::Maghrib).unwrap().unwrap();
    assert_eq!(NaiveTime::from_hms_opt(21, 2, 0).unwrap(), maghrib.time);
    assert_eq!(false, maghrib.extreme);
    let isha = pts.get(&Prayer::Isha).unwrap().unwrap();
    assert_eq!(NaiveTime::from_hms_opt(21, 53, 0).unwrap(), isha.time);
    assert_eq!(true, isha.extreme);
}

#[test]
fn test_juneau_ak_default_params_half_night_fajr_isha_always() {
    use Prayer::*;

    // Arrange
    let mut params = Params::new(Method::Isna);
    params.extreme_latitude_method = ExtremeLatitudeMethod::HalfOfNightFajrIshaAlways;
    *params.intervals.get_mut(&Fajr).unwrap() = -10.;
    *params.intervals.get_mut(&Isha).unwrap() = -10.;
    let latitude = Latitude::new(58.3019444).unwrap();
    let longitude = Longitude::new(-134.4197222).unwrap();
    let elevation = Elevation::new(87.).unwrap();
    let coords = Coordinates::new(latitude, longitude, elevation);
    let gmt = Gmt::new(-9.).unwrap();
    let location = Location { coords, gmt };
    let date = NaiveDate::from_ymd_opt(2022, 7, 6).unwrap();

    // Act
    let pts = prayer_times_dt(&params, location, date, None);

    // Assert
    let imsaak = pts.get(&Prayer::Imsaak).unwrap().unwrap();
    assert_eq!(NaiveTime::from_hms_opt(0, 6, 0).unwrap(), imsaak.time);
    assert_eq!(true, imsaak.extreme);
    let fajr = pts.get(&Prayer::Fajr).unwrap().unwrap();
    assert_eq!(NaiveTime::from_hms_opt(0, 8, 0).unwrap(), fajr.time);
    assert_eq!(true, fajr.extreme);
    let shurooq = pts.get(&Prayer::Shurooq).unwrap().unwrap();
    assert_eq!(NaiveTime::from_hms_opt(3, 2, 0).unwrap(), shurooq.time);
    assert_eq!(false, shurooq.extreme);
    let dhuhr = pts.get(&Prayer::Dhuhr).unwrap().unwrap();
    assert_eq!(NaiveTime::from_hms_opt(12, 3, 0).unwrap(), dhuhr.time);
    assert_eq!(false, dhuhr.extreme);
    let asr = pts.get(&Prayer::Asr).unwrap().unwrap();
    assert_eq!(NaiveTime::from_hms_opt(16, 38, 0).unwrap(), asr.time);
    assert_eq!(false, asr.extreme);
    let maghrib = pts.get(&Prayer::Maghrib).unwrap().unwrap();
    assert_eq!(NaiveTime::from_hms_opt(21, 2, 0).unwrap(), maghrib.time);
    assert_eq!(false, maghrib.extreme);
    let isha = pts.get(&Prayer::Isha).unwrap().unwrap();
    assert_eq!(NaiveTime::from_hms_opt(23, 48, 0).unwrap(), isha.time);
    assert_eq!(true, isha.extreme);
}

#[test]
fn test_juneau_ak_default_params_half_night_fajr_isha_inv() {
    use Prayer::*;

    // Arrange
    let mut params = Params::new(Method::Isna);
    params.extreme_latitude_method = ExtremeLatitudeMethod::HalfOfNightFajrIshaInvalid;
    *params.intervals.get_mut(&Fajr).unwrap() = -10.;
    *params.intervals.get_mut(&Isha).unwrap() = -10.;
    let latitude = Latitude::new(58.3019444).unwrap();
    let longitude = Longitude::new(-134.4197222).unwrap();
    let elevation = Elevation::new(87.).unwrap();
    let coords = Coordinates::new(latitude, longitude, elevation);
    let gmt = Gmt::new(-9.).unwrap();
    let location = Location { coords, gmt };
    let date = NaiveDate::from_ymd_opt(2022, 7, 6).unwrap();

    // Act
    let pts = prayer_times_dt(&params, location, date, None);

    // Assert
    let imsaak = pts.get(&Prayer::Imsaak).unwrap().unwrap();
    assert_eq!(NaiveTime::from_hms_opt(0, 6, 0).unwrap(), imsaak.time);
    assert_eq!(true, imsaak.extreme);
    let fajr = pts.get(&Prayer::Fajr).unwrap().unwrap();
    assert_eq!(NaiveTime::from_hms_opt(0, 8, 0).unwrap(), fajr.time);
    assert_eq!(true, fajr.extreme);
    let shurooq = pts.get(&Prayer::Shurooq).unwrap().unwrap();
    assert_eq!(NaiveTime::from_hms_opt(3, 2, 0).unwrap(), shurooq.time);
    assert_eq!(false, shurooq.extreme);
    let dhuhr = pts.get(&Prayer::Dhuhr).unwrap().unwrap();
    assert_eq!(NaiveTime::from_hms_opt(12, 3, 0).unwrap(), dhuhr.time);
    assert_eq!(false, dhuhr.extreme);
    let asr = pts.get(&Prayer::Asr).unwrap().unwrap();
    assert_eq!(NaiveTime::from_hms_opt(16, 38, 0).unwrap(), asr.time);
    assert_eq!(false, asr.extreme);
    let maghrib = pts.get(&Prayer::Maghrib).unwrap().unwrap();
    assert_eq!(NaiveTime::from_hms_opt(21, 2, 0).unwrap(), maghrib.time);
    assert_eq!(false, maghrib.extreme);
    let isha = pts.get(&Prayer::Isha).unwrap().unwrap();
    assert_eq!(NaiveTime::from_hms_opt(23, 48, 0).unwrap(), isha.time);
    assert_eq!(true, isha.extreme);
}

#[test]
fn test_juneau_ak_default_params_mins_from_fajr_isha_always() {
    use Prayer::*;

    // Arrange
    let mut params = Params::new(Method::Isna);
    params.extreme_latitude_method = ExtremeLatitudeMethod::MinutesFromMaghribFajrIshaAlways;
    *params.intervals.get_mut(&Fajr).unwrap() = 10.;
    *params.intervals.get_mut(&Isha).unwrap() = 10.;
    let latitude = Latitude::new(58.3019444).unwrap();
    let longitude = Longitude::new(-134.4197222).unwrap();
    let elevation = Elevation::new(87.).unwrap();
    let coords = Coordinates::new(latitude, longitude, elevation);
    let gmt = Gmt::new(-9.).unwrap();
    let location = Location { coords, gmt };
    let date = NaiveDate::from_ymd_opt(2022, 7, 6).unwrap();

    // Act
    let pts = prayer_times_dt(&params, location, date, None);

    // Assert
    let imsaak = pts.get(&Prayer::Imsaak).unwrap().unwrap();
    assert_eq!(NaiveTime::from_hms_opt(2, 51, 0).unwrap(), imsaak.time);
    assert_eq!(true, imsaak.extreme);
    let fajr = pts.get(&Prayer::Fajr).unwrap().unwrap();
    assert_eq!(NaiveTime::from_hms_opt(2, 53, 0).unwrap(), fajr.time);
    assert_eq!(true, fajr.extreme);
    let shurooq = pts.get(&Prayer::Shurooq).unwrap().unwrap();
    assert_eq!(NaiveTime::from_hms_opt(3, 2, 0).unwrap(), shurooq.time);
    assert_eq!(false, shurooq.extreme);
    let dhuhr = pts.get(&Prayer::Dhuhr).unwrap().unwrap();
    assert_eq!(NaiveTime::from_hms_opt(12, 3, 0).unwrap(), dhuhr.time);
    assert_eq!(false, dhuhr.extreme);
    let asr = pts.get(&Prayer::Asr).unwrap().unwrap();
    assert_eq!(NaiveTime::from_hms_opt(16, 38, 0).unwrap(), asr.time);
    assert_eq!(false, asr.extreme);
    let maghrib = pts.get(&Prayer::Maghrib).unwrap().unwrap();
    assert_eq!(NaiveTime::from_hms_opt(21, 2, 0).unwrap(), maghrib.time);
    assert_eq!(false, maghrib.extreme);
    let isha = pts.get(&Prayer::Isha).unwrap().unwrap();
    assert_eq!(NaiveTime::from_hms_opt(21, 12, 0).unwrap(), isha.time);
    assert_eq!(true, isha.extreme);
}

#[test]
fn test_juneau_ak_default_params_mins_from_fajr_isha_inv() {
    use Prayer::*;

    // Arrange
    let mut params = Params::new(Method::Isna);
    params.extreme_latitude_method = ExtremeLatitudeMethod::MinutesFromMaghribFajrIshaInvalid;
    *params.intervals.get_mut(&Fajr).unwrap() = 10.;
    *params.intervals.get_mut(&Isha).unwrap() = 10.;
    let latitude = Latitude::new(58.3019444).unwrap();
    let longitude = Longitude::new(-134.4197222).unwrap();
    let elevation = Elevation::new(87.).unwrap();
    let coords = Coordinates::new(latitude, longitude, elevation);
    let gmt = Gmt::new(-9.).unwrap();
    let location = Location { coords, gmt };
    let date = NaiveDate::from_ymd_opt(2022, 7, 6).unwrap();

    // Act
    let pts = prayer_times_dt(&params, location, date, None);

    // Assert
    let imsaak = pts.get(&Prayer::Imsaak).unwrap().unwrap();
    assert_eq!(NaiveTime::from_hms_opt(2, 51, 0).unwrap(), imsaak.time);
    assert_eq!(true, imsaak.extreme);
    let fajr = pts.get(&Prayer::Fajr).unwrap().unwrap();
    assert_eq!(NaiveTime::from_hms_opt(2, 53, 0).unwrap(), fajr.time);
    assert_eq!(true, fajr.extreme);
    let shurooq = pts.get(&Prayer::Shurooq).unwrap().unwrap();
    assert_eq!(NaiveTime::from_hms_opt(3, 2, 0).unwrap(), shurooq.time);
    assert_eq!(false, shurooq.extreme);
    let dhuhr = pts.get(&Prayer::Dhuhr).unwrap().unwrap();
    assert_eq!(NaiveTime::from_hms_opt(12, 3, 0).unwrap(), dhuhr.time);
    assert_eq!(false, dhuhr.extreme);
    let asr = pts.get(&Prayer::Asr).unwrap().unwrap();
    assert_eq!(NaiveTime::from_hms_opt(16, 38, 0).unwrap(), asr.time);
    assert_eq!(false, asr.extreme);
    let maghrib = pts.get(&Prayer::Maghrib).unwrap().unwrap();
    assert_eq!(NaiveTime::from_hms_opt(21, 2, 0).unwrap(), maghrib.time);
    assert_eq!(false, maghrib.extreme);
    let isha = pts.get(&Prayer::Isha).unwrap().unwrap();
    assert_eq!(NaiveTime::from_hms_opt(21, 12, 0).unwrap(), isha.time);
    assert_eq!(true, isha.extreme);
}
