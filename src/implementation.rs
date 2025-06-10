//! Author: AnormalDog (https://github.com/AnormalDog)
//! Copyright (c) 2025 AnormalDog
//! Licensed under the MIT License. See LICENSE file in the project root for full license information.
//!
//! Internal implementation of the library

use crate::{Date, DateError};

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

/// Returns how many days have passed in the year
///   Basically converts Y/M/D to Y/D
pub fn get_year_index(year: i64, month: u8, day: u8) -> u64 {
  let days: u64 = (1..month).map(|n| get_day_per_month(year, n)).sum();
  days + day as u64
}

/// Returns a pair month/day knowing the index of the year
pub fn get_date_standard(year: i64, index: u64) -> (u8, u8) {
  assert!(index < max_days_year(year) && index > 0);
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
  // check year
  if (year == 0)
    || (month == 0 || month > 12)
    || (day == 0 || u64::from(day) > get_day_per_month(year, month))
  {
    Err(DateError::ErrorWrongRawData)
  } else {
    Ok(())
  }
}

/// Returns the max days a year can hold
fn max_days_year(year: i64) -> u64 {
  if is_year_leap(year) { 366 } else { 365 }
}

/// modify a Date struct adding n ammount of days
pub fn add_n_days(date: &mut Date, n: u32) {
  let mut aux_sum = date.remain + (n as u64);
  while aux_sum > max_days_year(date.year) {
    aux_sum -= max_days_year(date.year);
    date.year += 1;
  }
  date.remain = aux_sum;
}


pub fn add_n_month(date: &mut Date, n: u32) {
  let mut aux = get_date_standard(date.year, date.remain);
  let mut month_aux = aux.0 as u32;
  month_aux += n;
  if month_aux > 12 {
    date.year += (month_aux / 12) as i64;
    month_aux %= 12;
  }
  aux.0 = month_aux as u8;
  if aux.1 > get_day_per_month(date.year, aux.0) as u8 {
    aux.1 = get_day_per_month(date.year, aux.0) as u8
  }
  date.remain = get_year_index(date.year, aux.0 as u8, aux.1);
}

#[cfg(test)]
mod impl_test {
  use crate::{
    implementation::{add_n_days, add_n_month, get_date_standard, get_year_index}, Date
  };

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

  #[test]
  fn add_n_days_test() {
    let mut x = Date::new(2000, 02, 28).unwrap();
    add_n_days(&mut x, 10000);
    let y = Date::new(2027, 07, 16).unwrap();
    assert_eq!(x.year, y.year);
    assert_eq!(x.remain, y.remain);
  }

  #[test]
  fn add_n_month_test() {
    let mut x = Date::new(2000, 02, 28).unwrap();
    add_n_month(&mut x, 50);
    let y = Date::new(2004, 04, 28).unwrap();
    assert_eq!(x.year, y.year);
    assert_eq!(x.remain, y.remain);
  }
}
