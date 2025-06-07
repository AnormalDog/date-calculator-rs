//! Author: AnormalDog (https://github.com/AnormalDog)
//! Copyright (c) 2025 AnormalDog
//! Licensed under the MIT License. See LICENSE file in the project root for full license information.
//!
//! Internal implementation of the library

/// Returns true or false if the year is leap or not
fn is_year_leap(year: i64) -> bool {
  if year % 4 != 0 {
    return false;
  }
  if year % 100 != 0 {
    return true;
  }
  if year % 400 == 0 { true } else { false }
}
/// Returns the number of leap gap between two years (both included)
fn leap_years_between(a: i64, b: i64) -> i64 {
  let mut n_of_leap_years: i64 = 0;
  for year in a..=b {
    if is_year_leap(year) == true {
      n_of_leap_years += 1;
    }
  }
  return n_of_leap_years;
}

pub fn get_year_index(year: i64, month: u8, day: u8) -> i64 {
  let mut number_of_days : i64 = 0;
  for n in 1..month {
    number_of_days += get_day_per_month(year, n);
  }
  number_of_days += i64::from(day);
  return number_of_days;
}

fn get_day_per_month(year: i64, month: u8) -> i64 {
  assert!(month > 0 && month < 13);
  match month {
    1 | 3 | 5 | 7 | 8 | 10 | 12 => return 31,
    2 => {
      if is_year_leap(year) == true {
        return 29;
      } else {
        return 28;
      }
    }
    _ => return 30,
  }
}

fn check_if_raw_date_is_ok(year : i64, month : u8, day : u8) -> bool {
  
  true
}



#[cfg(test)]
mod impl_test {
  use crate::implementation::{self, get_year_index};

  #[test]
  fn leap_years_between() {
    let n = implementation::leap_years_between(-44, 0);
    assert_eq!(n, 12);
  }

  #[test]
  fn get_year_index_test() {
    assert_eq!(get_year_index(2000, 01, 16), 16);
  }
}
