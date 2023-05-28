use chrono::{NaiveDate, NaiveTime};
use islamic_prayer_times::{
    geo::coordinates::{Coordinates, Elevation, Gmt, Latitude, Longitude},
    prayer_times::{
        params::{Method, Params},
        prayer_times_dt, Location, Prayer,
    },
};

#[test]
fn test_peurto_williams_cl_default_params_mwl() {
    use Prayer::*;

    // Arrange
    let params = Params::new(Method::Mwl);
    let latitude = Latitude::try_from(-54.9352).unwrap();
    let longitude = Longitude::try_from(-67.6059).unwrap();
    let elevation = Elevation::default();
    let coords = Coordinates::new(latitude, longitude, elevation);
    let gmt = Gmt::try_from(-3.).unwrap();
    let location = Location { coords, gmt };
    let date = NaiveDate::from_ymd_opt(2023, 2, 16).unwrap();

    // Act
    let pts = prayer_times_dt(&params, location, date, None);

    // Assert
    let imsaak = pts.get(&Imsaak).unwrap().unwrap();
    assert_eq!(NaiveTime::from_hms_opt(3, 23, 0).unwrap(), imsaak.time);
    assert_eq!(false, imsaak.extreme);
    let fajr = pts.get(&Fajr).unwrap().unwrap();
    assert_eq!(NaiveTime::from_hms_opt(3, 45, 0).unwrap(), fajr.time);
    assert_eq!(false, fajr.extreme);
    let shurooq = pts.get(&Shurooq).unwrap().unwrap();
    assert_eq!(NaiveTime::from_hms_opt(6, 25, 0).unwrap(), shurooq.time);
    assert_eq!(false, shurooq.extreme);
    let dhuhr = pts.get(&Dhuhr).unwrap().unwrap();
    assert_eq!(NaiveTime::from_hms_opt(13, 44, 0).unwrap(), dhuhr.time);
    assert_eq!(false, dhuhr.extreme);
    let asr = pts.get(&Asr).unwrap().unwrap();
    assert_eq!(NaiveTime::from_hms_opt(17, 42, 0).unwrap(), asr.time);
    assert_eq!(false, asr.extreme);
    let maghrib = pts.get(&Maghrib).unwrap().unwrap();
    assert_eq!(NaiveTime::from_hms_opt(21, 2, 0).unwrap(), maghrib.time);
    assert_eq!(false, maghrib.extreme);
    let isha = pts.get(&Isha).unwrap().unwrap();
    assert_eq!(NaiveTime::from_hms_opt(23, 31, 0).unwrap(), isha.time);
    assert_eq!(false, isha.extreme);
}

#[test]
fn test_cairo_eg_default_params_egyptian() {
    use Prayer::*;

    // Arrange
    let params = Params::new(Method::Egyptian);
    let latitude = Latitude::try_from(30.0444).unwrap();
    let longitude = Longitude::try_from(31.2357).unwrap();
    let elevation = Elevation::try_from(75.).unwrap();
    let coords = Coordinates::new(latitude, longitude, elevation);
    let gmt = Gmt::try_from(2.).unwrap();
    let location = Location { coords, gmt };
    let date = NaiveDate::from_ymd_opt(2023, 2, 18).unwrap();

    // Act
    let pts = prayer_times_dt(&params, location, date, None);

    // Assert
    let imsaak = pts.get(&Imsaak).unwrap().unwrap();
    assert_eq!(NaiveTime::from_hms_opt(4, 56, 0).unwrap(), imsaak.time);
    assert_eq!(false, imsaak.extreme);
    let fajr = pts.get(&Fajr).unwrap().unwrap();
    assert_eq!(NaiveTime::from_hms_opt(5, 3, 0).unwrap(), fajr.time);
    assert_eq!(false, fajr.extreme);
    let shurooq = pts.get(&Shurooq).unwrap().unwrap();
    assert_eq!(NaiveTime::from_hms_opt(6, 32, 0).unwrap(), shurooq.time);
    assert_eq!(false, shurooq.extreme);
    let dhuhr = pts.get(&Dhuhr).unwrap().unwrap();
    assert_eq!(NaiveTime::from_hms_opt(12, 9, 0).unwrap(), dhuhr.time);
    assert_eq!(false, dhuhr.extreme);
    let asr = pts.get(&Asr).unwrap().unwrap();
    assert_eq!(NaiveTime::from_hms_opt(15, 20, 0).unwrap(), asr.time);
    assert_eq!(false, asr.extreme);
    let maghrib = pts.get(&Maghrib).unwrap().unwrap();
    assert_eq!(NaiveTime::from_hms_opt(17, 46, 0).unwrap(), maghrib.time);
    assert_eq!(false, maghrib.extreme);
    let isha = pts.get(&Isha).unwrap().unwrap();
    assert_eq!(NaiveTime::from_hms_opt(19, 5, 0).unwrap(), isha.time);
    assert_eq!(false, isha.extreme);
}

