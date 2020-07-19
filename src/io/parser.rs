//! Top level functions for IO
//!
//! This is the wrapper that provides high level abstractions to low level  functions
//! like reading and writing to different formats
//!
//! This module should be used instead of the underlying modules provided in th `io` crate
//! as it performs error checking conventions the the underlying modules do not consider
extern crate lazy_static;

use crate::io::csv::Reader;
use crate::io::fwf::FWFReader;
#[allow(unused_imports)]
use crate::prelude::Series;

use std::collections::{HashMap, HashSet};
use std::fmt;
use std::fmt::Debug;
use std::iter::FromIterator;
use std::path::Path;

use crate::core::dataframe::DataFrame;
#[cfg(feature = "clipboard")]
use crate::io::clipboard::ClipReader;
#[cfg(feature = "hdf5")]
use crate::io::hdf5::read_dataset_to_series;
use crate::io::json::JsonReader;
#[cfg(feature = "hdf5")]
use hdf5::H5Type;

lazy_static! {
    static ref CSV_PARSER_DEFAULTS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("delimiter", ",");
        m.insert("escape_char", "");
        m.insert("quoting", "\"");
        m.insert("doublequote", "true");
        m.insert("line_terminator", "\n");
        m.insert("header", "infer");
        m.insert("prefix", "");
        m.insert("skiprows", "");
        m.insert("skipfooter", "0");
        m.insert("nrows", "");
        m.insert("na_values", "");
        m.insert("true_values", "");
        m.insert("false_values", "");
        m.insert("thousands", "");
        m.insert("comment", "");
        m.insert("decimal", ".");
        m.insert("names", "");
        m.insert("skip_blank_lines", "true");
        m
    };
}
enum Errors {
    NameError,
}
impl fmt::Debug for Errors {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NameError => write!(f, "Duplicate names are not allowed."),
        }
    }
}

