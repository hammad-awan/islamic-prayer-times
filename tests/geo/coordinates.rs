use islamic_prayer_times::geo::coordinates::*;

#[test]
fn should_new_positive_latitude() {
    // Arrange
    // Act
    let result = Latitude::new(45.);
    // Assert
    assert!(result.is_ok());
    assert_eq!(Direction::North, result.unwrap().direction());
}

#[test]
fn should_new_negative_latitude() {
    // Arrange
    // Act
    let result = Latitude::new(-45.);
    // Assert
    assert!(result.is_ok());
    assert_eq!(Direction::South, result.unwrap().direction());
}

#[test]
fn should_fail_new_when_latitude_gt_90() {
    // Arrange
    // Act
    let result = Latitude::new(90.3);
    // Assert
    assert!(result.is_err());
}

#[test]
fn should_fail_new_when_latitude_lt_negative_90() {
    // Arrange
    // Act
    let result = Latitude::new(-90.3);
    // Assert
    assert!(result.is_err());
}

#[test]
fn should_new_positive_longitude() {
    // Arrange
    // Act
    let result = Longitude::new(120.);
    // Assert
    assert!(result.is_ok());
    assert_eq!(Direction::East, result.unwrap().direction());
}

#[test]
fn should_new_negative_longitude() {
    // Arrange
    // Act
    let result = Longitude::new(-120.);
    // Assert
    assert!(result.is_ok());
    assert_eq!(Direction::West, result.unwrap().direction());
}

#[test]
fn should_fail_new_when_longitude_gt_180() {
    // Arrange
    // Act
    let result = Longitude::new(180.3);
    // Assert
    assert!(result.is_err());
}

#[test]
fn should_fail_new_when_longitude_lt_negative_180() {
    // Arrange
    // Act
    let result = Longitude::new(-180.3);
    // Assert
    assert!(result.is_err());
}

#[test]
fn should_new_positive_elevation() {
    // Arrange
    // Act
    let result = Elevation::new(Elevation::MAX - 1.);
    // Assert
    assert!(result.is_ok());
}

#[test]
fn should_new_negative_elevation() {
    // Arrange
    // Act
    let result = Elevation::new(Elevation::MIN + 1.);
    // Assert
    assert!(result.is_ok());
}

#[test]
fn should_fail_new_when_elevation_too_high() {
    // Arrange
    // Act
    let result = Elevation::new(Elevation::MAX + 1.);
    // Assert
    assert!(result.is_err());
}

#[test]
fn should_fail_new_when_elevation_too_low() {
    // Arrange
    // Act
    let result = Elevation::new(Elevation::MIN - 1.);
    // Assert
    assert!(result.is_err());
}

#[test]
fn should_new_coordinates() {
    // Arrange
    // Act
    let result = Coordinates::new(
        Latitude::new(77.3).unwrap(),
        Longitude::new(165.9).unwrap(),
        Elevation::new(1010.32).unwrap(),
    );
    // Assert
    assert_eq!(Latitude::new(77.3).unwrap(), result.latitude);
    assert_eq!(Longitude::new(165.9).unwrap(), result.longitude);
    assert_eq!(Elevation::new(1010.32).unwrap(), result.elevation);
}

#[test]
fn should_print_coordinates() {
    // Arrange
    // Act
    let result1 = Coordinates::new(
        Latitude::new(77.3).unwrap(),
        Longitude::new(165.9).unwrap(),
        Elevation::new(1010.32).unwrap(),
    );
    let result2 = Coordinates::new(
        Latitude::new(-77.3).unwrap(),
        Longitude::new(-165.9).unwrap(),
        Elevation::new(1010.32).unwrap(),
    );
    // Assert
    assert_eq!("77 N, 166 E, 1010 meters", result1.to_string());
    assert_eq!("77 S, 166 W, 1010 meters", result2.to_string());
}
