use std::ops::{Add, Div, Mul, Sub};

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

impl LimitAngle for f64 {}

#[cfg(test)]
mod tests {
    use super::*;
    use float_cmp::assert_approx_eq;

    const EPSILON_TEST: f64 = 0.00000001;

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
