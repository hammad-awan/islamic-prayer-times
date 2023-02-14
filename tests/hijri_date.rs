use chrono::NaiveDate;
use islamic_prayer_times::hijri_date::{HijriDate, HijriDay, HijriMonth};

#[test]
fn test_saturday_august() {
    use HijriDay::*;
    use HijriMonth::*;
    // Arrange
    let date = NaiveDate::from_ymd_opt(2020, 8, 1).unwrap();
    // Act
    let hijri_date = HijriDate::from(date);
    // Assert
    assert_eq!("Sabt Dhul Hijjah 11, 1441 A.H.", hijri_date.to_string());
    assert_eq!(date, hijri_date.date());
    assert_eq!(11, hijri_date.day());
    assert_eq!(Sabt, hijri_date.day_of_week());
    assert_eq!(DhulHijjah, hijri_date.month());
    assert_eq!(1441, hijri_date.year());
    assert!(!hijri_date.pre_epoch());
}

#[test]
fn test_tuesday_december() {
    // Arrange
    let date = NaiveDate::from_ymd_opt(2020, 12, 1).unwrap();
    // Act
    let hijri_date = HijriDate::from(date);
    // Assert
    assert_eq!("Thulatha Rabia Thani 15, 1442 A.H.", hijri_date.to_string());
}

#[test]
fn test_friday_july() {
    // Arrange
    let date = NaiveDate::from_ymd_opt(2020, 7, 3).unwrap();
    // Act
    let hijri_date = HijriDate::from(date);
    // Assert
    assert_eq!("Jumaa Dhul Qiddah 12, 1441 A.H.", hijri_date.to_string());
}

#[test]
fn test_monday_march() {
    // Arrange
    let date = NaiveDate::from_ymd_opt(2020, 3, 2).unwrap();
    // Act
    let hijri_date = HijriDate::from(date);
    // Assert
    assert_eq!("Ithnain Rajab 7, 1441 A.H.", hijri_date.to_string());
}

#[test]
fn test_sunday_november() {
    // Arrange
    let date = NaiveDate::from_ymd_opt(2020, 11, 1).unwrap();
    // Act
    let hijri_date = HijriDate::from(date);
    // Assert
    assert_eq!("Ahad Rabia Awal 15, 1442 A.H.", hijri_date.to_string());
}

#[test]
fn test_thursday_october() {
    // Arrange
    let date = NaiveDate::from_ymd_opt(2020, 10, 1).unwrap();
    // Act
    let hijri_date = HijriDate::from(date);
    // Assert
    assert_eq!("Khamees Safar 13, 1442 A.H.", hijri_date.to_string());
}

#[test]
fn test_wednesday_may() {
    // Arrange
    let date = NaiveDate::from_ymd_opt(2020, 5, 6).unwrap();
    // Act
    let hijri_date = HijriDate::from(date);
    // Assert
    assert_eq!("Arbiaa Ramadan 13, 1441 A.H.", hijri_date.to_string());
}

#[test]
fn test_pre_epoch() {
    // Arrange
    let date = NaiveDate::from_ymd_opt(622, 6, 19).unwrap();
    // Act
    let hijri_date = HijriDate::from(date);
    // Assert
    assert_eq!("Arbiaa Dhul Qiddah 30, 1 B.H.", hijri_date.to_string());
    assert!(hijri_date.pre_epoch());
}
