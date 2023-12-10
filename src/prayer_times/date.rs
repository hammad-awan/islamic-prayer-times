//! Date utility types.
//!

use std::{
    fmt::Display,
    ops::{Add, RangeInclusive},
};

use chrono::{Duration, Local, NaiveDate};
use serde::{Deserialize, Serialize};

/// A simple date range.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DateRange(RangeInclusive<NaiveDate>);

impl DateRange {
    /// Returns the start date of the `DateRange`.
    pub fn start_date(&self) -> &NaiveDate {
        self.0.start()
    }

    /// Returns the end date of the `DateRange`.
    pub fn end_date(&self) -> &NaiveDate {
        self.0.end()
    }

    // Returns the number of days in the `DateRange`.
    pub fn num_days(&self) -> usize {
        let duration = *self.end_date() - *self.start_date();
        (duration.num_days() + 1) as usize
    }

    // Partitions the date range into `Vec` of `count` date ranges.
    pub fn partition(&self, count: usize) -> Vec<DateRange> {
        if count == 1 {
            vec![self.clone()]
        } else {
            let days = self.num_days();
            let block_size = (days as f64 / count as f64).ceil() as i64;
            let mut date_ranges = Vec::with_capacity(if days < count { days } else { count });
            let mut start_date_iter = *self.start_date();
            while start_date_iter <= *self.end_date() {
                let mut end_date = start_date_iter.add(Duration::days(block_size - 1));
                if end_date > *self.end_date() {
                    end_date = *self.end_date();
                }
                date_ranges.push(DateRange(start_date_iter..=end_date));
                start_date_iter = start_date_iter.add(Duration::days(block_size));
            }
            date_ranges
        }
    }
}

impl From<RangeInclusive<NaiveDate>> for DateRange {
    fn from(value: RangeInclusive<NaiveDate>) -> Self {
        Self(value)
    }
}

impl Display for DateRange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} - {}", self.0.start(), self.0.end())
    }
}

impl Default for DateRange {
    /// Returns the "default value" for the type with today as its start and end dates. [Read more](Default::default)
    fn default() -> Self {
        let today = Local::now().date_naive();
        Self(today..=today)
    }
}