#[test]
fn test_cairo_eg_default_params_egypt() {
    use Prayer::*;

    // Arrange
    let params = Params::new(Method::Egypt);
    let latitude = Latitude::try_from(30.0444).unwrap();
    let longitude = Longitude::try_from(31.2357).unwrap();
    let elevation = Elevation::try_from(75.).unwrap();
    let coords = Coordinates::new(latitude, longitude, elevation);
    let gmt = Gmt::try_from(2.).unwrap();
    let location = Location { coords, gmt };
    let date = NaiveDate::from_ymd_opt(2023, 2, 18).unwrap();

    // Act
    let pts = prayer_times_dt(&params, location, date, None);

    // Assert
    let imsaak = pts.get(&Imsaak).unwrap().unwrap();
    assert_eq!(NaiveTime::from_hms_opt(4, 59, 0).unwrap(), imsaak.time);
    assert_eq!(false, imsaak.extreme);
    let fajr = pts.get(&Fajr).unwrap().unwrap();
    assert_eq!(NaiveTime::from_hms_opt(5, 6, 0).unwrap(), fajr.time);
    assert_eq!(false, fajr.extreme);
    let shurooq = pts.get(&Shurooq).unwrap().unwrap();
    assert_eq!(NaiveTime::from_hms_opt(6, 32, 0).unwrap(), shurooq.time);
    assert_eq!(false, shurooq.extreme);
    let dhuhr = pts.get(&Dhuhr).unwrap().unwrap();
    assert_eq!(NaiveTime::from_hms_opt(12, 9, 0).unwrap(), dhuhr.time);
    assert_eq!(false, dhuhr.extreme);
    let asr = pts.get(&Asr).unwrap().unwrap();
    assert_eq!(NaiveTime::from_hms_opt(15, 20, 0).unwrap(), asr.time);
    assert_eq!(false, asr.extreme);
    let maghrib = pts.get(&Maghrib).unwrap().unwrap();
    assert_eq!(NaiveTime::from_hms_opt(17, 46, 0).unwrap(), maghrib.time);
    assert_eq!(false, maghrib.extreme);
    let isha = pts.get(&Isha).unwrap().unwrap();
    assert_eq!(NaiveTime::from_hms_opt(19, 3, 0).unwrap(), isha.time);
    assert_eq!(false, isha.extreme);
}

