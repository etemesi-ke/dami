//! The DateTimeIndex module
use chrono::{DateTime, Datelike, NaiveDateTime};
use ndarray::Array1;
use prettytable::format::consts::FORMAT_CLEAN;
use prettytable::{Cell, Row, Table};
use std::fmt;
use std::ops::{Index, IndexMut};
/// A struct that holds Date and Time indexes
pub struct DateTimeIndex {
    index: Array1<i64>,
}
impl IndexMut<usize> for DateTimeIndex {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.index.index_mut(index)
    }
}
impl Index<usize> for DateTimeIndex {
    type Output = i64;

    fn index(&self, index: usize) -> &Self::Output {
        self.index.index(index)
    }
}
impl DateTimeIndex {
    /// Indicate whether the Index is empty
    pub fn is_empty(&self) -> bool {
        self.index.is_empty()
    }
    /// Create a DateTime index that can hold `capacity` dates
    pub fn with_capacity(capacity: usize) -> DateTimeIndex {
        DateTimeIndex {
            index: Array1::zeros(capacity),
        }
    }
    /// Parse a vec of RFC-3339 strings to a DateTimeIndex
    ///
    /// # Panics
    /// If any of the strings cannot be converted to a datetime
    pub fn from_rfc3339(dates: &[&str]) -> DateTimeIndex {
        let mut dt = DateTimeIndex::with_capacity(dates.len());
        for (len, i) in dates.iter().enumerate() {
            dt[len] = DateTime::parse_from_rfc3339(i).unwrap().timestamp();
        }
        dt
    }
    /// Parse a vec of RFC-2822 strings to a DateTimeIndex
    ///
    /// # Panics
    /// If any of the strings cannot be converted to a datetime
    pub fn from_rfc2822(dates: &[&str]) -> DateTimeIndex {
        let mut dt = DateTimeIndex::with_capacity(dates.len());
        for (len, i) in dates.iter().enumerate() {
            dt[len] = DateTime::parse_from_rfc2822(i).unwrap().timestamp();
        }
        dt
    }
    /// Parse a vec of date like strings to a DateTimeIndex using the format
    /// in `fmt` argument
    ///
    /// Supported format strings can be found [here](https://docs.rs/chrono/0.4.0/chrono/format/strftime/index.html)
    ///
    /// # Panics
    /// If any of the strings cannot be converted to a datetime
    pub fn from_str(dates: &[&str], fmt: &str) -> DateTimeIndex {
        let mut dt = DateTimeIndex::with_capacity(dates.len());
        for (len, i) in dates.iter().enumerate() {
            dt[len] = DateTime::parse_from_str(i, fmt).unwrap().timestamp();
        }
        dt
    }
    /// Parse a vec of date like strings to a DateTimeIndex using the format
    /// in `fmt` argument
    ///
    /// Supported format strings can be found [here](https://docs.rs/chrono/0.4.0/chrono/format/strftime/index.html)
    ///
    /// # Panics
    /// If any of the strings cannot be converted to a datetime
    pub fn insert_str_at(&mut self, idx: usize, str: &str, fmt: &str) {
        assert!(
            self.index.len() > idx,
            "attempted to insert index at a location that doesn't exist"
        );
        self.index[idx] = NaiveDateTime::parse_from_str(str, fmt).unwrap().timestamp();
    }
    /// Parse a reference of i64 into a DateTime index
    ///
    /// The integers are assumed to be seconds elapsed since epoch time
    pub fn from_i64(values: &[i64]) -> DateTimeIndex {
        let mut dt = DateTimeIndex::with_capacity(values.len());
        for (i, j) in values.iter().enumerate() {
            dt[i] = *j
        }
        dt
    }
    /// Insert a timestamp at index `idx`
    ///
    pub fn insert(&mut self, idx: usize, item: i64) {
        assert!(
            self.index.len() < idx,
            "attempted to insert index at a location that doesn't exist"
        );
        self.index[idx] = item
    }
    /// Get length of underlying array
    pub fn len(&self) -> usize {
        self.index.len()
    }
    /// Get the years from the array
    pub fn year(&self) -> Vec<i32> {
        self.index
            .iter()
            .map(|f| NaiveDateTime::from_timestamp(*f, 0).year())
            .collect::<Vec<i32>>()
    }

