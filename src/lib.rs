//! Author: AnormalDog (https://github.com/AnormalDog)
//! Copyright (c) 2025 AnormalDog
//! Licensed under the MIT License. See LICENSE file in the project root for full license information.
//!
//! Library

mod implementation;

use std::{fmt, ops::Rem};

#[derive(Clone, Copy, Debug)]
pub enum DateError {
  InvalidRawInput,
  OutOfLimits,
}

impl fmt::Display for DateError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match *self {
      DateError::InvalidRawInput => f.write_str("Invalid input introducing raw values"),
      DateError::OutOfLimits => {
        f.write_str("the date is outside of the limits (-9999 - 9999) years")
      }
    }
  }
}

#[derive(Clone, Copy, Debug)]
pub struct Date {
  year: i32,
  part: u32,
}

impl fmt::Display for Date {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.write_str("data")
  }
}

impl Date {
  pub fn new(year: i32, month: u32, day: u32) -> Result<Self, DateError> {}
}
