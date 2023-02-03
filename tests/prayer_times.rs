use chrono::{NaiveDate, NaiveTime};
use islamic_prayer_times::{
    geo::coordinates::{Coordinates, Elevation, Gmt, Latitude, Longitude},
    prayer_times::{
        params::{Method, Params},
        DateRange, Prayer, {get_prayer_times, Location},
    },
};

#[test]
fn test() {
    // Arrange
    let params = Params::new(Method::Isna);
    let latitude = Latitude::new(39.0181651).unwrap();
    let longitude = Longitude::new(-77.2085914).unwrap();
    let elevation = Elevation::new(110.).unwrap();
    let coords = Coordinates::new(latitude, longitude, elevation);
    let gmt = Gmt::new(-5.).unwrap();
    let location = Location { coords, gmt };
    let start_date = NaiveDate::from_ymd_opt(2011, 12, 10).unwrap();
    let middle_date = NaiveDate::from_ymd_opt(2011, 12, 11).unwrap();
    let end_date = NaiveDate::from_ymd_opt(2011, 12, 12).unwrap();
    let date_range = DateRange::new(start_date, end_date).unwrap();

    // Act
    let pt_by_day = get_prayer_times(&params, &location, date_range);
    // Assert

    assert_eq!(3, pt_by_day.len());
    assert_eq!(true, pt_by_day.contains_key(&start_date));
    assert_eq!(true, pt_by_day.contains_key(&middle_date));
    assert_eq!(true, pt_by_day.contains_key(&end_date));

    let pt = pt_by_day.get(&start_date).unwrap();
    let fajr = pt.get(&Prayer::Fajr).unwrap().unwrap();
    assert_eq!(NaiveTime::from_hms_opt(5, 57, 0).unwrap(), fajr.time);
    assert_eq!(false, fajr.extreme);
    let shurooq = pt.get(&Prayer::Shurooq).unwrap().unwrap();
    assert_eq!(NaiveTime::from_hms_opt(7, 9, 0).unwrap(), shurooq.time);
    assert_eq!(false, shurooq.extreme);
    let dhuhr = pt.get(&Prayer::Dhuhr).unwrap().unwrap();
    assert_eq!(NaiveTime::from_hms_opt(12, 2, 0).unwrap(), dhuhr.time);
    assert_eq!(false, dhuhr.extreme);
    let asr = pt.get(&Prayer::Asr).unwrap().unwrap();
    assert_eq!(NaiveTime::from_hms_opt(14, 29, 0).unwrap(), asr.time);
    assert_eq!(false, asr.extreme);
    let maghrib = pt.get(&Prayer::Maghrib).unwrap().unwrap();
    assert_eq!(NaiveTime::from_hms_opt(16, 53, 0).unwrap(), maghrib.time);
    assert_eq!(false, maghrib.extreme);
    let isha = pt.get(&Prayer::Isha).unwrap().unwrap();
    assert_eq!(NaiveTime::from_hms_opt(18, 6, 0).unwrap(), isha.time);
    assert_eq!(false, isha.extreme);
}
