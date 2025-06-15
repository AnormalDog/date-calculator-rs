# date-calculator-rs
## About
Library that implements date operations.   
My idea for this library is creating a simple, safe way of working with dates.  
All is around the struct Date, with a range of 9999BC to 9999AC.
### implementation
- The year is saved in a i32. The BC is moved one unit to right, so 1BC == 0 2BC == -1, etc.
- The M/D format is internally saved as Days \[1..365/366\].
## Usage
- Create a new instance of ```Date``` with ```Date::new(year, month, day)```
- if the method would change the value of the instance, will only do it if the result is valid. That's why methods returns ```Result<&Self, DateError>```
## Planned implementations
- [x]  Is date's year leap.
- [x]  New.
- [x]  Validate. 
- [x]  Add days/weeks/months/years.
- [x]  Delete days/weeks/months/years.
- [x]  Count days between two dates.
- [x]  What day of the week is.
## LICENSE
[MIT](https://choosealicense.com/licenses/mit/)