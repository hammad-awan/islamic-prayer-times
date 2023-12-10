use islamic_prayer_times::{Pressure, Temperature, Weather};

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
