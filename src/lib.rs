//! Author: AnormalDog (https://github.com/AnormalDog)
//! Copyright (c) 2025 AnormalDog
//! Licensed under the MIT License. See LICENSE file in the project root for full license information.
//!
//! Library

mod implementation;

use crate::implementation::{add_n_days, normalize_year, validate, validate_raw, year_index};
use std::fmt;

#[derive(Clone, Copy, Debug)]
pub enum DateError {
  InvalidNewInput,
  OutOfLimits,
}

impl fmt::Display for DateError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      DateError::InvalidNewInput => f.write_str("Invalid input introducing values to NEW"),
      DateError::OutOfLimits => {
        f.write_str("the date is outside of the limits (-9999 - 9999) years")
      }
    }
  }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Date {
  year: i32,
  index: u32,
}

impl fmt::Display for Date {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.write_str("data")
  }
}

impl Date {
  /// Create a new instance of Date
  pub fn new(year: i32, month: u32, day: u32) -> Result<Date, DateError> {
    validate_raw(year, month, day)?;
    let year = normalize_year(year);
    let x = Date {
      year: year,
      index: year_index(year, month, day),
    };
    Ok(x)
  }

  /// Add days to an instance, then checks if still valid
  pub fn add_days(&mut self, days: u32) -> Result<&Self, DateError> {
    add_n_days(self, days);
    validate(self)?;
    Ok(self)
  }

  pub fn add_weeks(&mut self, weeks : u32) -> Result<&Self, DateError> {
    add_n_days(self, weeks * 7);
    validate(self)?;
    Ok(self)
  }
}

#[cfg(test)]
mod lib_test {
  use crate::Date;
  #[test]
  fn new_test() {
    let _w = Date::new(2000, 2, 29).expect("error in w");
    let _x = Date::new(2001, 2, 29).expect_err("expected error in x");
    let _y = Date::new(-1, 2, 28).expect("error in y");
    let _z = Date::new(0, 50, 15).expect_err("error expected in z");
  }

  #[test]
  fn add_days_test() {
    let mut y = Date::new(9999, 12, 31).expect("error creating the obj test");
    y.add_days(1).expect_err("no error?");
  }
}
