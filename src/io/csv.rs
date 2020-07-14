//! Read/write/investigate/ CSV files

use std::collections::HashMap;
use std::fmt;
use std::fmt::{Debug, Display, Formatter};
use std::num::{ParseFloatError, ParseIntError};
use std::path::Path;

use prettytable::{format, Cell, Row, Table};

use crate::core::series::Series;
use crate::io::dtypes::{is_bool, is_float, is_int, str_to_bool, str_to_float, str_to_int};
use crate::io::utils::read;
use crate::prelude::DataFrame;
use std::cmp::min;
use std::f64::NAN;
use std::io::Write;

/// The Error type for CSV
pub enum CSVError {
    /// The CSV cannot be parsed
    ParseError,
    /// The CSV file could not be converted to a certain type
    ConvertError(String),
}

impl Debug for CSVError {
    #[allow(unreachable_patterns)]
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::ParseError => write!(f, "Could not parse csv"),
            Self::ConvertError(ref pos) => write!(f, "Could not convert csv to type\n{}", pos),
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
    /// Create a new Reader with a customised builder
    ///
    /// This will override the default settings of the builder
    pub fn with_builder(builder: Builder<'a>) -> Reader {
        let has_headers = builder.has_headers;
        Reader {
            data: Vec::new(),
            builder,
            headers: Vec::new(),
            has_headers,
            settings: HashMap::new(),
        }
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
    pub fn parse_string_csv(&mut self, data: &str) -> Self {
        // Split the string to lines
        let mut lines: Vec<String> = data
            .split(self.builder.line_terminator)
            .collect::<Vec<&str>>()
            .into_iter()
            .map(std::string::ToString::to_string)
            .collect::<Vec<String>>();
        // We can already guess its capacity
        self.data = Vec::with_capacity(lines.len());
        if self.settings.contains_key("headers") && !self.has_headers {
            let headers = self.settings.get("headers").unwrap();
            // Infer column names
            // TODO:Change how this orientation is seen
            if headers == &"0" {
                lines.clone().into_iter().for_each(|f| {
                    self.smart_push(
                        smart_split(&f, self.builder.delimiter, self.builder.quote_char),
                        false,
                    )
                    .unwrap()
                });
                if has_header(self.data.as_slice()) {
                    // DEPLOY THE TROOPS => |:|:|:|:|
                    self.has_headers = true;
                } else {
                    let values = self.data.len();
                    let prefix = self.settings.get("prefix").unwrap_or(&"");
                    // :\
                    for p in 0..values {
                        self.headers.push(format!("{}{}", prefix, p))
                    }
                    self.has_headers = true
                }
            }
        }
        if self.builder.has_headers && self.headers.is_empty() {
            let headers = lines.remove(0);
            self.smart_push(
                smart_split(
                    headers.as_str(),
                    self.builder.delimiter,
                    self.builder.quote_char,
                ),
                true,
            )
            .unwrap();
        }
        for line in lines {
            if line.starts_with(self.builder.ignore){
                continue
            }
            // Smart split is actually noice :)
            let split_lines = smart_split(
                line.as_str(),
                self.builder.delimiter,
                self.builder.quote_char,
            );
            self.smart_push(split_lines, false).unwrap()
        }
        self.own_it()
    }
    fn own_it(&self) -> Self {
        self.to_owned()
    }
    ///Parse a csv file
    ///
    /// This fetches data and calls [`parse_string_csv`](#method.parse_string_csv)
    /// to parse the data and also calls [`update_kwargs`](#method.update_kwargs) to update keyword
    /// arguments
    pub fn parse_csv<P: AsRef<Path> + Debug + Clone>(
        &mut self,
        path: P,
        kwargs: HashMap<&'a str, &'a str>,
    ) -> Reader<'a> {
        let lines = read(path);
        self.update_kwargs(kwargs);
        self.parse_string_csv(&lines)
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
    pub fn update_kwargs(&mut self, mut new_kwargs: HashMap<&'a str, &'a str>) {
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
    /// Convert a csv row to A [`Series`] type which allows higher level manipulation of data
    /// # Arguments
    /// > `idx`: ID of the row to convert
    pub fn to_series_string(&self, idx: usize) -> Series<String> {
        Series::from(self.data[idx].clone())
    }

    /// Convert a csv row to A [`Series`] type which allows higher level manipulation of data
    /// # Arguments
    /// > `idx`: ID of the row to convert
    ///
    /// If type can't be converted, it is replaced with NaN
    pub fn to_series_float(&self, idx: usize) -> Series<f64> {
        let mut new_vec = vec![];
        self.data[idx].clone().into_iter().for_each(|f| {
            let convert: f64 = f.parse::<f64>().unwrap_or(NAN);
            new_vec.push(convert);
        });
        // What is a NAN value? What who is the fastest coder alive?
        let mut new_series = Series::from(new_vec);
        new_series.set_name(self.headers[idx].as_str());
        new_series
    }

    /// Convert a csv row to A [`Series`] type which allows higher level manipulation of data
    /// # Arguments
    /// > `idx`: ID of the row to convert
    ///
    /// > `default`: Default value to use if the [`String`] cannot be converted to an int
    pub fn to_series_int(&self, idx: usize, default: i32) -> Series<i32> {
        let mut new_vec = vec![];
        // Clone and parse
        self.data[idx].clone().into_iter().for_each(|f| {
            let convert: i32 = f.parse::<i32>().unwrap_or(default);
            new_vec.push(convert);
        });
        // Watch Star Wars Clone wars
        let mut new_series = Series::from(new_vec);
        new_series.set_name(self.headers[idx].as_str());
        new_series
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
                let mut series = Series::from(j.as_slice());
                series.set_name(header.as_str());
                df.add_series(series, true).unwrap();
            }
        }
        df
    }

