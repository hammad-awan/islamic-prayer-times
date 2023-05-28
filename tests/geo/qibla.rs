use float_cmp::assert_approx_eq;
use islamic_prayer_times::geo::{coordinates::*, qibla::*};

#[test]
fn test() {
    // Arrange
    let coords = Coordinates::new(
        Latitude::try_from(39.0181651).unwrap(),
        Longitude::try_from(-77.2085914).unwrap(),
        Elevation::default(),
    );
    let qibla = Qibla::new(coords);
    // Assert
    assert_approx_eq!(f64, -56.43742554, qibla.degrees(), epsilon = 0.00000001);
    assert_eq!(Rotation::Cw, qibla.rotation());
    assert_eq!(coords, qibla.coords());
    assert_eq!("56.4° CW", qibla.to_string());
}
