use std::fmt::Display;

use chrono::{Datelike, NaiveDate};

use crate::error::OutOfRangeError;

/// Hijri day of the week.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HijriDay {
    Ahad = 1,
    Ithnain,
    Thulatha,
    Arbiaa,
    Khamees,
    Jumaa,
    Sabt,
}

impl Display for HijriDay {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl TryFrom<u8> for HijriDay {
    type Error = OutOfRangeError<u8>;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        use HijriDay::*;

        match value {
            1 => Ok(Ahad),
            2 => Ok(Ithnain),
            3 => Ok(Thulatha),
            4 => Ok(Arbiaa),
            5 => Ok(Khamees),
            6 => Ok(Jumaa),
            7 => Ok(Sabt),
            _ => Err(OutOfRangeError(1..=7)),
        }
    }
}

/// Hijri month of the year.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HijriMonth {
    Muharram = 1,
    Safar,
    RabiaAwal,
    RabiaThani,
    JumadaAwal,
    JumadaThani,
    Rajab,
    Shaaban,
    Ramadan,
    Shawwal,
    DhulQiddah,
    DhulHijjah,
}

impl TryFrom<u8> for HijriMonth {
    type Error = OutOfRangeError<u8>;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        use HijriMonth::*;

        match value {
            1 => Ok(Muharram),
            2 => Ok(Safar),
            3 => Ok(RabiaAwal),
            4 => Ok(RabiaThani),
            5 => Ok(JumadaAwal),
            6 => Ok(JumadaThani),
            7 => Ok(Rajab),
            8 => Ok(Shaaban),
            9 => Ok(Ramadan),
            10 => Ok(Shawwal),
            11 => Ok(DhulQiddah),
            12 => Ok(DhulHijjah),
            _ => Err(OutOfRangeError(1..=12)),
        }
    }
}

impl Display for HijriMonth {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use HijriMonth::*;

        let default = format!("{:?}", self);
        let val = match self {
            RabiaAwal => "Rabia Awal",
            RabiaThani => "Rabia Thani",
            JumadaAwal => "Jumada Awal",
            JumadaThani => "Jumada Thani",
            DhulQiddah => "Dhul Qiddah",
            DhulHijjah => "Dhul Hijjah",
            _ => &default,
        };

        write!(f, "{}", val)
    }
}

/// A date in the Hijri calender.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HijriDate {
    date: NaiveDate,
    day: u8,
    month: u8,
    year: u32,
    pre_epoch: bool,
    weekday: u8,
}

impl HijriDate {
    const HIJRI_EPOCH: i32 = 227015;

    /// Returns the ISO 8601 calendar date without timezone.
    pub fn date(&self) -> NaiveDate {
        self.date
    }

    /// Returns the Hijri day of the month.
    pub fn day(&self) -> u8 {
        self.day
    }

    /// Returns the Hijri day of the week.
    pub fn day_of_week(&self) -> HijriDay {
        HijriDay::try_from(self.weekday).unwrap()
    }

    /// Hijri month of the year.
    pub fn month(&self) -> HijriMonth {
        HijriMonth::try_from(self.month).unwrap()
    }

    /// Returns the Hijri year.
    pub fn year(&self) -> u32 {
        self.year
    }

    /// Returns true when the Hijri date is before Hijra, false otherwise.
    pub fn pre_epoch(&self) -> bool {
        self.pre_epoch
    }

    fn greg_abs_date(date: NaiveDate) -> i32 {
        let y_1 = (date.year() - 1) as f64;
        (date.ordinal() as f64 + 365. * y_1 + (y_1 / 4.).floor() - (y_1 / 100.).floor()
            + (y_1 / 400.).floor()) as i32
    }

    fn hijri_year(greg_date: i32) -> i32 {
        let mut year: i32;
        if greg_date < Self::HIJRI_EPOCH {
            year = 0;
            while greg_date <= Self::hijri_abs_date(1, 1, year) {
                year -= 1;
            }
        } else {
            year = ((greg_date - Self::HIJRI_EPOCH - 1) as f64 / 355.).floor() as i32;
            while greg_date >= Self::hijri_abs_date(1, 1, year + 1) {
                year += 1;
            }
        }

        year
    }

    fn hijri_abs_date(day: u8, month: u8, year: i32) -> i32 {
        let day: f64 = day as f64;
        let month: f64 = month as f64;
        let year: f64 = year as f64;
        (day + 29. * (month - 1.)
            + (month / 2.).floor()
            + 354. * (year - 1.)
            + ((3. + 11. * year) / 30.).floor()
            + Self::HIJRI_EPOCH as f64
            - 1.) as i32
    }

    fn month_val(greg_date: i32, year: i32) -> u8 {
        let mut month = 1;
        while greg_date > Self::hijri_abs_date(Self::days_in_month(month, year), month, year) {
            month += 1;
        }

        month
    }

    fn days_in_month(month: u8, year: i32) -> u8 {
        if month % 2 != 1 && (month != 12 || !Self::is_hijri_leap_year(year)) {
            29
        } else {
            30
        }
    }

    fn is_hijri_leap_year(year: i32) -> bool {
        ((11 * year).abs() + 14) % 30 < 11
    }

    fn adj_pre_epoch(mut year: i32) -> (u32, bool) {
        let mut pre_epoch = false;
        if year <= 0 {
            pre_epoch = true;
            year = -(year - 1);
        }

        (year as u32, pre_epoch)
    }
}

impl From<NaiveDate> for HijriDate {
    fn from(value: NaiveDate) -> Self {
        let adj_date = if value.year() < 0 {
            NaiveDate::from_ymd_opt(value.year() + 1, value.month(), value.day()).unwrap()
        } else {
            value
        };
        let greg_date = Self::greg_abs_date(adj_date);
        let year = Self::hijri_year(greg_date);
        let month = Self::month_val(greg_date, year);
        let day = (greg_date - Self::hijri_abs_date(1, month, year) + 1) as u8;
        let (year, pre_epoch) = Self::adj_pre_epoch(year);
        let weekday = ((greg_date % 7).abs() + 1) as u8;
        Self {
            date: value,
            year,
            month,
            day,
            pre_epoch,
            weekday,
        }
    }
}

impl Display for HijriDate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}, {} {}, {} {}",
            self.day_of_week(),
            self.month(),
            self.day,
            self.year,
            if self.pre_epoch { "B.H." } else { "A.H." }
        )
    }
}
