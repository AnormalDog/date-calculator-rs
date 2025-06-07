//! Author: AnormalDog (https://github.com/AnormalDog)
//! Copyright (c) 2025 AnormalDog
//! Licensed under the MIT License. See LICENSE file in the project root for full license information.
//!
//! Internal implementation of the library

use crate::implementation::standardize_year;

mod implementation;

pub struct Date {
  year: i32,
  month: Months,
  day: u32,
}

#[derive(Clone, Copy)]
pub enum Months {
  Jan,
  Feb,
  Mar,
  Apr,
  May,
  Jun,
  Jul,
  Aug,
  Sep,
  Oct,
  Nov,
  Dec,
}

impl Date {
  /// Create a new instance of Date
  pub fn new(year: i32, month: Months, day: u32) -> Result<Self, String> {
    match implementation::is_new_date_valid(year, month, day) {
      Ok(_) => Ok(Date{year : standardize_year(year), month : month, day : day}),
      Err(error) => Err(error)
    }
  }
} // impl Date

#[cfg(test)]
mod tests {
  use crate::{implementation, Date, Months};

  #[test]
  fn test_is_leap() {
    let x = implementation::is_year_leap(1996);
    assert_eq!(x, true);
  }

  #[test]
  fn test_new_date() {
    match Date::new(-1, Months::Feb, 29) {
      Ok(_) => assert!(true),
      Err(_) => assert!(false)
    }
  }


}