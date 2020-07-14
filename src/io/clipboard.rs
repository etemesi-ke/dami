#![cfg(feature="clipboard")]
//! Read data from the clipboard and pass it to [csv](/dami/io/csv/struct.Reader.html#method.parse_string_csv)
//!
//! This module requires crate [rust-clipboard](https://github.com/aweinstock314/rust-clipboard) to interact with the
//! system clipboard.
//!
//! This module also requires `feature=clipboard` on in order to use and can be disabled by compiling the library with
//! `features=['minimal']` option on Cargo dependencies
//!
//! Overall it adds about 2 seconds on compile time (debug profile)
//! # Notes
//! Requirements on:
//!
//! - Linux: `sudo  apt install xorg-dev libxcb-shape0-dev libxcb-xfixes0-dev`
extern crate clipboard;
use crate::core::series::Series;
use crate::io::csv::{series_to_csv, Reader};
use clipboard::{ClipboardContext, ClipboardProvider};
use std::collections::HashMap;
use std::fmt::Display;

#[derive(Clone, Debug)]
/// Read data from clipboard and pass it to `read_csv` method of csv module
pub struct ClipReader<'a> {
    data: String,
    csv_reader: Reader<'a>,
}
impl<'a> Default for ClipReader<'a> {
    fn default() -> ClipReader<'a> {
        ClipReader {
            data: String::new(),
            csv_reader: Reader::new(),
        }
    }
}
impl<'a> ClipReader<'a> {
    /// Create a new instance of Clipboard reader
    ///
    /// The Reader initializes also its own `CSVReader` from the csv module
    /// and whatever data is read from the clipboard is passed on to the underlying `CSVReader`.
    ///
    #[must_use]
    pub fn new() -> ClipReader<'a> {
        Self::default()
    }
    /// Read data from the clipboard
    ///
    /// # Arguments
    /// * `sep`:A string or regex delimiter. The default of `\s+` denotes
    /// one or more whitespace characters.
    /// > `type`: [`&str`]
    /// * `options`: A `HashMap` containing settings for the CSV Parser to consider when parsing the CSV
    /// since it is wrapped in an [`Option`] None is also valid, where the default settings will be used
    /// > `type`: [`Option<Hashmap>`]
    /// # Returns
    ///  [`Reader<'a>`](/dami/io/csv/struct.Reader.html)
    ///
    /// [`&str`]: /std/primitive.str.html
    /// [`Option`]: /std/option/enum.Option.html
    /// [`Option<Hashmap>`]: /std/option/enum.Option.html
    pub fn read(&mut self, sep: &'a str, options: Option<HashMap<&'a str, &'a str>>) -> Reader<'a> {
        let mut options = options.unwrap_or_default();
        options.insert("sep", sep);
        let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
        self.data.push_str(ctx.get_contents().unwrap().as_str());
        self.csv_reader.update_kwargs(options);
        self.csv_reader.parse_string_csv(&self.data)
    }

    /// Return the data read from the clipboard as a [`String`](https://doc.rust-lang.org/std/string/struct.String.html) type
    #[allow(clippy::must_use_candidate)]
    pub fn data(&self) -> String {
        self.data.clone()
    }
}
/// Write text data to the clipboard after parsing it
/// The data can then be pasted to Excel for example
///
/// # Arguments
/// * `obj`:to be written to the clipboard
///
/// > `type`: [`String`]: or any type that implements `into<String>`
///
/// * `sep`: Optional separator defaults to tab
///
///  > `type`: [`&str`]
///
/// *  `other` keywords are passed to `to_csv_writer` method of csv
///  > `type`: [`HashMap`]
///
/// # Warning ⚠️
///
/// This overwrites any data stored on the clipboard
///
/// [`&str`]: /std/primitive.str.html
/// [`String`]: /std/string/struct.String.html
/// [`HashMap`]: /std/collections/struct.HashMap.html
pub fn series_to_clipboard<T: Display + Clone + Default + 'static>(series: &Series<T>) {
    let mut buff = Vec::new();
    series_to_csv(series, &mut buff);
    // Call to_writer
    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
    // Set contents
    ctx.set_contents(String::from_utf8_lossy(&buff).to_string())
        .unwrap();
}
