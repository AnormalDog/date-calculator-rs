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

fn days_per_month(year: i32, part: u32) -> u32 {}

pub fn validate_raw_input(year: i32, month: u32, day: u32) -> Result<(), DateError> {}
