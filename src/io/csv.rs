//! Read/write/investigate/ CSV files
#![allow(dead_code)]

use std::collections::HashMap;
use std::fmt;
use std::fmt::{Debug, Display, Formatter};
use std::path::Path;

use crate::core::series::Series;
use crate::io::dtypes::{is_bool, is_float, is_int, str_to_bool, str_to_float, str_to_int};
use crate::io::utils::{is_compressed, is_url, read};
use crate::prelude::DataFrame;
use std::cmp::min;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};

/// The Error type for CSV
pub enum CSVError {
    /// The CSV cannot be parsed
    ParseError,
}

impl Debug for CSVError {
    #[allow(unreachable_patterns)]
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::ParseError => write!(f, "Could not parse csv"),
        }
    }
}

/// The CSV Reader
#[derive(Clone)]
pub struct Reader<'a> {
    data: Vec<Vec<String>>,
    builder: Builder<'a>,
    headers: Vec<String>,
    has_headers: bool,
    settings: HashMap<&'a str, &'a str>,
}

impl<'a> Default for Reader<'a> {
    fn default() -> Reader<'a> {
        Reader {
            data: Vec::new(),
            builder: Builder::new(),
            headers: Vec::new(),
            has_headers: true,
            settings: HashMap::new(),
        }
    }
}
impl<'a> Reader<'a> {
    /// Initializes a new CSV reader
    /// The underlying [`Builder`](struct.Builder.html)
    /// has the following settings set, which is default for most CSV files
    ///
    /// * `has_headers`:`true` The csv file has headers
    /// * `delimiter`:`","` The records are separated by a comma
    /// * `line_terminator`: `"\n"` Lines inside the records are separated by newline character
    /// * `quote_char`:`'\"'` Records which are quotes use the double quote character
    /// * `ignore`: `#` Lines starting with this line are ignored and treated as comments
    /// * `flexible`:`true` Wrong/erroneous records are skipped silently
    ///
    /// If these settings do not help in your context, use [`with_builder`](#method.with_builder) method
    /// instead
    /// # Returns
    ///  [`Reader<'a>`](struct.Reader.html)
    pub fn new() -> Reader<'a> {
        Self::default()
    }
    /// Parse a String as a csv
    ///
    /// If the underlying builder indicates that the CSV has headers,
    /// the first record will be treated as a header
    ///
    /// If otherwise, headers will be inferred meaning the headers will be 0,1,2...n where n
    /// is the number of records
    ///
    /// If a prefix has been set, we append the numbers to the prefix so if prefix `tatar` has been set
    /// the headers change to `tatar0`,`tatar1` and so on
    ///
    /// # Warning
    /// This function should not be called publicly in production
    /// as it does not ascertain whether some corner cases are checked eg if no duplicate names are passed
    ///
    /// Call it at your own risk
    ///
    /// # Returns
    /// [Reader<'a>](struct.Reader.html)
    fn parse_string_csv(&mut self, data: &str) -> DataFrame {
        if self.builder.has_headers && self.headers.is_empty() {
            let headers = data.lines().next().unwrap();
            self.smart_push(
                smart_split(headers, self.builder.delimiter, self.builder.quote_char),
                true,
            )
            .unwrap();
        }
        for line in data.lines() {
            if line.starts_with(self.builder.ignore) {
                continue;
            }
            // Smart split is actually noice :)
            let split_lines = smart_split(line, self.builder.delimiter, self.builder.quote_char);
            self.smart_push(split_lines, false).unwrap()
        }
        self.to_dataframe()
    }
    fn own_it(&self) -> Self {
        self.to_owned()
    }
    ///Parse a csv file
    ///
    /// to parse the data and also calls [`update_kwargs`](#method.update_kwargs) to update keyword
    /// arguments
    pub fn parse_csv<P: AsRef<Path> + Debug + Clone>(
        &mut self,
        path: P,
        kwargs: HashMap<&'a str, &'a str>,
    ) -> DataFrame {
        self.update_kwargs(kwargs);

        if is_url(path.as_ref().to_str().unwrap()) || is_compressed(path.as_ref().to_str().unwrap())
        {
            let lines = read(path);
            self.parse_string_csv(&lines)
        } else {
            // For local files. We don't need to read the whole file to memory we can parse it line by line
            self.parse_local_file(path.as_ref().to_str().unwrap())
        }
    }
    fn parse_local_file(&mut self, path: &str) -> DataFrame {
        let fd = File::open(path).unwrap();
        let buf = BufReader::new(fd);
        for line in buf.lines() {
            let line = line.unwrap();
            if self.builder.has_headers && self.headers.is_empty() {
                let headers = line;
                self.smart_push(
                    smart_split(
                        headers.as_str(),
                        self.builder.delimiter,
                        self.builder.quote_char,
                    ),
                    true,
                )
                .unwrap();
                continue;
            }
            if line.starts_with(self.builder.ignore) {
                continue;
            }
            // Smart split is actually noice :)
            let split_lines = smart_split(
                line.as_str(),
                self.builder.delimiter,
                self.builder.quote_char,
            );
            self.smart_push(split_lines, false).unwrap()
        }
        self.to_dataframe()
    }
    /// Update keyword arguments settings for the CSV reader
    ///
    /// # Arguments
    /// `new_kwargs`: New  keyword arguments
    ///
    /// > The following keywords are ripped from the `new_kwargs`
    ///
    /// >> `sep` or `delimiter`: Becomes the new delimiter of the underlying [Builder](struct.Builder.html)
    ///
    /// >> `line_terminator`: Becomes the new line terminator od the underlying builder.
    ///
    /// >> `ignore` : becomes the new ignore of the underlying builder
    ///
    /// >> `names` : Becomes the new headers of the CSV files
    fn update_kwargs(&mut self, mut new_kwargs: HashMap<&'a str, &'a str>) {
        if new_kwargs.contains_key("sep") || new_kwargs.contains_key("delimiter") {
            self.builder.set_delimiter(
                new_kwargs
                    .get("sep")
                    .unwrap_or_else(|| new_kwargs.get("delimiter").unwrap()),
            );
        }
        self.builder
            .set_line_terminator(new_kwargs.get("line_terminator").unwrap_or(&"\n"));
        self.builder
            .set_ignore(new_kwargs.get("ignore").unwrap_or(&"#"));
        // A string containing comma separated values of headers
        if !new_kwargs.get("names").unwrap_or(&"").is_empty() {
            // Is this right?
            // TODO: Review this (I hope it works)
            let headers = new_kwargs.get("names").unwrap();
            let headers = smart_split(headers, self.builder.delimiter, self.builder.quote_char);
            self.smart_push(headers, true).unwrap();
            new_kwargs.remove("names");
            self.has_headers = true;
            self.builder.set_headers(false);
            self.settings = new_kwargs;
        }
    }
    /// Push data into the buffer
    ///
    /// # Memory representation
    /// ```text
    ///
    /// | index 0         | index 1                  |index 2
    /// |-----------------|--------------------------|--------------------------|
    /// |Title of record  | Record at column 1 line 2| record at column 1 line 3|
    /// |Title of record  | Record at column 1 line 2| record at column 2 line 3|
    ///```
    fn smart_push(&mut self, data: Vec<String>, is_headers: bool) -> Result<(), CSVError> {
        if is_headers {
            data.into_iter()
                .for_each(|f| self.headers.push(f.trim().to_string()));
            Ok(())
        } else {
            for (pos, record) in data.into_iter().enumerate() {
                let in_pos = self.data.get_mut(pos);
                if let Some(pos) = in_pos {
                    pos.push(record.trim().to_string());
                } else {
                    self.data.push(vec![record.trim().to_string()]);
                }
            }
            // If the builder hasn't been set to flexible, see if the vectors all have same length
            let len = self.data.len();
            if !self.builder.flexible {
                for i in &self.data {
                    if i.len() != len {
                        return Err(CSVError::ParseError);
                    }
                }
            }
            Ok(())
        }
    }
    /// Convert a CSV to a DataFrame
    ///
    /// Currently. This uses the first record in the array to determine the type of the records for that column
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
                let mut series = Series::from(j.to_owned());
                series.set_name(header.as_str());
                df.add_series(series, true).unwrap();
            }
        }
        df
    }
}
/// A  builder that exposes some common settings for the CSV reader
///
/// The following settings can be set from the builder
/// # Arguments
/// * `has_headers`:[`bool`] the CSV reader uses this to determine if the line of the csv is a header or not
/// * `delimiter`:[`str`] the delimiter to use to separate csv records
/// * `line_terminator`:[`str`] the line terminator `\n` for Linux mainly and `\r\n` for Windows
/// * `flexible`:[`bool`] whether erroneous daa should be skipped
/// * `quote_char`:[`str`] Quote character for the csv
/// * `ignore`:[`str`]  Ignore lines starting with the following character
#[derive(Debug, Clone)]
pub struct Builder<'a> {
    /// Whether or not the CSV file has headers
    has_headers: bool,
    /// Delimiters for the csv file
    delimiter: &'a str,
    ////Line terminator for the file
    line_terminator: &'a str,
    ///Whether the structure of the CSV is fixed (all files have equal fields) or not
    flexible: bool,
    ///quote character
    quote_char: &'a str,
    ignore: &'a str,
}

