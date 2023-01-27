use float_cmp::assert_approx_eq;
use islamic_prayer_times::geo::{coordinates::*, qibla::*};

#[test]
fn test() {
    // Arrange
    let coords = Coordinates::new(
        Latitude::new(39.0181651).unwrap(),
        Longitude::new(-77.2085914).unwrap(),
        Elevation::new(0.).unwrap(),
    );
    let qibla = Qibla::new(coords);
    // Assert
    assert_approx_eq!(f64, -56.43742554, qibla.angle(), epsilon = 0.00000001);
    assert_eq!(Rotation::Cw, qibla.rotation());
    assert_eq!(coords, qibla.coords());
    assert_eq!("56.4° CW", qibla.to_string());
}
