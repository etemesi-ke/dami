//! Common miscellaneous functions for the crate
#![allow(unused_imports)]
use crate::core::index::date_time::DateTimeIndex;
use chrono::NaiveDateTime;
#[cfg(feature = "regex")]
use regex::Regex;
use std::collections::HashMap;
use std::hash::Hash;

/// Get the most frequent element in an array
///
/// Time complexity O(n)
pub fn most_frequent<T: Default + Eq + Hash + Clone>(arr: &[T]) -> (i32, T) {
    let mut dict = HashMap::new();
    // Insert elements into the hash
    (0..arr.len()).for_each(|i| {
        if dict.contains_key(&arr[i]) {
            let a = dict.get_mut(&arr[i]).unwrap();
            *a += 1;
        } else {
            //Add one
            dict.insert(&arr[i], 1);
        }
    });
    // Find max frequency
    let mut max_count = 0;
    let mut elm = &T::default();
    for i in dict {
        if max_count < i.1 {
            max_count = i.1;
            elm = i.0;
        }
    }
    (max_count, elm.to_owned())
}
/// Create a date-time index
///
/// The date range is matched to a regex which accepts the format dd-mm-yyyy dd.mm.yyyy or dd/mm/yyyy
/// format
/// # Note
/// * NaiveDateTime struct from [chrono] is used so TimeZones are not respected and to create new days
///   we add 864000 seconds (1 day == 86,4000 sec). to the previous' date's epoch
/// * Leap seconds are not recognized.But leap years are.
/// * Dates are started from midnight
/// # Panics
/// * If the `start` or `Option<end>` do not match the regex.
/// * If either `periods` or `end` option is not specified
///
/// [chrono]: https://docs.rs/chrono
#[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
#[cfg(feature = "regex")]
pub fn date_range(start: &str, end: Option<&str>, periods: Option<i32>) -> DateTimeIndex {
    let start = start.replace(".", "/").replace("-", "/").trim().to_string();
    // See https://stackoverflow.com/questions/15491894/regex-to-validate-date-format-dd-mm-yyyy
    // This is long :<\
    let regex_dt = Regex::new(
        r"(^(((0[1-9]|1[0-9]|2[0-8])[/](0[1-9]|1[012]))|((29|30|31)[/](0[13578]|1[02]))|((29|30)[/](0[4,6,9]|11)))[/](19|[2-9][0-9])\d\d$)|(^29[/]02[/](19|[2-9][0-9])(00|04|08|12|16|20|24|28|32|36|40|44|48|52|56|60|64|68|72|76|80|84|88|92|96)$)").unwrap();
    if regex_dt.is_match(&start) {
        // So we have start dates all aligned to dd-mm-yyyy
        let mut start = start.replace(".", "/").replace("-", "/").trim().to_string();
        if let Some(per) = periods {
            // Periods are treated as days eg period 3 means 3 days.
            // Negative periods means we go back...
            // TODO: Is it fine to have negative periods..
            start.push_str(" 00:00:00");
            let mut dt = DateTimeIndex::with_capacity(per.abs() as usize + 1);
            dt.insert_str_at(0, &start, "%d/%m/%Y %H:%M:%S");

            (1..per).for_each(|f| {
                let temp_dt = NaiveDateTime::parse_from_str(&start, "%d/%m/%Y %H:%M:%S").unwrap();
                let new_time = temp_dt.timestamp() + i64::from(f * 86400);
                dt.insert(f.abs() as usize, new_time);
            });
            dt
        } else if let Some(stop) = end {
            if regex_dt.is_match(stop) {
                let mut end = stop.replace(".", "/").replace("-", "/");
                start.push_str(" 00:00:00");
                let start_dt = NaiveDateTime::parse_from_str(&start, "%d/%m/%Y %H:%M:%S")
                    .unwrap()
                    .timestamp();
                // Again we have dd-mm-yyyy
                end.push_str(" 00:00:00");
                let end_dt = NaiveDateTime::parse_from_str(&end, "%d/%m/%Y %H:%M:%S")
                    .unwrap()
                    .timestamp();
                // Get date range
                let periods_in_between = (end_dt - start_dt) / 86400;
                let mut dt = DateTimeIndex::with_capacity((periods_in_between.abs() + 1) as usize);
                dt.insert(0, start_dt);
                dt.insert(dt.len() - 1, end_dt);
                let mut prev_date = start_dt;
                for lazy in 1..periods_in_between {
                    prev_date += 86400;
                    dt.insert(lazy as usize, prev_date);
                }
                dt
            } else {
                panic!("End time str {} does not match the regex", stop);
            }
        } else {
            panic!("Either specify a end argument or periods argument")
        }
    } else {
        panic!("Start date doesn't match dd-mm-yyyy format")
    }
}
