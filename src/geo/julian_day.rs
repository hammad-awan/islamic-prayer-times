use std::ops::{Add, Sub};

use chrono::{Datelike, Days, NaiveDate};

use super::coordinates::Gmt;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct JulianDay {
    pub date: NaiveDate,
    pub gmt: Gmt,
    pub value: f64,
}

impl JulianDay {
    pub fn new(date: NaiveDate, gmt: Gmt) -> Self {
        let mut new_year = date.year() as f64;
        let mut new_month = date.month() as f64;
        if new_month <= 2. {
            new_year -= 1.;
            new_month += 12.;
        }

        if date.year() < 1 {
            new_year += 1.;
        }

        let mut b = 0.;
        if date.year() > 1582
            || date.year() == 1582 && (date.month() > 10 || date.month() == 10 && date.day() > 15)
        {
            let a = (new_year / 100.).floor();
            b = 2. - a + (a / 4.).floor();
        }

        let c = (365.25 * (new_year + 4716.)).floor();
        let d = (30.6001 * (new_month + 1.)).floor();
        let value = b + c + d + (date.day() as f64 - f64::from(gmt) / 24.) - 1524.5;

        Self { date, gmt, value }
    }

    pub fn sub(&self, days: u64) -> Self {
        Self {
            date: self.date.sub(Days::new(days)),
            gmt: self.gmt,
            value: self.value - days as f64,
        }
    }

    pub fn add(&self, days: u64) -> Self {
        JulianDay {
            date: self.date.add(Days::new(days)),
            gmt: self.gmt,
            value: self.value + days as f64,
        }
    }
}

impl From<JulianDay> for f64 {
    fn from(value: JulianDay) -> Self {
        value.value
    }
}

#[cfg(test)]
mod tests {
    use chrono::NaiveDate;
    use float_cmp::assert_approx_eq;

    use super::*;

    const EPSILON_TEST: f64 = 0.00000001;

    #[test]
    fn should_new() {
        // Arrange
        let date = NaiveDate::from_ymd_opt(2022, 12, 4).unwrap();
        // Act
        let julian_day = JulianDay::new(date, Gmt::try_from(-4.).unwrap());
        // Assert
        assert_approx_eq!(
            f64,
            2459917.66666667,
            julian_day.value,
            epsilon = EPSILON_TEST
        );
        assert_eq!(date, julian_day.date);
    }

    #[test]
    fn should_return_prev_julian_day() {
        // Arrange
        let date = NaiveDate::from_ymd_opt(2022, 12, 4).unwrap();
        let julian_day = JulianDay::new(date, Gmt::try_from(-4.).unwrap());
        // Act
        let prev_julian_day = julian_day.sub(1);
        // Assert
        assert_approx_eq!(
            f64,
            2459916.66666667,
            prev_julian_day.value,
            epsilon = EPSILON_TEST
        );
        assert_eq!(
            NaiveDate::from_ymd_opt(2022, 12, 3).unwrap(),
            prev_julian_day.date
        );
    }

    #[test]
    fn should_return_next_julian_day() {
        // Arrange
        let date = NaiveDate::from_ymd_opt(2022, 12, 4).unwrap();
        let julian_day = JulianDay::new(date, Gmt::try_from(-4.).unwrap());
        // Act
        let new_julian_day = julian_day.add(1);
        // Assert
        assert_approx_eq!(
            f64,
            2459918.66666667,
            new_julian_day.value,
            epsilon = EPSILON_TEST
        );
        assert_eq!(
            NaiveDate::from_ymd_opt(2022, 12, 5).unwrap(),
            new_julian_day.date
        );
    }
}
