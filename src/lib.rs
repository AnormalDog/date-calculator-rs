//! Author: AnormalDog (https://github.com/AnormalDog)
//! Copyright (c) 2025 AnormalDog
//! Licensed under the MIT License. See LICENSE file in the project root for full license information.
//!
//! Library

use crate::implementation::{
  check_if_raw_date_is_ok, get_date_standard, get_year_index, is_year_leap, normalize_year,
};

use std::fmt;

mod implementation;

pub struct Date {
  year: i64,
  remain: u64,
}

#[derive(Debug, Clone, Copy)]
pub enum DateError {
  ErrorWrongRawData
}

impl fmt::Display for DateError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      match *self {
          DateError::ErrorWrongRawData => f.write_str("The date introduced is wrong"),
      }
  }
}

impl Date {
  /// Creates a new instance of Date
  pub fn new(year: i64, month: u8, day: u8) -> Result<Self, DateError> {
    check_if_raw_date_is_ok(year, month, day)?;
    let x = Date {
      year: normalize_year(year),
      remain: get_year_index(year, month, day),
    };
    return Ok(x);
  }

  /// Check if the date's year is leap
  pub fn is_leap(&self) -> bool {
    return is_year_leap(self.year);
  }

  /// add n days to a Date
  pub fn get_simple(&self) -> String {
    let x = get_date_standard(self.year, self.remain);
    return format!("{} - {:0>2} - {:0>2}", self.year, x.0, x.1);
  }
}

#[cfg(test)]
mod lib_test {
  use crate::Date;

  #[test]
  fn new_test() {
    Date::new(2024, 2, 29).unwrap();
  }

  #[test]
  fn leap_test() {
    assert_eq!(Date::is_leap(&Date::new(2000, 1, 1).unwrap()), true);
  }

  #[test]
  fn get_simple_test() {
    assert_eq!(
      String::from("2024 - 04 - 04"),
      Date::get_simple(&Date::new(2024, 04, 04).unwrap())
    );
  }
}
