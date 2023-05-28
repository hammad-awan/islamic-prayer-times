use islamic_prayer_times::geo::coordinates::*;

#[test]
fn should_new_positive_latitude() {
    // Arrange
    // Act
    let lat_res = Latitude::try_from(45.);
    // Assert
    assert!(lat_res.is_ok());
    let lat = lat_res.unwrap();
    assert_eq!(45., f64::from(lat));
    assert_eq!(Direction::North, lat.direction());
}

#[test]
fn should_new_negative_latitude() {
    // Arrange
    // Act
    let lat_res = Latitude::try_from(-45.);
    // Assert
    assert!(lat_res.is_ok());
    let lat = lat_res.unwrap();
    assert_eq!(-45., f64::from(lat));
    assert_eq!(Direction::South, lat.direction());
}

#[test]
fn should_fail_new_when_latitude_gt_90() {
    // Arrange
    // Act
    let lat_res = Latitude::try_from(90.3);
    // Assert
    assert!(lat_res.is_err());
}

#[test]
fn should_fail_new_when_latitude_lt_negative_90() {
    // Arrange
    // Act
    let lat_res = Latitude::try_from(-90.3);
    // Assert
    assert!(lat_res.is_err());
}

#[test]
fn should_new_positive_longitude() {
    // Arrange
    // Act
    let lon_res = Longitude::try_from(120.);
    // Assert
    assert!(lon_res.is_ok());
    let lon = lon_res.unwrap();
    assert_eq!(120., f64::from(lon));
    assert_eq!(Direction::East, lon.direction());
}

#[test]
fn should_new_negative_longitude() {
    // Arrange
    // Act
    let lon_res = Longitude::try_from(-120.);
    // Assert
    assert!(lon_res.is_ok());
    let lon = lon_res.unwrap();
    assert_eq!(-120., f64::from(lon));
    assert_eq!(Direction::West, lon.direction());
}

#[test]
fn should_fail_new_when_longitude_gt_180() {
    // Arrange
    // Act
    let lon_res = Longitude::try_from(180.3);
    // Assert
    assert!(lon_res.is_err());
}

#[test]
fn should_fail_new_when_longitude_lt_negative_180() {
    // Arrange
    // Act
    let lon_res = Longitude::try_from(-180.3);
    // Assert
    assert!(lon_res.is_err());
}

#[test]
fn should_new_positive_elevation() {
    // Arrange
    // Act
    let elev = 1000.;
    let elev_res = Elevation::try_from(elev);
    // Assert
    assert!(elev_res.is_ok());
    assert_eq!(elev, f64::from(elev_res.unwrap()));
}

#[test]
fn should_new_negative_elevation() {
    // Arrange
    // Act
    let elev = -100.;
    let elev_res = Elevation::try_from(elev);
    // Assert
    assert!(elev_res.is_ok());
    assert_eq!(elev, f64::from(elev_res.unwrap()));
}

#[test]
fn should_fail_new_when_elevation_too_high() {
    // Arrange
    // Act
    let elev = 10000.;
    let elev_res = Elevation::try_from(elev);
    // Assert
    assert!(elev_res.is_err());
}

#[test]
fn should_fail_new_when_elevation_too_low() {
    // Arrange
    // Act
    let elev = -10000.;
    let elev_res = Elevation::try_from(elev);
    // Assert
    assert!(elev_res.is_err());
}

#[test]
fn should_new_coordinates() {
    // Arrange
    // Act
    let result = Coordinates::new(
        Latitude::try_from(77.3).unwrap(),
        Longitude::try_from(165.9).unwrap(),
        Elevation::try_from(1010.32).unwrap(),
    );
    // Assert
    assert_eq!(Latitude::try_from(77.3).unwrap(), result.latitude);
    assert_eq!(Longitude::try_from(165.9).unwrap(), result.longitude);
    assert_eq!(Elevation::try_from(1010.32).unwrap(), result.elevation);
}

#[test]
fn should_print_coordinates() {
    // Arrange
    // Act
    let result1 = Coordinates::new(
        Latitude::try_from(77.3).unwrap(),
        Longitude::try_from(165.9).unwrap(),
        Elevation::try_from(1010.32).unwrap(),
    );
    let result2 = Coordinates::new(
        Latitude::try_from(-77.3).unwrap(),
        Longitude::try_from(-165.9).unwrap(),
        Elevation::try_from(1010.32).unwrap(),
    );
    // Assert
    assert_eq!("77 N, 166 E, 1010 meters", result1.to_string());
    assert_eq!("77 S, 166 W, 1010 meters", result2.to_string());
}
