//! Read fixed width columns
//!
//!Fixed width text files are special cases of text files where the format is specified by column widths,
//! pad character and left/right alignment.
//!
//! Column widths are measured in units of characters.
//! For example, if you have data in a text file where the first column always has exactly 10 characters,
//! and the second column has exactly 5, the third has exactly 12 (and so on), this would be categorized
//! as a fixed width text file.
//!
//! To be very specific if a text file follows the rules below it is a fixed width text file:
//!
//! - Each row(paragraph) contains one complete record of information
//! - Each row contains one or many pieces of data (also referred to as columns or fields).
//! - Each data column has a defined width specified as a number of characters that is always the same for all rows.
//! - The data within each column is padded with spaces (or any character you specify) if it does not completely use all the characters allotted to it (empty space).
//! - Each column must consistently use the same number of characters, same pad character and same alignment (left/right).
use crate::core::series::Series;
use crate::io::dtypes::{is_bool, is_float, is_int, str_to_bool, str_to_float, str_to_int};
use crate::io::utils::read;
use crate::prelude::DataFrame;
use std::cmp::min;
use std::collections::HashMap;
use std::fmt::Debug;
use std::path::Path;

///The The Fixed Width File Reader
#[derive(Clone)]
pub struct FWFReader<'a> {
    data: Vec<Vec<String>>,
    settings: HashMap<&'a str, &'a str>,
    headers: Vec<String>,
}
impl Default for FWFReader<'_> {
    fn default() -> Self {
        FWFReader {
            data: Vec::new(),
            settings: HashMap::new(),
            headers: Vec::new(),
        }
    }
}
impl<'a> FWFReader<'a> {
    /// Create a new FWF reader
    pub fn new() -> FWFReader<'a> {
        Self::default()
    }
    /// Read a fixed width file
    ///
    /// To set the delimiter and line terminator,
    ///
    /// Pass option `delimiter` or `sep` in argument `settings` for delimiter and `line_terminator` to use a custom line terminator
    /// otherwise the default
    /// which is `\n` for line terminator and ` `(space) for delimiter will be used
    ///
    /// See also [`read_with_colspecs`](#method.read_with_colspecs)
    pub fn read<P: AsRef<Path> + Clone + Debug>(
        &mut self,
        path: P,
        settings: HashMap<&'a str, &'a str>,
    ) -> Self {
        let data = read(path);
        let sep = settings.get("line_terminator").unwrap_or(&"\n");
        let delimiter = settings
            .get("sep")
            .unwrap_or_else(|| settings.get("delimiter").unwrap_or(&" "));
        let split_data: Vec<String> = data
            .split(sep)
            .collect::<Vec<&str>>()
            .into_iter()
            .map(std::string::ToString::to_string)
            .collect::<Vec<String>>();
        for line in split_data {
            let split_records: Vec<String> = line
                .split(delimiter)
                .collect::<Vec<&str>>()
                .into_iter()
                .map(std::string::ToString::to_string)
                .map(|f| {
                    let data: String = f;
                    if data.starts_with(delimiter) {
                        data.trim().to_string()
                    } else {
                        data
                    }
                })
                .collect::<Vec<String>>();
            if self.headers.is_empty() {
                self.smart_push(split_records, true);
            } else {
                self.smart_push(split_records, false);
            }
        }
        self.settings = settings;
        self.own_it()
    }
    fn own_it(&self) -> Self {
        self.to_owned()
    }

    /// Read using a pair of tuples giving extents of the fixed-width fields of each line as half open
    /// intervals (i.e `[from..to]`).
    ///
    /// To set a separate line terminator,
    /// pass the argument `line_terminator` with your custom line terminator to be used
    ///
    /// If the're is no data inside colspecs or an out of bound error the data is skipped
    /// See also [read](#method.read)
    pub fn read_with_colspecs<P: AsRef<Path> + Debug + Clone>(
        &mut self,
        path: P,
        colspecs: &[(usize, usize)],
        settings: HashMap<&'a str, &'a str>,
    ) -> Self {
        let data = read(path);
        let line_sep = settings.get("line_terminator").unwrap_or(&"\n");
        let split_data: Vec<String> = data
            .split(line_sep)
            .collect::<Vec<&str>>()
            .into_iter()
            .map(std::string::ToString::to_string)
            .collect::<Vec<String>>();
        for data in split_data {
            let mut holder = Vec::new();
            for numbers in colspecs {
                let data_to_be_pushed = data.to_string().clone();
                let temp = data_to_be_pushed.get(numbers.0..numbers.1);
                match temp {
                    Some(data) => {
                        // RFC says we respect spaces :)
                        if !data.trim().is_empty() {
                            holder.push(data.trim().to_string())
                        }
                    }
                    None => continue,
                }
            }
            if self.headers.is_empty() {
                self.smart_push(holder, true);
            } else {
                self.smart_push(holder, false);
            }
        }
        self.settings = settings;
        self.own_it()
    }
    fn smart_push(&mut self, data: Vec<String>, headers: bool) {
        if headers {
            data.into_iter().for_each(|f| {
                self.headers.push(f);
            })
        } else {
            for (pos, record) in data.into_iter().enumerate() {
                let in_pos = self.data.get_mut(pos);
                if let Some(pos) = in_pos {
                    pos.push(record);
                } else {
                    self.data.push(vec![record]);
                }
            }
        }
    }
    /// Return the fwf file as a DataFrame
    pub fn to_dataframe(&self) -> DataFrame {
        let size = min(10, self.data[0].len());
        let mut df = DataFrame::new();
        for (i, j) in self.data.iter().enumerate() {
            let header = self.headers.get(i).unwrap();
            if is_int(&j[0..size]) {
                let mut series = Series::from(str_to_int(j));
                series.set_name(header.as_str());
                df.add_series(series, true).unwrap();
            } else if is_float(&j[0..size]) {
                let mut series = Series::from(str_to_float(j));
                series.set_name(header.as_str());
                df.add_series(series, true).unwrap();
            } else if is_bool(&j[0..size]) {
                let mut series = Series::from(str_to_bool(j));
                series.set_name(header.as_str());
                df.add_series(series, true).unwrap();
            } else {
                let mut series = Series::from(j.as_slice());
                series.set_name(header.as_str());
                df.add_series(series, true).unwrap();
            }
        }
        df
    }
}
