//! Author: AnormalDog (https://github.com/AnormalDog)
//! Copyright (c) 2025 AnormalDog
//! Licensed under the MIT License. See LICENSE file in the project root for full license information.
//!
//! Internal implementation of the library

use crate::{Date, DateError};

/// Returns true is the year is leap
fn is_year_leap(year: i32) -> bool {
  if year % 4 != 0 {
    false
  } else if year % 100 != 0 {
    true
  } else {
    year % 400 == 0
  }
}

/// Returns the max number a month can hold
fn days_per_month(year: i32, month: u32) -> u32 {
  assert!(month > 0 && month <= 12);
  match month {
    1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
    2 if is_year_leap(year) => 29,
    2 if !is_year_leap(year) => 28,
    _ => 30,
  }
}

/// Returns the number of day a year has
fn days_per_year(year: i32) -> u32 {
  if is_year_leap(year) { 366 } else { 365 }
}

/// validates if the raw data is correct
pub fn validate_raw(year: i32, month: u32, day: u32) -> Result<(), DateError> {
  if year < -9999 || year > 9999 {
    Err(DateError::OutOfLimits)
  } else if (year == 0)
    || (month == 0)
    || (month > 12)
    || (day == 0)
    || (day > days_per_month(normalize_year(year), month))
  {
    Err(DateError::InvalidNewInput)
  } else {
    Ok(())
  }
}

/// Validate the current state of the instance
pub fn validate(date: &Date) -> Result<(), DateError> {
  assert!(date.index < days_per_year(date.year));
  if date.year > 9999 || date.year < -9998 {
    Err(DateError::OutOfLimits)
  } else {
    Ok(())
  }
}

/// Convert from Y/M/D -> Y/D
pub fn year_index(year: i32, month: u32, day: u32) -> u32 {
  (1..month).map(|x| days_per_month(year, x)).sum::<u32>() + day
}

/// Move all BC(negative) years one number to right, making the 1BC the 0, the 2BC the -1...
pub fn normalize_year(year: i32) -> i32 {
  assert_ne!(year, 0);
  assert!(year >= -9999 && year <= 9999);
  if year < 0 { year + 1 } else { year }
}

/// Add n days
pub fn add_n_days(date: &mut Date, n: u32) {
  date.index += n;
  while date.index > days_per_year(date.year) {
    date.index -= days_per_year(date.year);
    date.year += 1;
  }
}

#[cfg(test)]
mod implementation_test {
  use crate::{Date, DateError};
  use crate::implementation::{add_n_days, validate_raw};
  #[test]
  fn validate_raw_test() {
    validate_raw(10000, 5, 12).expect_err("correct error");
    validate_raw(2000, 2, 29).expect("correct no error");
    validate_raw(1, 2, 29).expect_err("correct error");
  }

  #[test]
  fn add_days_test() {
    let mut y = Date::new(2000, 2, 29).expect("error creating the obj test");
    add_n_days(&mut y, 1000);
    let z = Date::new(2002, 11, 25).expect("error creating the obj test");
    assert_eq!(y, z);
  }
}
