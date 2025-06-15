//! Author: AnormalDog (https://github.com/AnormalDog)
//! Copyright (c) 2025 AnormalDog
//! Licensed under the MIT License. See LICENSE file in the project root for full license information.
//!
//! Library

mod implementation;

use crate::implementation::{
  add_n_days, add_n_months, add_n_years, date_index, normalize_year, remove_n_days,
  remove_n_months, remove_n_years, validate, validate_raw, year_index,
};
use std::fmt;

#[derive(Clone, Copy, Debug)]
pub enum DateError {
  InvalidNewInput,
  OutOfLimits,
  InternalError,
}

impl fmt::Display for DateError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      DateError::InvalidNewInput => f.write_str("Invalid input introducing values to NEW"),
      DateError::OutOfLimits => {
        f.write_str("the date is outside of the limits (-9999 - 9999) years")
      }
      DateError::InternalError => f.write_str("internal error, found a weird number in index"),
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
  /// Create a new instance of Date.
  pub fn new(year: i32, month: u32, day: u32) -> Result<Date, DateError> {
    validate_raw(year, month, day)?;
    let year = normalize_year(year);
    let x = Date {
      year,
      index: year_index(year, month, day),
    };
    Ok(x)
  }

  /// Checks if the result is valid, then add n days.
  pub fn add_days(&mut self, days: u32) -> Result<&mut Self, DateError> {
    let mut aux = *self;
    add_n_days(&mut aux, days);
    validate(&aux)?;
    *self = aux;
    Ok(self)
  }

  /// Checks if the result is valid, then add n weeks.
  pub fn add_weeks(&mut self, weeks: u32) -> Result<&mut Self, DateError> {
    let mut aux = *self;
    add_n_days(&mut aux, weeks * 7);
    validate(&aux)?;
    *self = aux;
    Ok(self)
  }

  /// Checks if the result is valid, then add n months. More expensive than others.
  pub fn add_months(&mut self, months: u32) -> Result<&mut Self, DateError> {
    let mut aux = *self;
    add_n_months(&mut aux, months);
    validate(&aux)?;
    *self = aux;
    Ok(self)
  }

  /// Checks if the result is valid, then add n years.
  pub fn add_years(&mut self, years: u32) -> Result<&mut Self, DateError> {
    let mut aux = *self;
    add_n_years(&mut aux, years);
    validate(&aux)?;
    *self = aux;
    Ok(self)
  }

  /// Check if the result is valid, then remove n days.
  pub fn remove_days(&mut self, days: u32) -> Result<&mut Self, DateError> {
    let mut aux = *self;
    remove_n_days(&mut aux, days);
    validate(&aux)?;
    *self = aux;
    Ok(self)
  }

  /// Check if the result is valid, then remove n weeks.
  pub fn remove_weeks(&mut self, days: u32) -> Result<&mut Self, DateError> {
    let mut aux = *self;
    remove_n_days(&mut aux, days * 7);
    validate(&aux)?;
    *self = aux;
    Ok(self)
  }

  /// Checks if the result is valid, then remove n months. More expensive than others.
  pub fn remove_months(&mut self, months: u32) -> Result<&mut Self, DateError> {
    let mut aux = *self;
    remove_n_months(&mut aux, months);
    validate(&aux)?;
    *self = aux;
    Ok(self)
  }

  /// Checks if the result is valid, then remove n years.
  pub fn remove_years(&mut self, years: u32) -> Result<&mut Self, DateError> {
    let mut aux = *self;
    remove_n_years(&mut aux, years);
    validate(&aux)?;
    *self = aux;
    Ok(self)
  }

  /// Returns the number of days between two Dates. If date1 is after date2, the result will be negative
  pub fn days_between(date1: &Date, date2: &Date) -> i64 {
    date_index(date2) - date_index(date1)
  }
} // impl Date

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
  fn add_test() {
    let mut x = Date::new(2000, 2, 29).expect("error creating instance in test");
    let y = x;
    x.add_years(10000).expect_err("msg");
    assert_eq!(x, y);
  }

  #[test]
  fn multiple_operation_test() {
    let mut x = Date::new(2000, 2, 29).expect("error creating instance in test");
    let expected = x.clone();
    x.add_days(50)
      .expect("error adding")
      .add_weeks(5)
      .expect("error adding");
    x.remove_days(50)
      .expect("error removing")
      .remove_weeks(5)
      .expect("error removing");
    assert_eq!(x, expected);
  }

  #[test]
  fn days_between_test() {
    let x: Date = Date::new(2000, 1, 1).expect("error creating instance in test");
    let y = Date::new(2005, 3, 30).expect("error creating instance in test");
    assert_eq!(1915, Date::days_between(&x, &y));
    assert_eq!(-1915, Date::days_between(&y, &x));
  }
} // mod lib_test
