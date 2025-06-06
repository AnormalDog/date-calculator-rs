//! Author: AnormalDog (https://github.com/AnormalDog)
//! Copyright (c) 2025 AnormalDog
//! Licensed under the MIT License. See LICENSE file in the project root for full license information.
//!
//! Internal implementation of the library

use crate::Months;

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

/// Returns if a year is valid or not
/// For now, the function quite simple, not sure if I will need to do more
///   checks
fn is_year_valid(year: i32) -> bool {
  if year == 0 {
    return false;
  };
  return true;
}

/// Returns the number of days a months has.
/// If the month is february, returns 0, which means that need to consideer the year
fn get_number_of_days(month: Months) -> u32 {
  match month {
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

/// Returs the max number of days a months can hold
fn max_num_days(date: &crate::Date) -> u32 {
  match date.month {
    Months::Feb => {
      if is_year_leap(date.year) == true {
        29
      } else {
        28
      }
    }
    _ => get_number_of_days(date.month),
  }
}

/// Check if a object of Struct Date is correct and valid
pub fn is_date_valid(date: &crate::Date) -> Result<(), String> {
  // check year
  if is_year_valid(date.year) == false {
    return Err(String::from("Invalid year. Probably 0?"));
  }
  // check days
  let max_num_days: u32 = max_num_days(date);
  if date.day == 0 || date.day > max_num_days {
    return Err(String::from("Invalid day, out of month range"));
  }
  Ok(())
}
