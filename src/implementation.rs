//! Author: AnormalDog (https://github.com/AnormalDog)
//! Copyright (c) 2025 AnormalDog
//! Licensed under the MIT License. See LICENSE file in the project root for full license information.
//!
//! Internal implementation of the library

use crate::{Date, DateError};

const REFERENCE_DATE: Date = Date { year: 1, index: 1 };

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
  if !(-9999..=9999).contains(&year) {
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
  if date.index > days_per_year(date.year) {
    Err(DateError::InternalError)
  } else if date.year > 9999 || date.year < -9998 {
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
  assert!((-9999..=9999).contains(&year));
  if year < 0 { year + 1 } else { year }
}

/// Reverse operation to normalize_year
#[allow(dead_code)]
pub fn standarize_year(year: i32) -> i32 {
  if year <= 0 { year - 1 } else { year }
}

/// Add n days
pub fn add_n_days(date: &mut Date, n: u32) {
  date.index += n;
  while date.index > days_per_year(date.year) {
    date.index -= days_per_year(date.year);
    date.year += 1;
  }
  assert!((1..=days_per_year(date.year)).contains(&date.index));
}

/// Add n months. Requires to standarize the Date, so is more expensive.
pub fn add_n_months(date: &mut Date, n: u32) {
  let mut standard = standarized_months_day(date);
  standard.0 += n;
  while standard.0 > 12 {
    date.year += 1;
    standard.0 -= 12;
  }
  assert!((1..=12).contains(&standard.0));
  if standard.1 > days_per_month(date.year, standard.0) {
    standard.1 = days_per_month(date.year, standard.0);
  }
  date.index = year_index(date.year, standard.0, standard.1);
}

/// Add n years
pub fn add_n_years(date: &mut Date, n: u32) {
  date.year += n as i32;
  if date.index > days_per_year(date.year) {
    date.index = days_per_year(date.year);
  }
}

/// Converts from D -> M/D
fn standarized_months_day(date: &Date) -> (u32, u32) {
  assert!(date.index <= days_per_year(date.year));
  let mut index_aux: u32 = date.index;
  let mut month_index: u32 = 1;
  while index_aux > days_per_month(date.year, month_index) {
    index_aux -= days_per_month(date.year, month_index);
    month_index += 1;
  }
  (month_index, index_aux)
}

/// Remove n days
pub fn remove_n_days(date: &mut Date, n: u32) {
  let mut substraction: i64 = date.index as i64 - n as i64;
  while substraction < 1 {
    date.year -= 1;
    substraction += days_per_year(date.year) as i64;
  }
  assert!((1..=days_per_year(date.year)).contains(&(substraction as u32)));
  date.index = substraction.unsigned_abs() as u32;
}

/// Remove n months. Requires to standarize the Date, so is more expensive
pub fn remove_n_months(date: &mut Date, n: u32) {
  let standard = standarized_months_day(date);
  let mut months: i64 = standard.0 as i64;
  let mut days: u32 = standard.1;
  months -= n as i64;
  while months < 1 {
    date.year -= 1;
    months += 12;
  }
  assert!((1..=12).contains(&months));
  if days > days_per_month(date.year, months as u32) {
    days = days_per_month(date.year, months as u32);
  }
  date.index = year_index(date.year, months as u32, days);
}

/// Remove n years.
pub fn remove_n_years(date: &mut Date, n: u32) {
  date.year -= n as i32;
  if date.index > days_per_year(date.year) {
    date.index = days_per_year(date.year);
  }
}

/// Returns the number of days that have happened since the REFERENCE_DATE
pub fn date_index(date: &Date) -> i64 {
  let mut sum: u64;
  if date.year > 0 {
    sum = (REFERENCE_DATE.year..date.year)
      .map(|x| days_per_year(x) as u64)
      .sum();
  } else {
    sum = (date.year..REFERENCE_DATE.year)
      .map(|x| days_per_year(x) as u64)
      .sum();
  }
  sum += (date.index - REFERENCE_DATE.index) as u64;
  if is_date_before(date, &REFERENCE_DATE) {
    -(sum as i64)
  } else {
    sum as i64
  }
}

/// Returns true if date1 is before date2
fn is_date_before(date1: &Date, date2: &Date) -> bool {
  if date1.year < date2.year {
    true
  } else if date1.year > date2.year {
    false
  } else {
    date1.index < date2.index
  }
}

/// Month offset required in gauss_algorithm
const MONTH_OFFSET: [i32; 12] = [0, 3, 3, 6, 1, 4, 6, 2, 5, 0, 3, 5];
const MONTH_OFFSET_LEAP: [i32; 12] = [0, 3, 4, 0, 2, 5, 0, 3, 6, 1, 4, 6];

/// Due in gauss algorithm weekday starts with 0 == sunday, this fn normalize that.
fn gauss_algorithm_weekday_normalizer(weekday: u8) -> u8 {
  assert!(weekday < 7);
  match weekday {
    0 => 6, // sunday
    1 => 0, // monday
    2 => 1, // tuesday
    3 => 2, // weknesday
    4 => 3, // thursday
    5 => 4, // friday
    _ => 5, // saturday
  }
}

/// Does the gauss algorithm to find the weekday. Can operate with BC years. 
/// See: https://en.wikipedia.org/wiki/Determination_of_the_day_of_the_week
pub fn gauss_algorithm(date: &Date) -> u8 {
  assert!(date.year > 0);
  let month_day = standarized_months_day(date);
  let index = (month_day.0 - 1) as usize;
  let offset = {
    if is_year_leap(date.year) {
      MONTH_OFFSET_LEAP[index]
    } else {
      MONTH_OFFSET[index]
    }
  };
  let d: i32 = (month_day.1 as i32
    + offset
    + 5 * ((date.year - 1) % 4)
    + 4 * ((date.year - 1) % 100)
    + 6 * ((date.year - 1) % 400))
    % 7;
  gauss_algorithm_weekday_normalizer(d as u8)
}

#[cfg(test)]
mod implementation_test {
  use crate::Date;
  use crate::implementation::{
    add_n_days, add_n_months, add_n_years, date_index, gauss_algorithm, remove_n_days,
    remove_n_months, remove_n_years, standarized_months_day, validate_raw,
  };
  #[test]
  fn validate_raw_test() {
    validate_raw(10000, 5, 12).expect_err("correct error");
    validate_raw(2000, 2, 29).expect("correct no error");
    validate_raw(1, 2, 29).expect_err("correct error");
  }

  #[test]
  fn standarized_months_day_test() {
    let x = Date::new(2000, 12, 31).expect("error creating instance in test");
    assert_eq!(standarized_months_day(&x), (12, 31));
  }

  #[test]
  fn add_days_test() {
    let mut y = Date::new(2000, 2, 29).expect("error creating instance in test");
    add_n_days(&mut y, 1000);
    let z = Date::new(2002, 11, 25).expect("error creating instance in test");
    assert_eq!(y, z);
  }

  #[test]
  fn add_n_months_test() {
    let mut x = Date::new(2000, 2, 20).expect("error creating instance in test");
    let expected = Date::new(2001, 5, 20).expect("error creating instance in test");
    add_n_months(&mut x, 15);
    assert_eq!(x, expected);
  }

  #[test]
  fn add_n_years_test() {
    let mut x = Date::new(2000, 12, 31).expect("error creating instance in test");
    let expected = Date::new(2002, 12, 31).expect("error creating instance in test");
    add_n_years(&mut x, 2);
    assert_eq!(x, expected);
  }

  #[test]
  fn remove_n_days_test() {
    let mut x = Date::new(2000, 12, 31).expect("error creating instance in test");
    let expected = Date::new(1999, 12, 31).expect("error creating instance in test");
    remove_n_days(&mut x, 366);
    assert_eq!(x, expected);
  }

  #[test]
  fn remove_n_months_test() {
    let mut x = Date::new(2000, 12, 31).expect("error creating instance in test");
    let expected = Date::new(1984, 4, 30).expect("error creating instance in test");
    remove_n_months(&mut x, 200);
    assert_eq!(x, expected);
  }

  #[test]
  fn remove_years_test() {
    let mut x = Date::new(2000, 12, 31).expect("error creating instance in test");
    let expected = Date::new(1801, 12, 31).expect("error creating instance in test");
    remove_n_years(&mut x, 199);
    assert_eq!(x, expected);
  }

  #[test]
  fn date_index_test() {
    assert_eq!(
      -184813,
      date_index(&Date::new(-506, 1, 1).expect("error creating instance in test"))
    );
    assert_eq!(
      0,
      date_index(&Date::new(1, 1, 1).expect("error creating instance in test"))
    );
    assert_eq!(
      365026,
      date_index(&Date::new(1000, 5, 30).expect("error creating instance in test"))
    );
    assert_eq!(
      1310776,
      date_index(&Date::new(3589, 10, 14).expect("error creating instance in test"))
    );
  }

  #[test]
  fn gauss_algorithm_test() {
    let w = Date::new(2000, 1, 1).expect("error creating instance in test");
    let x = Date::new(1856, 1, 31).expect("error creating instance in test");
    let y = Date::new(1000, 7, 26).expect("error creating instance in test");
    let z = Date::new(1, 5, 15).expect("error creating instance in test");
    assert_eq!(gauss_algorithm(&w), 5);
    assert_eq!(gauss_algorithm(&x), 3);
    assert_eq!(gauss_algorithm(&y), 5);
    assert_eq!(gauss_algorithm(&z), 1);
  }
} // mod implementation_test
