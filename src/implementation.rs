//! Author: AnormalDog (https://github.com/AnormalDog)
//! Copyright (c) 2025 AnormalDog
//! Licensed under the MIT License. See LICENSE file in the project root for full license information.
//!
//! Internal implementation of the library

use crate::Months;

impl Months {
  /// Returns a i8 representing the order of the months
  fn get_month_index(self) -> i8 {
    match self {
      Months::Jan => 0,
      Months::Feb => 1,
      Months::Mar => 2,
      Months::Apr => 3,
      Months::May => 4,
      Months::Jun => 5,
      Months::Jul => 6,
      Months::Aug => 7,
      Months::Sep => 8,
      Months::Oct => 9,
      Months::Nov => 10,
      Months::Dec => 11
    }
  }

  /// Returns the number of days a months has.
  /// If the month is february, returns 0, which means that need to consideer the year
  fn get_number_of_days(self) -> u32 {
    match self {
      Months::Jan
      | Months::Mar
      | Months::May
      | Months::Jul
      | Months::Aug
      | Months::Oct
      | Months::Dec => 31,
      Months::Feb => 0,
      _ => 30,
    }
  }
} // impl Months

/// Returns true or false if the year is leap or not
pub fn is_year_leap(year: i32) -> bool {
  if year % 4 != 0 {
    return false;
  }
  if year % 100 != 0 {
    return true;
  }
  if year % 400 == 0 { true } else { false }
}

/// Returs the max number of days a months can hold
fn max_num_days_in_month(year : i32, month : Months) -> u32 {
  match month {
    Months::Feb => {
      if is_year_leap(year) == true {
        29
      } else {
        28
      }
    }
    _ => month.get_number_of_days(),
  }
}

/// Check if a object of Struct Date is correct and valid
pub fn is_new_date_valid(year: i32, month: Months, day: u32) -> Result<(), String> {
  // check year
  if year == 0 {
    return Err(String::from("Invalid year, probably 0?"));
  }
  // check days
  let max_num_days: u32 = max_num_days_in_month(standardize_year(year), month);
  if day == 0 || day > max_num_days {
    return Err(String::from("Invalid day, out of month range"));
  }
  Ok(())
}

/// Standardize the year, adding +1 to BC years
pub fn standardize_year(year : i32) -> i32 {
  assert_ne!(year, 0);
  if year < 0 {
    return year + 1;
  }
  else {
    return year;
  }
}

/// Normalize into BC-AC year format
pub fn unstandardize_year(year : i32) -> i32 {
  if year <= 0 {
    return year - 1;
  }
  else {
    return year;
  }
}