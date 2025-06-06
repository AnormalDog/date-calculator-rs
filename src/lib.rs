//! Author: AnormalDog (https://github.com/AnormalDog)
//! Copyright (c) 2025 AnormalDog
//! Licensed under the MIT License. See LICENSE file in the project root for full license information.
//!
//! Internal implementation of the library

mod implementation;

pub struct Date {
  day: u32,
  month: Months,
  year: i32,
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
  pub fn new(day: u32, month: Months, year: i32) -> Result<Self, String> {
    let x = Date {
      day: day,
      month: month,
      year: year,
    };
    match implementation::is_date_valid(&x) {
      Ok(()) => Ok(x),
      Err(string) => Err(string)
    }
  }

  pub fn is_leap(&self) -> bool {
    return implementation::is_year_leap(self.year);
  }
}

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
    match Date::new(28, Months::Feb, 2001) {
      Ok(_) => assert!(true),
      Err(_) => assert!(false)
    }
  }
}