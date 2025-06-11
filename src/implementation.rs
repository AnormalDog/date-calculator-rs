//! Author: AnormalDog (https://github.com/AnormalDog)
//! Copyright (c) 2025 AnormalDog
//! Licensed under the MIT License. See LICENSE file in the project root for full license information.
//!
//! Internal implementation of the library

use crate::{Date, DateError};

fn is_year_leap(year: i32) -> bool {
  if year % 4 != 0 {
    false
  } else if year % 100 != 0 {
    true
  } else {
    year % 400 == 0
  }
}

fn days_per_month(year: i32, month: u32) -> u32 {
  assert!(month > 0 && month <= 12);
  match month {
    1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
    2 if is_year_leap(year) => 29,
    2 if !is_year_leap(year) => 28,
    _ => 30,
  }
}

pub fn validate_raw_input(year: i32, month: u32, day: u32) -> Result<(), DateError> {
  if year == 0 ||
    month == 0 || month > 12 ||
    day == 0 || day > days_per_month(year, month) {
      Err(DateError::InvalidRawInput)
    }
    else {
      Ok(())
    }
}

fn get