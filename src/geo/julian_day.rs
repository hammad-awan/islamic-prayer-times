use std::ops::{Add, Sub};

use chrono::{Datelike, Days, NaiveDate};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Gmt(i8);

impl Gmt {
    pub fn new(gmt: i8) -> Result<Gmt, ()> {
        if gmt < -12 || gmt > 12 {
            return Err(());
        }

        Ok(Gmt(gmt))
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct JulianDay {
    date: NaiveDate,
    gmt: Gmt,
    value: f64,
}

impl JulianDay {
    pub fn new(date: NaiveDate, gmt: Gmt) -> JulianDay {
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
        let value = b + c + d + (date.day() as f64 - gmt.0 as f64 / 24.) - 1524.5;

        JulianDay { date, gmt, value }
    }

    pub fn prev_day(&self) -> JulianDay {
        JulianDay {
            date: self.date.sub(Days::new(1)),
            gmt: self.gmt,
            value: self.value - 1.,
        }
    }

    pub fn next_day(&self) -> JulianDay {
        JulianDay {
            date: self.date.add(Days::new(1)),
            gmt: self.gmt,
            value: self.value + 1.,
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
        let julian_day = JulianDay::new(date, Gmt::new(-4).unwrap());
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
        let julian_day = JulianDay::new(date, Gmt::new(-4).unwrap());
        // Act
        let prev_julian_day = julian_day.prev_day();
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
        let julian_day = JulianDay::new(date, Gmt::new(-4).unwrap());
        // Act
        let new_julian_day = julian_day.next_day();
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
