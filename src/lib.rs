//! Author: AnormalDog (https://github.com/AnormalDog)
//! Copyright (c) 2025 AnormalDog
//! Licensed under the MIT License. See LICENSE file in the project root for full license information.
//!
//! Internal implementation of the library

mod implementation;

struct Date {
  day: u32,
  month: Months,
  year: i32,
}
#[derive(Clone, Copy)]
enum Months {
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
  pub fn new(day: u32, month: Months, year: i32) -> Self {}
}

#[cfg(test)]
mod tests {
  use crate::implementation;

  #[test]
  fn test_is_leap() {
    let x =  implementation::is_year_leap(1996);
    assert_eq!(x, true);
  }
}
