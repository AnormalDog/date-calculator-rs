//! Author: AnormalDog (https://github.com/AnormalDog)
//! Copyright (c) 2025 AnormalDog
//! Licensed under the MIT License. See LICENSE file in the project root for full license information.
//!
//! Library

mod implementation;

use crate::implementation::{
  add_n_days, add_n_months, add_n_years, date_index, gauss_algorithm, normalize_year,
  remove_n_days, remove_n_months, remove_n_years, validate, validate_raw, year_index,
};
use std::fmt;

#[derive(Clone, Copy, Debug)]
pub enum DateError {
  InvalidNewInput,
  OutOfLimits,
  InternalError,
  GregorianLimits,
}

impl fmt::Display for DateError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      DateError::InvalidNewInput => f.write_str("Invalid input introducing values to NEW"),
      DateError::OutOfLimits => {
        f.write_str("the date is outside of the limits (-9999 - 9999) years")
      }
      DateError::InternalError => f.write_str("internal error, found a weird number in index"),
      DateError::GregorianLimits => {
        f.write_str("Dude to gregorian limitations, this operation is not possible")
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

  /// Check if the result is valid, then adds n years, m months and x days to a date, in that order
  pub fn add(&mut self, year: u32, months: u32, days: u32) -> Result<&mut Self, DateError> {
    let mut aux = *self;
    if year > 0 {
      add_n_years(&mut aux, year);
    }
    if months > 0 { // More expensive than others.
      add_n_months(&mut aux, months);
    }
    if days > 0 {
      add_n_days(&mut aux, days);
    }
    validate(&aux)?;
    *self = aux;
    Ok(self)
  }

  /// Check if the result is valid, then substract n years, m months and x days to a date, in that order
  pub fn substract(&mut self, year: u32, months: u32, days: u32) -> Result<&mut Self, DateError> {
      let mut aux = *self;
    if year > 0 {
      remove_n_years(&mut aux, year);
    }
    if months > 0 { // More expensive than others.
      remove_n_months(&mut aux, months);
    }
    if days > 0 {
      remove_n_days(&mut aux, days);
    }
    validate(&aux)?;
    *self = aux;
    Ok(self)  
  }

  /// Returns the number of days between two Dates. If date1 is after date2, the result will be negative
  pub fn days_between(date1: &Date, date2: &Date) -> i64 {
    date_index(date2) - date_index(date1)
  }

  /// Returns the weekday, being 0 Monday and 6 Sunday. If the year is BC, returns Err
  pub fn weekday(&self) -> Result<u8, DateError> {
    if self.year <= 0 {
      Err(DateError::GregorianLimits)
    } else {
      Ok(gauss_algorithm(self))
    }
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
  fn multiple_operation_test() {

  }

  #[test]
  fn days_between_test() {
    let x: Date = Date::new(2000, 1, 1).expect("error creating instance in test");
    let y = Date::new(2005, 3, 30).expect("error creating instance in test");
    assert_eq!(1915, Date::days_between(&x, &y));
    assert_eq!(-1915, Date::days_between(&y, &x));
  }

  #[test]
  fn add_test() {
    let mut x = Date::new(2000, 1, 1).expect("error creating instance in test");
    let expected = Date::new(2005, 11, 21).expect("error creating instance in test");
    x.add(5, 10, 20).expect("error adding");
    assert_eq!(x, expected);
  }

  #[test]
  fn substract_test() {
    let mut x = Date::new(2000, 1, 1).expect("error creating instance in test");
    let expected = Date::new(1994, 2, 9).expect("error creating instance in test");
    x.substract(5, 10, 20).expect("error substracting");
    assert_eq!(x, expected); 
  }
} // mod lib_test
