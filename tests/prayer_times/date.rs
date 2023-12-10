use std::ops::Add;

use chrono::{Duration, NaiveDate};
use islamic_prayer_times::DateRange;

#[test]
fn test_partition_date_range() {
    // Arrange
    let start_date = NaiveDate::from_ymd_opt(2023, 1, 1).unwrap();
    let end_date = NaiveDate::from_ymd_opt(2023, 12, 31).unwrap();
    let date_range = DateRange::from(start_date..=end_date);
    let count = 10;
    // Act
    let date_ranges = date_range.partition(count);
    // Assert
    assert_eq!(count, date_ranges.len());
    let block_size = 37;
    let mut next_start_date = start_date;
    for idx in 0..date_ranges.len() {
        let mut end_date = next_start_date.add(Duration::days(block_size - 1));
        if end_date > *date_range.end_date() {
            end_date = *date_range.end_date();
        }
        let next_date_range = DateRange::from(next_start_date..=end_date);
        assert_eq!(next_date_range, date_ranges[idx]);
        next_start_date = next_date_range.end_date().add(Duration::days(1));
    }
}
