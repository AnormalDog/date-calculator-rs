//! Author: AnormalDog (https://github.com/AnormalDog)
//! Copyright (c) 2025 AnormalDog
//! Licensed under the MIT License. See LICENSE file in the project root for full license information.
//!
//! Internal implementation of the library

use std::error::Error;
use std::fmt;

const MAX_DAYS_YEAR: u64 = 366;

/// Returns true or false if the year is leap or not
pub fn is_year_leap(year: i64) -> bool {
  if year % 4 != 0 {
    return false;
  }
  if year % 100 != 0 {
    return true;
  }
  year % 400 == 0
}

/// Returns the number of leap gap between two years (both included)
fn leap_years_between(a: i64, b: i64) -> i64 {
  debug_assert!(a >= b);
  (a..=b).filter(|year| is_year_leap(*year)).count() as i64
}

/// Returns how many days have passed in the year
///   Basically converts Y/M/D to Y/D
pub fn get_year_index(year: i64, month: u8, day: u8) -> u64 {
  let days_in_months = (1..month)
    .map(|month| get_day_per_month(year, month))
    .sum::<u64>();
  days_in_months + day as u64
}

/// Returns a pair month/day knowing the index of the year
pub fn get_date_standard(year: i64, index: u64) -> (u8, u8) {
  assert!(index < MAX_DAYS_YEAR && index > 0);
  let mut total_number_of_days: u64 = index;
  let mut number_of_month: u64 = 1;
  while total_number_of_days > get_day_per_month(year, number_of_month as u8) {
    total_number_of_days -= get_day_per_month(year, number_of_month as u8);
    number_of_month += 1;
  }
  (number_of_month as u8, total_number_of_days as u8)
}

/// Returns the number of days a month has
fn get_day_per_month(year: i64, month: u8) -> u64 {
  assert!(month > 0 && month < 13);
  match month {
    1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
    2 if is_year_leap(year) => 29,
    2 => 28,
    _ => 30,
  }
}

/// returns the year, moving all negative years one number to right, being 1BC the 0, 2BC the -1...
#[inline]
pub fn normalize_year(year: i64) -> i64 {
  if year < 0 { year + 1 } else { year }
}

/// Returns ok or a string with what was wrong first
pub fn check_if_raw_date_is_ok(year: i64, month: u8, day: u8) -> Result<(), DateError> {
  if year == 0 {
    return Err(DateError::InvalidYear);
  }
  if month == 0 || month > 12 {
    return Err(DateError::InvalidMonth);
  }
  if day == 0 || u64::from(day) > get_day_per_month(year, month) {
    return Err(DateError::InvalidDay);
  }
  Ok(())
}

#[derive(Debug, Copy, Clone)]
pub enum DateError {
  InvalidYear,
  InvalidMonth,
  InvalidDay,
}

impl fmt::Display for DateError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match *self {
      DateError::InvalidYear => f.write_str("the year is invalid"),
      DateError::InvalidMonth => f.write_str("the month is invalid"),
      DateError::InvalidDay => f.write_str("the day is invalid"),
    }
  }
}

impl Error for DateError {}

#[cfg(test)]
mod impl_test {
  use crate::implementation::{self, get_date_standard, get_year_index};

  #[test]
  fn leap_years_between() {
    let n = implementation::leap_years_between(-44, 0);
    assert_eq!(n, 12);
  }

  #[test]
  fn get_year_index_test() {
    assert_eq!(get_year_index(2024, 12, 11), 346);
    assert_eq!(get_year_index(2024, 02, 29), 060);
    assert_eq!(get_year_index(2024, 07, 18), 200);
    assert_eq!(get_year_index(2024, 07, 01), 183);
  }

  #[test]
  fn get_date_standard_test() {
    assert_eq!((12, 11), get_date_standard(2024, 346));
    assert_eq!((02, 29), get_date_standard(2024, 060));
    assert_eq!((07, 18), get_date_standard(2024, 200));
    assert_eq!((07, 01), get_date_standard(2024, 183));
  }
}