#[test]
fn test_mecca_sa_default_params_umm_al_qurra() {
    use Prayer::*;

    // Arrange
    let params = Params::new(Method::UmmAlQurra);
    let latitude = Latitude::try_from(21.3891).unwrap();
    let longitude = Longitude::try_from(39.8579).unwrap();
    let elevation = Elevation::try_from(909.).unwrap();
    let coords = Coordinates::new(latitude, longitude, elevation);
    let gmt = Gmt::try_from(3.).unwrap();
    let location = Location { coords, gmt };
    let date = NaiveDate::from_ymd_opt(2023, 2, 19).unwrap();

    // Act
    let pts = prayer_times_dt(&params, location, date, None);

    // Assert
    let imsaak = pts.get(&Imsaak).unwrap().unwrap();
    assert_eq!(NaiveTime::from_hms_opt(5, 28, 0).unwrap(), imsaak.time);
    assert_eq!(false, imsaak.extreme);
    let fajr = pts.get(&Fajr).unwrap().unwrap();
    assert_eq!(NaiveTime::from_hms_opt(5, 34, 0).unwrap(), fajr.time);
    assert_eq!(false, fajr.extreme);
    let shurooq = pts.get(&Shurooq).unwrap().unwrap();
    assert_eq!(NaiveTime::from_hms_opt(6, 48, 0).unwrap(), shurooq.time);
    assert_eq!(false, shurooq.extreme);
    let dhuhr = pts.get(&Dhuhr).unwrap().unwrap();
    assert_eq!(NaiveTime::from_hms_opt(12, 34, 0).unwrap(), dhuhr.time);
    assert_eq!(false, dhuhr.extreme);
    let asr = pts.get(&Asr).unwrap().unwrap();
    assert_eq!(NaiveTime::from_hms_opt(15, 53, 0).unwrap(), asr.time);
    assert_eq!(false, asr.extreme);
    let maghrib = pts.get(&Maghrib).unwrap().unwrap();
    assert_eq!(NaiveTime::from_hms_opt(18, 20, 0).unwrap(), maghrib.time);
    assert_eq!(false, maghrib.extreme);
    let isha = pts.get(&Isha).unwrap().unwrap();
    assert_eq!(NaiveTime::from_hms_opt(19, 50, 0).unwrap(), isha.time);
    assert_eq!(false, isha.extreme);
}

#[test]
fn test_dubai_uae_default_params_fixed_isha() {
    use Prayer::*;

    // Arrange
    let params = Params::new(Method::FixedIsha);
    let latitude = Latitude::try_from(25.1649).unwrap();
    let longitude = Longitude::try_from(55.4084).unwrap();
    let elevation = Elevation::try_from(11.).unwrap();
    let coords = Coordinates::new(latitude, longitude, elevation);
    let gmt = Gmt::try_from(4.).unwrap();
    let location = Location { coords, gmt };
    let date = NaiveDate::from_ymd_opt(2023, 2, 19).unwrap();

    // Act
    let pts = prayer_times_dt(&params, location, date, None);

    // Assert
    let imsaak = pts.get(&Imsaak).unwrap().unwrap();
    assert_eq!(NaiveTime::from_hms_opt(5, 20, 0).unwrap(), imsaak.time);
    assert_eq!(false, imsaak.extreme);
    let fajr = pts.get(&Fajr).unwrap().unwrap();
    assert_eq!(NaiveTime::from_hms_opt(5, 27, 0).unwrap(), fajr.time);
    assert_eq!(false, fajr.extreme);
    let shurooq = pts.get(&Shurooq).unwrap().unwrap();
    assert_eq!(NaiveTime::from_hms_opt(6, 50, 0).unwrap(), shurooq.time);
    assert_eq!(false, shurooq.extreme);
    let dhuhr = pts.get(&Dhuhr).unwrap().unwrap();
    assert_eq!(NaiveTime::from_hms_opt(12, 32, 0).unwrap(), dhuhr.time);
    assert_eq!(false, dhuhr.extreme);
    let asr = pts.get(&Asr).unwrap().unwrap();
    assert_eq!(NaiveTime::from_hms_opt(15, 48, 0).unwrap(), asr.time);
    assert_eq!(false, asr.extreme);
    let maghrib = pts.get(&Maghrib).unwrap().unwrap();
    assert_eq!(NaiveTime::from_hms_opt(18, 15, 0).unwrap(), maghrib.time);
    assert_eq!(false, maghrib.extreme);
    let isha = pts.get(&Isha).unwrap().unwrap();
    assert_eq!(NaiveTime::from_hms_opt(19, 45, 0).unwrap(), isha.time);
    assert_eq!(false, isha.extreme);
}