/// Read a CSV file/url and parse it
///
/// # Arguments
/// `path`: A local file or URL string pointing to a csv file.
/// Can be compressed or not
///
/// `options`: If present. Contains a key-value reference of the below options.
/// > * `delimiter`: The delimiter to use for separating CSV records defaults to `\n`.
/// > * `quoting`: The quote character in the CSV file defaults to `"`
/// > * `names`: A String containing comma-separated names to be used as the column names.
/// > * `prefix`: Prefix to add to column names.
///
/// If the above options do not suit the CSV file you are reading
/// ```ignore
/// read_csv("a_well_parsed_file.csv",None)
/// ```
/// should work for you
/// # Panics
/// * If the names argument in the `options` settings contains duplicates
/// # Notes
/// For local non-zipped files this function is much faster as it uses
/// buffered io to parse the files as it is being read.
///
/// For remote files/zipped files. The files are first read to memory before being parsed
pub fn read_csv<'a, P: AsRef<Path> + Debug + Clone>(
    path: P,
    options: Option<HashMap<&'a str, &'a str>>,
) -> DataFrame {
    let options = options.unwrap_or_default();
    let settings = update_kwargs(options);
    //Validate names
    validate_names(settings.get("names").unwrap()).unwrap();
    let mut new_reader = Reader::new();
    new_reader.parse_csv(path, settings)
}
/// Read a JSON file to a DataFrame.
///
/// # Arguments
/// `path`: A string pointing to either a local or a remote file which contains JSON data.
///
/// `lines` : Whether to read json a one json object per line.
///
/// # Notes
/// * DataTypes by default are inferred from the first 10 lines (or the length if its smaller than 10)
/// * Integer types are converted to [`i64`] and then cast to [`i32`] as this library has better support for i32
/// than i64's. This may lead to loss of precision for numbers greater or less than i64::MAX or i64::MIN respectively.
/// * Currently it does not support parsing of arrray-like json values
pub fn read_json<P: AsRef<Path> + Debug + Clone>(path_or_buffer: P, lines: bool) -> DataFrame {
    let mut reader = JsonReader::new();
    reader.read(path_or_buffer, lines);
    reader.to_dataframe()
}
/// Read a fixed width file
///
/// A fixed width file looks like this;
/// ```text
/// id8141    360.242940   149.910199   11950.7
/// id1594    444.953632   166.985655   11788.4
/// id1849    364.136849   183.628767   11806.2
/// id1230    413.836124   184.375703   11916.8
/// id1948    502.953953   173.237159   12468.3
/// ```
///  Where columns are separated by a fixed number od characters.
/// # Arguments
/// > * `path`: A string pointing to a fixed width file
/// > * colspecs: A Vec containing tuples of usize that indicates the column widths
/// > * `options`: A HashMap containing options. Currently this does nothing but the behavior will change in the future.
///
/// If the two latter functions are not needed for your case you can use:
/// ```ignore
/// read_fwf("a_well_parsed_file.csv",None,None);
/// ```
///
/// By Default leading and trailing whitespace characters will be `trim()`ed
///
pub fn read_fwf<'a, P: AsRef<Path> + Debug + Clone>(
    path: P,
    colspecs: Option<Vec<(usize, usize)>>,
    options: Option<HashMap<&'a str, &'a str>>,
) -> DataFrame {
    let options = options.unwrap_or_default();
    let settings = update_kwargs(options);
    if let Some(specs) = colspecs {
        let mut a = FWFReader::new();

        a.read_with_colspecs(path, &specs, settings).to_dataframe()
    } else {
        let mut a = FWFReader::new();
        a.read(path, settings).to_dataframe()
    }
}
#[cfg(feature = "clipboard")]
/// Requires feature
/// * `clipboard`
///
/// Read text from the system clipboard and pass it to `CSVReader`
/// # Arguments
/// >    `sep`:`A string used to separate record
///
/// >    `options`: See [`update_kwargs_method`](/dami/io/csv/struct.Reader.html#method.update_kwargs)
///
/// # Notes
///  On linux platforms, run
/// ```bash
/// sudo apt install xorg-dev libxcb-shape0-dev libxcb-xfixes0-de
/// ```
/// To install needed dependencies
pub fn read_clipboard<'a>(sep: &'a str, options: Option<HashMap<&'a str, &'a str>>) -> DataFrame {
    let mut clip_reader = ClipReader::new();
    let options = update_kwargs(options.unwrap_or_default());
    clip_reader.read(sep, Some(options)).to_dataframe()
}
/// Read a HDF5 dataSet to a dami [`Series`]
/// # Arguments
/// * `Generic` T: Which derives the [`Clone`] and  HDF5Type trait, for the latter
/// see the example at [hdf5](https://github.com/aldanor/hdf5-rust/blob/master/examples/simple.rs)
///
/// * `file`:`str` The string pointing to a HDF5 file in the local filesystem
///
/// * `dataset`:`str`: The dataset name to load
/// # Returns
/// [`Series`] with the underlying array as the dataset
/// # Panics
/// * If the file cannot be opened
///
/// * If the dataset is not a one dimensional array
///
/// * If the array cannot be converted into type `T`
#[cfg(feature = "hdf5")]
pub fn read_hdf5_to_series<T: Clone + H5Type + Default>(path: &str, dataset: &str) -> Series<T> {
    read_dataset_to_series(path, dataset)
}
/// Updates keyword arguments
///
/// It takes a `HashMap` and iterates over its values and updates the default
/// csv parser values of corresponding keys with the new specified values
fn update_kwargs<'a>(kwargs: HashMap<&'a str, &'a str>) -> HashMap<&'a str, &'a str> {
    let mut settings: HashMap<&str, &str> = CSV_PARSER_DEFAULTS.clone().to_owned();
    for (key, value) in kwargs {
        let val = settings.clone();
        let val = val.get(key);
        match val {
            None => continue,
            Some(_) => {
                settings.insert(key, value);
            }
        }
    }
    settings
}
///Check to ensure there are no duplicates in names
fn validate_names(names: &str) -> Result<(), Errors> {
    let broken_names = names
        .split(',')
        .collect::<Vec<&str>>()
        .into_iter()
        .map(std::string::ToString::to_string)
        .collect::<Vec<String>>();
    let set: HashSet<String> = HashSet::from_iter(broken_names.clone());
    if set.len() != broken_names.len() {
        return Err(Errors::NameError);
    }
    Ok(())
}