    /// Get the months from the array
    pub fn month(&self) -> Vec<u32> {
        self.index
            .iter()
            .map(|f| NaiveDateTime::from_timestamp(*f, 0).month())
            .collect::<Vec<u32>>()
    }
    /// Get days of the month from the array
    pub fn day_of_month(&self) -> Vec<u32> {
        self.index
            .iter()
            .map(|f| NaiveDateTime::from_timestamp(*f, 0).day())
            .collect::<Vec<u32>>()
    }
    /// Get the ISO week from the array
    pub fn week(&self) -> Vec<u32> {
        self.index
            .iter()
            .map(|f| NaiveDateTime::from_timestamp(*f, 0).iso_week().week())
            .collect::<Vec<u32>>()
    }
    /// Get days of the year from the array
    pub fn day(&self) -> Vec<u32> {
        self.index
            .iter()
            .map(|f| NaiveDateTime::from_timestamp(*f, 0).ordinal())
            .collect::<Vec<u32>>()
    }
    /// Indicate whether the date is the months start
    pub fn is_month_start(&self) -> Vec<bool> {
        self.index
            .iter()
            .map(|f| NaiveDateTime::from_timestamp(*f, 0).day() == 1)
            .collect::<Vec<bool>>()
    }
    /// Indicate whether the date is the month's end
    pub fn is_month_end(&self) -> Vec<bool> {
        let thirty_one_month: [u32; 7] = [1, 3, 5, 7, 8, 9, 11];
        self.index
            .iter()
            .map(|f| {
                let dt = NaiveDateTime::from_timestamp(*f, 0);
                if dt.day() == 31 && thirty_one_month.contains(&dt.month()) {
                    true
                }
                // Cover leap years
                else if dt.day() >= 28 && dt.month() == 2 {
                    if dt.year() % 4 == 0
                        && (dt.year() % 100 != 0 || dt.year() % 400 == 0)
                        && dt.day() == 29
                    {
                        true
                    } else {
                        !(dt.year() % 4 == 0
                            && (dt.year() % 100 != 0 || dt.year() % 400 == 0)
                            && dt.day() == 28)
                    }
                }
                // If day is 30 tick
                else {
                    dt.day() == 30
                }
            })
            .collect::<Vec<bool>>()
    }
    /// Indicator whether the date is the first date of the quarter
    pub fn is_quarter_start(&self) -> Vec<bool> {
        self.index
            .iter()
            .map(|f| {
                let dt = NaiveDateTime::from_timestamp(*f, 0);
                dt.month() % 3 == 0 && dt.day() == 1
            })
            .collect::<Vec<bool>>()
    }
    /// Indicator whether the date is the last day of the quarter
    pub fn is_quarter_end(&self) -> Vec<bool> {
        let q_months: [u32; 4] = [1, 4, 7, 10];
        self.index
            .iter()
            .map(|f| {
                let dt = NaiveDateTime::from_timestamp(*f, 0);
                q_months.contains(&dt.month()) && dt.day() == 1
            })
            .collect::<Vec<bool>>()
    }
    /// Indicate whether the date is the start of the year
    pub fn is_year_start(&self) -> Vec<bool> {
        self.index
            .iter()
            .map(|f| {
                let dt = NaiveDateTime::from_timestamp(*f, 0);
                dt.month() == 1 && dt.day() == 1
            })
            .collect::<Vec<bool>>()
    }
    /// Indicate whether the date is the year end
    pub fn is_year_end(&self) -> Vec<bool> {
        self.index
            .iter()
            .map(|f| {
                let dt = NaiveDateTime::from_timestamp(*f, 0);
                dt.month() == 12 && dt.day() == 31
            })
            .collect::<Vec<bool>>()
    }
    /// Indicator if the date belongs to a leap year
    ///
    /// A leap year is a year, which has 366 days (instead of 365)
    /// including 29th of February as an intercalary day.
    ///
    /// Leap years are years which are multiples of four with the exception of years
    /// divisible by 100 but not by 400.
    pub fn is_leap_year(&self) -> Vec<bool> {
        self.index
            .iter()
            .map(|f| {
                let year = NaiveDateTime::from_timestamp(*f, 0).year();
                year % 4 == 0 && (year % 100 != 0 || year % 400 == 0)
            })
            .collect::<Vec<bool>>()
    }
    /// Convert to index using the specified date format
    ///
    /// For help on formats visit [here](https://docs.rs/chrono/0.4.0/chrono/format/strftime/index.html)
    pub fn strftime(&self, date_format: &str) -> Vec<String> {
        self.index
            .iter()
            .map(|f| {
                let dt = NaiveDateTime::from_timestamp(*f, 0);
                dt.format(date_format).to_string()
            })
            .collect::<Vec<String>>()
    }
}
impl fmt::Debug for DateTimeIndex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut tables = Table::new();
        tables.set_format(*FORMAT_CLEAN);
        for i in self.index.iter() {
            let dt = NaiveDateTime::from_timestamp(*i, 0);
            tables.add_row(Row::new(vec![Cell::new(
                &dt.format("%Y-%m-%d").to_string(),
            )]));
        }
        write!(f, "{}", tables.to_string())
    }
}