impl Default for Builder<'_> {
    /// Creates the default settings for most csv files
    fn default() -> Self {
        Builder {
            has_headers: true,
            delimiter: ",",
            line_terminator: "\n",
            flexible: true,
            quote_char: "\"",
            ignore: "#",
        }
    }
}

impl<'a> Builder<'a> {
    /// Create a new builder with default options
    pub fn new() -> Self {
        Self::default()
    }
    /// Get delimiter of the builder
    pub const fn delimiter(self) -> &'a str {
        self.delimiter
    }
    /// Set the delimiter of the Builder
    pub fn set_delimiter(&mut self, delimiter: &'a str) -> &mut Self {
        self.delimiter = delimiter;
        self
    }
    /// Whether the CSV has headers
    ///  # Example
    /// ```
    pub const fn headers(&self) -> bool {
        self.has_headers
    }
    /// Set whether the csv has headers
    ///
    /// This will make the csv parser to assume the first line contains headers
    ///
    ///# Arguments
    ///  `value`: [`bool`] set to true if the CSV has headers and false if otherwise
    pub fn set_headers(&mut self, value: bool) -> &mut Self {
        self.has_headers = value;
        self
    }
    ///Get the line terminator of the builder
    pub const fn line_terminator(&self) -> &'a str {
        self.line_terminator
    }
    /// Set the line terminator of the builder
    pub fn set_line_terminator(&mut self, terminator: &'a str) -> &mut Self {
        self.line_terminator = terminator;
        self
    }
    /// Get the flexibility of the builder
    ///
    /// If this returns `false` it means the CSV parser will panic on many things
    ///
    /// Eg. a line containing excess records
    ///
    /// A line containing newline
    ///
    /// A line containing little records e.t.c
    pub const fn flexible(&self) -> bool {
        self.flexible
    }
    /// Set the flexibility of the builder
    /// # Warning
    /// Please set this to be true if you are **sure** that the csv is well parsed
    /// Otherwise it will panic if it encounters a wrongly formatted record
    ///
    /// If set to true, the CSV parser will ignore any wrongly formatted records
    pub fn set_flexible(&mut self, flexibility: bool) -> &mut Self {
        self.flexible = flexibility;
        self
    }
    /// Get the ignore character
    ///
    /// If a line starts with this, the whole line will be ignored and not parsed
    ///
    /// This can be used if a csv has comments
    pub const fn ignore(self) -> &'a str {
        self.ignore
    }
    /// Set the ignore string

    /// Any line matching this input will be ignored
    pub fn set_ignore(&mut self, ignore: &'a str) -> &mut Self {
        self.ignore = ignore;
        self
    }
    /// Own the data
    ///
    /// This function is called to convert a `&mut Builder` to a `Builder`
    /// Which can then be used with the CSV parser [`with_builder`](struct.Reader.html#method.with_builder) method
    pub fn build(&self) -> Self {
        self.to_owned()
    }
}
/// Write a [`Series`] to a csv
pub fn series_to_csv<T: Clone + Display + Default + 'static, P: Write>(
    series: &Series<T>,
    filepath_or_buffer: &mut P,
) {
    let record_name = series.get_name() + "\n";
    let records = series.to_ndarray();
    filepath_or_buffer
        .write_all(record_name.as_bytes())
        .unwrap();
    // Hey there coder :)
    records.iter().for_each(|f| {
        filepath_or_buffer
            .write_all(format!("{}\n", f).as_bytes())
            .unwrap();
    });
    filepath_or_buffer.flush().unwrap();
}
fn smart_split(string: &str, split_at: &str, quote_char: &str) -> Vec<String> {
    if string.contains(quote_char) {
        // Otherwise use the special split if we have a quote character
        let mut new_list = vec![];
        let mut temp_holder = vec![];
        let mut inside_quotes = false;
        for each_letter in <&str>::clone(&string).chars() {
            if each_letter.to_string() == quote_char && !inside_quotes {
                inside_quotes = true;
            } else if each_letter.to_string() == quote_char && inside_quotes {
                inside_quotes = false
            }
            if !inside_quotes && each_letter.to_string() == split_at {
                new_list.push(temp_holder.clone().into_iter().collect());
                temp_holder.clear();
                continue;
            } else {
                temp_holder.push(each_letter);
            }
        }
        if !temp_holder.is_empty() {
            new_list.push(temp_holder.into_iter().collect());
        }
        new_list
    } else {
        // If it doesn't contain the quote_char we can use default split
        string
            .split(split_at)
            .map(std::string::ToString::to_string)
            .collect()
    }
}

// Why this long :<|
