use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Angle {
    degrees: f64,
    radians: f64,
    cos: f64,
    sin: f64,
    tan: f64,
}

impl Angle {
    pub fn from_degrees(degrees: f64) -> Angle {
        let radians = degrees.to_radians();
        Angle {
            degrees,
            radians,
            cos: radians.cos(),
            sin: radians.sin(),
            tan: radians.tan(),
        }
    }

    pub fn from_radians(radians: f64) -> Angle {
        Angle {
            radians,
            degrees: radians.to_degrees(),
            cos: radians.cos(),
            sin: radians.sin(),
            tan: radians.tan(),
        }
    }

    pub fn degrees(&self) -> f64 {
        self.degrees
    }

    pub fn radians(&self) -> f64 {
        self.radians
    }

    pub fn cos(&self) -> f64 {
        self.cos
    }

    pub fn sin(&self) -> f64 {
        self.sin
    }

    pub fn tan(&self) -> f64 {
        self.tan
    }
}

impl Add for Angle {
    type Output = Angle;

    fn add(self, rhs: Self) -> Self::Output {
        Angle::from_degrees(self.degrees + rhs.degrees)
    }
}

impl Sub for Angle {
    type Output = Angle;

    fn sub(self, rhs: Self) -> Self::Output {
        Angle::from_degrees(self.degrees - rhs.degrees)
    }
}

impl Mul for Angle {
    type Output = Angle;

    fn mul(self, rhs: Self) -> Self::Output {
        Angle::from_degrees(self.degrees * rhs.degrees)
    }
}

impl Div for Angle {
    type Output = Angle;

    fn div(self, rhs: Self) -> Self::Output {
        Angle::from_degrees(self.degrees / rhs.degrees)
    }
}

impl PartialOrd for Angle {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.degrees.partial_cmp(&other.degrees) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        self.radians.partial_cmp(&other.radians)
    }
}

impl From<f64> for Angle {
    fn from(value: f64) -> Self {
        Angle::from_degrees(value)
    }
}

pub trait CanFloor {
    fn floor(self) -> Self;
}

pub trait LimitAngle
where
    Self: Sized
        + Add<Output = Self>
        + Sub<Output = Self>
        + Mul<Output = Self>
        + Div<Output = Self>
        + PartialOrd
        + CanFloor
        + From<f64>
        + Copy,
{
    // Caps an angle between 0 and cap degrees.
    fn cap_angle(self, cap: Self) -> Self {
        let val = self / cap;
        let val = val - val.floor();
        if val > Self::from(0.) {
            cap * val
        } else if val < Self::from(0.) {
            cap - cap * val
        } else {
            val
        }
    }

    /// Caps angle between 0 and 360 degrees.
    fn cap_angle_360(self) -> Self {
        self.cap_angle(Self::from(360.))
    }

    /// Caps an angle between 0 and 180 degrees.
    fn cap_angle_180(self) -> Self {
        self.cap_angle(Self::from(180.))
    }

    // Caps an angle between 0 and cap degrees.
    fn cap_angle_1(self) -> Self {
        let val = self - self.floor();
        if val < Self::from(0.) {
            val + Self::from(1.)
        } else {
            val
        }
    }

    // Caps an angle between -180 and 180 degrees.
    fn cap_angle_between_180(self) -> Self {
        let val = self / Self::from(360.);
        let val = (val - val.floor()) * Self::from(360.);
        if val < Self::from(-180.) {
            val + Self::from(360.)
        } else if val > Self::from(180.) {
            val - Self::from(360.)
        } else {
            val
        }
    }
}

impl CanFloor for f64 {
    fn floor(self) -> Self {
        self.floor()
    }
}

impl CanFloor for Angle {
    fn floor(self) -> Self {
        Angle::from_degrees(self.degrees.floor())
    }
}

impl LimitAngle for f64 {}

impl LimitAngle for Angle {}

#[cfg(test)]
mod tests {
    use std::f64::consts::PI;

    use super::*;
    use float_cmp::assert_approx_eq;

    const EPSILON_TEST: f64 = 0.00000001;

    #[test]
    fn angle_from_degrees() {
        // Arrange
        // Act
        let result = Angle::from_degrees(180.);
        // Assert
        assert_approx_eq!(f64, PI, result.radians(), epsilon = EPSILON_TEST);
    }

    #[test]
    fn angle_from_radians() {
        // Arrange
        // Act
        let result = Angle::from_radians(PI);
        // Assert
        assert_approx_eq!(f64, 180., result.degrees(), epsilon = EPSILON_TEST);
    }

    #[test]
    fn should_cap_fractional_positive_angle_at_360() {
        // Arrange
        // Act
        let result = 723.2.cap_angle_360();
        // Assert
        assert_approx_eq!(f64, 3.2, result, epsilon = EPSILON_TEST);
    }

    #[test]
    fn should_cap_fractional_negative_angle_at_360() {
        // Arrange
        // Act
        let result = (-723.2).cap_angle_360();
        // Assert
        assert_approx_eq!(f64, 356.8, result, epsilon = EPSILON_TEST);
    }

    #[test]
    fn should_cap_integer_multiple_angle_at_360() {
        // Arrange
        // Act
        let result = 720.0.cap_angle_360();
        // Assert
        assert_approx_eq!(f64, 0., result, epsilon = EPSILON_TEST);
    }

    #[test]
    fn should_cap_fractional_positive_angle_at_180() {
        // Arrange
        // Act
        let result = 183.2.cap_angle_180();
        // Assert
        assert_approx_eq!(f64, 3.2, result, epsilon = EPSILON_TEST);
    }

    #[test]
    fn should_cap_fractional_negative_angle_at_180() {
        // Arrange
        // Act
        let result = (-183.2).cap_angle_180();
        // Assert
        assert_approx_eq!(f64, 176.8, result, epsilon = EPSILON_TEST);
    }

    #[test]
    fn should_cap_integer_multiple_angle_at_180() {
        // Arrange
        // Act
        let result = 360.0.cap_angle_180();
        // Assert
        assert_approx_eq!(f64, 0., result, epsilon = EPSILON_TEST);
    }

    #[test]
    fn should_cap_positive_angle_1() {
        // Arrange
        // Act
        let result = 0.1.cap_angle_1();
        // Assert
        assert_approx_eq!(f64, 0.1, result, epsilon = EPSILON_TEST);
    }

    #[test]
    fn should_cap_negative_angle_1() {
        // Arrange
        // Act
        let result = (-0.9).cap_angle_1();
        // Assert
        assert_approx_eq!(f64, 0.1, result, epsilon = EPSILON_TEST);
    }

    #[test]
    fn should_cap_between_180_when_angle_gt_180() {
        // Arrange
        // Act
        let result = 189.3.cap_angle_between_180();
        // Assert
        assert_approx_eq!(f64, -170.7, result, epsilon = EPSILON_TEST);
    }

    #[test]
    fn should_cap_between_180_when_angle_lt_180() {
        // Arrange
        // Act
        let result = 165.0.cap_angle_between_180();
        // Assert
        assert_approx_eq!(f64, 165., result, epsilon = EPSILON_TEST);
    }

    #[test]
    fn should_cap_between_180_when_angle_lt_negative_180() {
        // Arrange
        // Act
        let result = (-189.3).cap_angle_between_180();
        // Assert
        assert_approx_eq!(f64, 170.7, result, epsilon = EPSILON_TEST);
    }
}
