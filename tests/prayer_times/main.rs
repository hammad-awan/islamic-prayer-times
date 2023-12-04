use std::ops::Add;

use chrono::{Duration, NaiveDate};
use islamic_prayer_times::{
    prayer_times::{Pressure, Temperature, Weather},
    DateRange,
};

mod juneau_ak;
mod params;
mod potomac_md;
mod world;

#[test]
fn should_new_pressure() {
    // Arrange
    // Act
    let pressure_res = Pressure::try_from(1000.);
    // Assert
    assert!(pressure_res.is_ok());
}
#[test]
fn should_fail_new_pressure_when_less_than_min() {
    // Arrange
    // Act
    let pressure_res = Pressure::try_from(99.9);
    // Assert
    assert!(pressure_res.is_err());
}

#[test]
fn should_fail_new_pressure_when_more_than_max() {
    // Arrange
    // Act
    let pressure_res = Pressure::try_from(1050.1);
    // Assert
    assert!(pressure_res.is_err());
}

#[test]
fn test_f64_from_pressure() {
    // Arrange
    let pressure = Pressure::try_from(1010.).unwrap();
    // Act
    let result = f64::from(pressure);
    // Assert
    assert_eq!(1010., result);
}

#[test]
fn should_new_temp() {
    // Arrange
    // Act
    let temp_res = Temperature::try_from(20.);
    // Assert
    assert!(temp_res.is_ok());
}

#[test]
fn should_fail_new_temp_when_less_than_min() {
    // Arrange
    // Act
    let temp_res = Temperature::try_from(-90.1);
    // Assert
    assert!(temp_res.is_err());
}

#[test]
fn should_fail_new_temp_when_more_than_max() {
    // Arrange
    // Act
    let temp_res = Temperature::try_from(57.1);
    // Assert
    assert!(temp_res.is_err());
}

#[test]
fn test_default_weather() {
    // Arrange
    // Act
    let weather = Weather::default();
    // Assert
    assert_eq!(Pressure::try_from(1010.).unwrap(), weather.pressure);
    assert_eq!(Temperature::try_from(14.).unwrap(), weather.temperature);
}

#[test]
fn test_partition_date_range() {
    // Arrange
    let start_date = NaiveDate::from_ymd_opt(2023, 1, 1).unwrap();
    let end_date = NaiveDate::from_ymd_opt(2023, 12, 31).unwrap();
    let date_range = DateRange::from(start_date..=end_date);
    let count = 10;
    // Act
    let date_ranges = date_range.partition(count);
    // Assert
    assert_eq!(count, date_ranges.len());
    let block_size = 37;
    let mut next_start_date = start_date;
    for idx in 0..date_ranges.len() {
        let mut end_date = next_start_date.add(Duration::days(block_size - 1));
        if end_date > *date_range.end_date() {
            end_date = *date_range.end_date();
        }
        let next_date_range = DateRange::from(next_start_date..=end_date);
        assert_eq!(next_date_range, date_ranges[idx]);
        next_start_date = next_date_range.end_date().add(Duration::days(1));
    }
}