    ///Pretty print data
    pub fn pretty_print(&self) {
        beautify(self.data.as_ref(), self.headers.as_ref())
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
    /// # Example
    /// ```
    ///    use crate::dami::io::csv::Builder as Builder;
    ///    fn main(){
    ///         let builder = Builder::new();
    ///         assert_eq!(builder.delimiter(),",") // Returns true
    /// }
    /// ```
    pub const fn delimiter(self) -> &'a str {
        self.delimiter
    }
    /// Set the delimiter of the Builder
    /// # Example
    /// ```
    ///use crate::dami::io::csv::Builder as Builder;
    ///
    /// fn main(){
    ///     let builder = Builder::new().set_delimiter(":").build();
    ///     assert_eq!(builder.delimiter(),":") // Returns true
    /// }
    /// ```
    pub fn set_delimiter(&mut self, delimiter: &'a str) -> &mut Self {
        self.delimiter = delimiter;
        self
    }
    /// Whether the CSV has headers
    ///  # Example
    /// ```
    /// use crate::dami::io::csv::Builder as Builder;
    ///
    /// fn main(){
    ///     let builder = Builder::new();
    ///     assert_eq!(builder.headers(),true) // Returns true
    /// }
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
    ///
    /// # Example
    /// ```
    /// use crate::dami::io::csv::Builder as Builder;
    ///
    /// fn main(){
    ///     let builder = Builder::new().set_headers(false).build();
    ///     assert_eq!(builder.headers(),false) // Returns false
    /// }
    /// ```
    pub fn set_headers(&mut self, value: bool) -> &mut Self {
        self.has_headers = value;
        self
    }
    ///Get the line terminator of the builder
    /// # Example
    /// ```
    /// use crate::dami::io::csv::Builder;
    /// fn main(){
    ///     let builder = Builder::new();
    ///     assert_eq!(builder.line_terminator(),"\n") // Returns true
    /// }
    /// ```
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
    /// # Example
    /// ```
    ///  use crate::dami::io::csv::Builder;
    ///  fn main(){
    ///     let bd = Builder::new();
    ///     assert_eq!(bd.flexible(),true); // Returns true
    /// }
    /// ```
    pub const fn flexible(&self) -> bool {
        self.flexible
    }
    /// Set the flexibility of the builder
    /// # Warning
    /// Please set this to be true if you are **sure** that the csv is well parsed
    /// Otherwise it will panic if it encounters a wrongly formatted record
    ///
    /// If set to true, the CSV parser will ignore any wrongly formatted records
    ///
    /// # Example
    /// ```
    /// use crate::dami::io::csv::Builder;
    /// fn main(){
    ///     let bd = Builder::new().set_flexible(false).build(); // Do not do this
    ///     assert_eq!(bd.flexible(),false) // May the universe be with you
    /// }
    /// ```
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
/// Beautify csv output
/// using [prettytable-rs](https://docs.rs/prettytable-rs/0.8.0/prettytable/)
#[allow(clippy::needless_range_loop)]
fn beautify(data: &[Vec<String>], headers: &[String]) {
    //Create a new table
    let mut table = Table::new();
    // If the first option is a title voila
    let items_len = data[0].len();
    let mut title = vec![];
    for i in headers {
        title.push(Cell::new(i).style_spec("bFG"))
    }
    table.set_titles(Row::new(title));
    // Add rows
    // counter should start from 1 since the first items were used as titles
    for i in 1..items_len {
        let mut final_ = Row::new(vec![]);
        for j in 0..data.len() {
            final_.add_cell(Cell::new(&data[j][i]))
        }
        table.add_row(final_);
    }
    table.set_format(*format::consts::FORMAT_BOX_CHARS);
    table.print_tty(true);
}

///Creates a dictionary of types of data in each column. If any
///column is of a single type (say, integers), *except* for the first
///row, then the first row is presumed to be labels. If the type
///can't be determined, it is assumed to be a string in which case
///we assume it has no headers :\
///Finally, a 'vote' is taken at the end for each column, adding or
///subtracting from the likelihood of the first row being a header.
///
/// # THIS FUNCTION IS NOT GUARANTEED TO WORK
#[allow(clippy::needless_range_loop, clippy::for_kv_map)]
fn has_header(sample: &[Vec<String>]) -> bool {
    // Assume first row is a header
    let mut header = vec![];
    for i in sample.to_owned() {
        header.push(i.get(0).unwrap().to_string())
    }
    let columns = sample.get(0).unwrap().len();
    let mut column_types: HashMap<usize, String> = HashMap::new();
    for i in 0..columns {
        column_types.insert(i, String::new());
    }

    let mut has_header = 0;
    let mut counter = 0;
    //arbitrary number of rows to check, to keep it sane
    for checked in 0..sample.len() {
        if counter > 20 {
            break;
        }

        for col in column_types.clone().keys() {
            counter += 1;
            let temp = &sample[checked][*col].to_string();
            let try_type: Result<String, ParseIntError> = {
                let try_parse = temp.parse::<i32>();
                match try_parse {
                    Ok(num) => Ok(format!("{}", num)),
                    Err(parse_int_error) => {
                        let try_parse = temp.parse::<f64>();
                        match try_parse {
                            Ok(num) => Ok(format!("{}", num)),
                            Err(_) => Err(parse_int_error),
                        }
                    }
                }
            };
            match try_type {
                Ok(type_) => {
                    column_types.insert(*col, type_);
                }
                Err(_) => continue,
            }
        }
    }
    let try_type: Vec<Result<String, ParseIntError>> = {
        let mut holder = vec![];
        for temp in header.clone() {
            let try_parse = temp.parse::<i32>();
            let a = match try_parse {
                Ok(num) => Ok(format!("{}", num)),
                Err(parse_int_error) => {
                    let try_parse: Result<f64, ParseFloatError> = temp.parse::<f64>();
                    match try_parse {
                        Ok(num) => Ok(format!("{}", num)),
                        Err(_) => Err(parse_int_error),
                    }
                }
            };
            holder.push(a);
        }
        holder
    };

    for (_, col_type) in &column_types {
        if col_type.is_empty() {
            has_header -= 1;
        } else {
            has_header += 1;
        }
    }
    for i in try_type {
        // If the first one is an int return false
        if i.is_ok() {
            return false;
        }
    }
    has_header > 0
}
fn smart_split(string: &str, split_at: &str, quote_char: &str) -> Vec<String> {
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
}

// Why this long :<|