#[test]
fn test_tehran_ir_default_params_shafi() {
    use Prayer::*;

    // Arrange
    let params = Params::new(Method::Shafi);
    let latitude = Latitude::try_from(35.7219).unwrap();
    let longitude = Longitude::try_from(51.3347).unwrap();
    let elevation = Elevation::try_from(900.).unwrap();
    let coords = Coordinates::new(latitude, longitude, elevation);
    let gmt = Gmt::try_from(3.5).unwrap();
    let location = Location { coords, gmt };
    let date = NaiveDate::from_ymd_opt(2023, 2, 19).unwrap();

    // Act
    let pts = prayer_times_dt(&params, location, date, None);

    // Assert
    let imsaak = pts.get(&Imsaak).unwrap().unwrap();
    assert_eq!(NaiveTime::from_hms_opt(5, 15, 0).unwrap(), imsaak.time);
    assert_eq!(false, imsaak.extreme);
    let fajr = pts.get(&Fajr).unwrap().unwrap();
    assert_eq!(NaiveTime::from_hms_opt(5, 22, 0).unwrap(), fajr.time);
    assert_eq!(false, fajr.extreme);
    let shurooq = pts.get(&Shurooq).unwrap().unwrap();
    assert_eq!(NaiveTime::from_hms_opt(6, 47, 0).unwrap(), shurooq.time);
    assert_eq!(false, shurooq.extreme);
    let dhuhr = pts.get(&Dhuhr).unwrap().unwrap();
    assert_eq!(NaiveTime::from_hms_opt(12, 18, 0).unwrap(), dhuhr.time);
    assert_eq!(false, dhuhr.extreme);
    let asr = pts.get(&Asr).unwrap().unwrap();
    assert_eq!(NaiveTime::from_hms_opt(15, 24, 0).unwrap(), asr.time);
    assert_eq!(false, asr.extreme);
    let maghrib = pts.get(&Maghrib).unwrap().unwrap();
    assert_eq!(NaiveTime::from_hms_opt(17, 50, 0).unwrap(), maghrib.time);
    assert_eq!(false, maghrib.extreme);
    let isha = pts.get(&Isha).unwrap().unwrap();
    assert_eq!(NaiveTime::from_hms_opt(19, 15, 0).unwrap(), isha.time);
    assert_eq!(false, isha.extreme);
}

#[test]
fn test_kabul_af_default_params_hanafi() {
    use Prayer::*;

    // Arrange
    let params = Params::new(Method::Hanafi);
    let latitude = Latitude::try_from(34.5553).unwrap();
    let longitude = Longitude::try_from(69.2075).unwrap();
    let elevation = Elevation::try_from(1790.).unwrap();
    let coords = Coordinates::new(latitude, longitude, elevation);
    let gmt = Gmt::try_from(4.5).unwrap();
    let location = Location { coords, gmt };
    let date = NaiveDate::from_ymd_opt(2023, 2, 19).unwrap();

    // Act
    let pts = prayer_times_dt(&params, location, date, None);

    // Assert
    let imsaak = pts.get(&Imsaak).unwrap().unwrap();
    assert_eq!(NaiveTime::from_hms_opt(5, 4, 0).unwrap(), imsaak.time);
    assert_eq!(false, imsaak.extreme);
    let fajr = pts.get(&Fajr).unwrap().unwrap();
    assert_eq!(NaiveTime::from_hms_opt(5, 11, 0).unwrap(), fajr.time);
    assert_eq!(false, fajr.extreme);
    let shurooq = pts.get(&Shurooq).unwrap().unwrap();
    assert_eq!(NaiveTime::from_hms_opt(6, 34, 0).unwrap(), shurooq.time);
    assert_eq!(false, shurooq.extreme);
    let dhuhr = pts.get(&Dhuhr).unwrap().unwrap();
    assert_eq!(NaiveTime::from_hms_opt(12, 7, 0).unwrap(), dhuhr.time);
    assert_eq!(false, dhuhr.extreme);
    let asr = pts.get(&Asr).unwrap().unwrap();
    assert_eq!(NaiveTime::from_hms_opt(16, 0, 0).unwrap(), asr.time);
    assert_eq!(false, asr.extreme);
    let maghrib = pts.get(&Maghrib).unwrap().unwrap();
    assert_eq!(NaiveTime::from_hms_opt(17, 40, 0).unwrap(), maghrib.time);
    assert_eq!(false, maghrib.extreme);
    let isha = pts.get(&Isha).unwrap().unwrap();
    assert_eq!(NaiveTime::from_hms_opt(19, 3, 0).unwrap(), isha.time);
    assert_eq!(false, isha.extreme);
}
