use islamic_prayer_times::{
    geo::coordinates::Latitude,
    prayer_times::{
        params::{AsrShadowRatio, ExtremeLatitudeMethod, Method, Params, RoundSeconds},
        Prayer,
    },
};

#[test]
fn test_default() {
    use Prayer::*;

    // Arrange
    // Act
    let params = Params::default();
    // Assert
    assert_default(&params);
    assert_eq!(AsrShadowRatio::Shafi, params.asr_shadow_ratio);
    assert_eq!(15.0, params.angles[&Fajr]);
    assert_eq!(15.0, params.angles[&Isha]);
    assert_eq!(0., params.intervals[&Isha]);
    assert_eq!(AsrShadowRatio::Shafi, params.asr_shadow_ratio);
}

#[test]
fn test_method_none() {
    use Method::*;
    use Prayer::*;

    // Arrange
    // Act
    let params = Params::new(None);
    // Assert
    assert_default(&params);
    assert_eq!(0., params.angles[&Fajr]);
    assert_eq!(0., params.angles[&Isha]);
    assert_eq!(0., params.intervals[&Isha]);
    assert_eq!(AsrShadowRatio::Shafi, params.asr_shadow_ratio);
}

#[test]
fn test_method_egyptian() {
    use Method::*;
    use Prayer::*;

    // Arrange
    // Act
    let params = Params::new(Egyptian);
    // Assert
    assert_default(&params);
    assert_eq!(20., params.angles[&Fajr]);
    assert_eq!(18., params.angles[&Isha]);
    assert_eq!(0., params.intervals[&Isha]);
    assert_eq!(AsrShadowRatio::Shafi, params.asr_shadow_ratio);
}

#[test]
fn test_method_egypt() {
    use Method::*;
    use Prayer::*;

    // Arrange
    // Act
    let params = Params::new(Egypt);
    // Assert
    assert_default(&params);
    assert_eq!(19.5, params.angles[&Fajr]);
    assert_eq!(17.5, params.angles[&Isha]);
    assert_eq!(0., params.intervals[&Isha]);
    assert_eq!(AsrShadowRatio::Shafi, params.asr_shadow_ratio);
}

#[test]
fn test_method_shafi() {
    use Method::*;
    use Prayer::*;

    // Arrange
    // Act
    let params = Params::new(Shafi);
    // Assert
    assert_eq!(18., params.angles[&Fajr]);
    assert_eq!(18., params.angles[&Isha]);
    assert_eq!(0., params.intervals[&Isha]);
    assert_eq!(AsrShadowRatio::Shafi, params.asr_shadow_ratio);
}

#[test]
fn test_method_hanafi() {
    use Method::*;
    use Prayer::*;

    // Arrange
    // Act
    let params = Params::new(Hanafi);
    // Assert
    assert_default(&params);
    assert_eq!(18., params.angles[&Fajr]);
    assert_eq!(18., params.angles[&Isha]);
    assert_eq!(0., params.intervals[&Isha]);
    assert_eq!(AsrShadowRatio::Hanafi, params.asr_shadow_ratio);
}

#[test]
fn test_method_isna() {
    use Method::*;
    use Prayer::*;

    // Arrange
    // Act
    let params = Params::new(Isna);
    // Assert
    assert_default(&params);
    assert_eq!(15., params.angles[&Fajr]);
    assert_eq!(15., params.angles[&Isha]);
    assert_eq!(0., params.intervals[&Isha]);
    assert_eq!(AsrShadowRatio::Shafi, params.asr_shadow_ratio);
}

#[test]
fn test_method_mwl() {
    use Method::*;
    use Prayer::*;

    // Arrange
    // Act
    let params = Params::new(Mwl);
    // Assert
    assert_default(&params);
    assert_eq!(18., params.angles[&Fajr]);
    assert_eq!(17., params.angles[&Isha]);
    assert_eq!(0., params.intervals[&Isha]);
    assert_eq!(AsrShadowRatio::Shafi, params.asr_shadow_ratio);
}

#[test]
fn test_method_umm_al_qurra() {
    use Method::*;
    use Prayer::*;

    // Arrange
    // Act
    let params = Params::new(UmmAlQurra);
    // Assert
    assert_default(&params);
    assert_eq!(18., params.angles[&Fajr]);
    assert_eq!(0., params.angles[&Isha]);
    assert_eq!(90., params.intervals[&Isha]);
    assert_eq!(AsrShadowRatio::Shafi, params.asr_shadow_ratio);
}

#[test]
fn test_method_fixed_isha() {
    use Method::*;
    use Prayer::*;

    // Arrange
    // Act
    let params = Params::new(FixedIsha);
    // Assert
    assert_default(&params);
    assert_eq!(19.5, params.angles[&Fajr]);
    assert_eq!(0., params.angles[&Isha]);
    assert_eq!(90., params.intervals[&Isha]);
    assert_eq!(AsrShadowRatio::Shafi, params.asr_shadow_ratio);
}

fn assert_default(params: &Params) {
    use ExtremeLatitudeMethod::*;
    use Prayer::*;
    use RoundSeconds::*;

    assert_eq!(Params::DEF_IMSAAK_ANGLE, params.angles[&Imsaak]);
    assert_eq!(0., params.intervals[&Imsaak]);
    assert_eq!(0., params.intervals[&Fajr]);
    assert_eq!(0., params.minutes[&Imsaak]);
    assert_eq!(0., params.minutes[&Fajr]);
    assert_eq!(0., params.minutes[&Shurooq]);
    assert_eq!(0., params.minutes[&Dhuhr]);
    assert_eq!(0., params.minutes[&Asr]);
    assert_eq!(0., params.minutes[&Maghrib]);
    assert_eq!(0., params.minutes[&Isha]);
    assert_eq!(Latitude::new(48.5).unwrap(), params.nearest_latitude);
    assert_eq!(
        NearestGoodDayFajrIshaInvalid,
        params.extreme_latitude_method
    );
    assert_eq!(SpecialRounding, params.round_seconds);
}
