#![allow(unused_imports)]

//! IO utilities
//!
//! This module provides common functions for IO operations like reading local
//! and remote files
//!
//! For remote binary files eg excel and compressed files
//! They are written to the system's temporary directory and then read from there.
extern crate lzma_rs;
#[cfg(feature = "remote")]
extern crate ureq;
#[cfg(feature = "remote")]
extern crate url;
extern crate zip;

use std::env::temp_dir;
use std::fmt;
use std::fmt::Formatter;
use std::fs::{File, OpenOptions};
use std::io::{BufReader, BufWriter, Read, Write};
use std::path::Path;
use std::str::FromStr;

use lzma_rs::{lzma_decompress, xz_decompress};
#[cfg(feature = "remote")]
use url::Url;
use zip::ZipArchive;

/// Main enum for Error types
enum IOError {
    /// Zip archives contain more than one file
    ZIPError(usize),
    /// The zip archive contains a directory
    DirectoryError,
}

impl<'a> std::fmt::Debug for IOError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match &self {
            Self::ZIPError(ref len)=>write!(f,"Zip archives should contain only one file\n {} files were found in the zip archive",len),
            Self::DirectoryError => write!(f,"Expected file, found directory in zip archive"),

        }
    }
}

/// Open a file and return the string representation of it
///
///Uses `BufReader` to speed up reading operation
/// # Arguments
/// * `path`: The path pointing to the file name to open
/// > `type`:Any thing that implements [`AsRef<Path>`] This includes String,&str,Path,OSStr e.t.c
/// # Warning
/// This reads to a string in memory therefore if used to read large chunks of file
/// You may run out of memory
///
/// [`AsRef<Path>`]: https://doc.rust-lang.org/std/convert/trait.AsRef.html
fn read_file<P: AsRef<Path> + fmt::Debug + Clone>(path: P) -> String {
    let mut temp = String::new();
    let fd =
        File::open(path.clone()).unwrap_or_else(|_| panic!("Could not open {:?}", path.as_ref()));
    let mut reader = BufReader::new(fd);
    reader.read_to_string(&mut temp).unwrap();
    temp
}
/// Fetch data from a remote url
///
/// This uses [`curl`](/curl/0.4.29/curl/) library to fetch data
///
/// # Arguments
/// * `url`: The remote website to fetch data
/// > `type`:[`&str`]
/// # Panics
/// - This function is safe when fetching [UTF-8](https://en.wikipedia.org/wiki/UTF-8) data
/// if data is in another encoding it is converted to � (UTF-8 replacement character)
/// - If the curl library fails for some reason
///
/// [`FromUtf8Error`]: /std/string/struct.FromUtf8Error.html
/// [`&str`]: https://doc.rust-lang.org/nightly/std/primitive.str.html
#[cfg(feature = "remote")]
fn get_remote(url: &str) -> String {
    ureq::get(url).call().into_string().unwrap()
}
/// Determines whether the resulting path is to be opened as a url or opened as a file
/// And calls the underlying function to fetch data
/// * `path`: The path to the file, can be a [URL](https://en.wikipedia.org/wiki/Uniform_Resource_Locator)
///    or a String pointing to a local file name
/// # Returns
/// * A [`String`] containing data in the file/url
/// # Panics
///  This function is safe when fetching [UTF-8](https://en.wikipedia.org/wiki/UTF-8) data
///  Passing binary data will cause it to PANIC and you will be presented with [`FromUtf8Error`]
///
/// [`FromUtf8Error`]: /std/string/struct.FromUtf8Error.html
#[allow(unreachable_code)]
pub fn read<P: AsRef<Path> + fmt::Debug + Clone>(path: P) -> String {
    // TODO: These functions are wrangled it would be nice if they were done better
    if is_url(path.as_ref().to_str().unwrap()) {
        #[cfg(feature = "remote")]
        if is_compressed(path.as_ref().to_str().unwrap()) {
            let path = write_remote_to_file(path.as_ref().to_str().unwrap());
            if path.ends_with(".zip") {
                return open_zip(path).unwrap();
            } else if path.ends_with(".lzma") || path.ends_with(".lzma2") || path.ends_with(".xz") {
                return open_lzma(path);
            } else {
                // TODO: Instead of panic see if we can use compile error marco here
                panic!("Remote feature not implemented cannot fetch remote files , enable it with feature=[\"remote\"] on \
                your Cargo.toml");
            }
        } else {
            return get_remote(path.as_ref().to_str().unwrap());
        }
        panic!("Remote feature not implemented cannot fetch remote files , enable it with feature=[\"remote\"] on \
                your Cargo.toml");
    } else if path.as_ref().to_str().unwrap().ends_with(".zip") {
        open_zip(path).unwrap()
    } else if path.as_ref().to_str().unwrap().ends_with(".lzma")
        || path.as_ref().to_str().unwrap().ends_with(".lzma2")
        || path.as_ref().to_str().unwrap().ends_with(".xz")
    {
        open_lzma(path)
    } else {
        read_file(path)
    }
}

///Open a [`LZMA`](https://en.wikipedia.org/wiki/LZMA) compressed file
/// # Arguments
/// * `file`:The path to the compressed file
/// # Panics
/// - If the file doesn't exist and if the file contains characters not in UTF-8
/// - If the underlying decompressor cannot decompress data
fn open_lzma<P: AsRef<Path> + fmt::Debug + Clone>(file: P) -> String {
    let mut decompose: Vec<u8> = Vec::new();
    let fd = File::open(file.as_ref())
        .unwrap_or_else(|e| panic!("Could not open {:?},reason {:?}", file.clone(), e));
    let mut fd = BufReader::new(fd);
    // LZMA version 1
    if file.as_ref().to_str().unwrap().ends_with(".lzma") {
        lzma_decompress(&mut fd, &mut decompose).unwrap();
    }
    // xz file
    else {
        xz_decompress(&mut fd, &mut decompose).unwrap();
    }
    String::from_utf8(decompose).unwrap()
}
/// Open a zip file
/// # Arguments
/// * `file`:The path to the compressed file
/// # Panics
/// - If the file does not exist
/// - If the zip reader cannot be initialized
/// - If there are multiple files in the zip archive
/// - If there is a directory in the zip archive
/// - If resulting data in the archive cannot be read to string
fn open_zip<P: AsRef<Path> + fmt::Debug + Clone>(file: P) -> Result<String, IOError> {
    let buf = File::open(file.as_ref()).unwrap();
    let mut temp = String::new();
    let fd = BufReader::new(buf);
    let mut zip = ZipArchive::new(fd).unwrap();
    if zip.len() != 1 {
        return Err(IOError::ZIPError(zip.len()));
    }
    let mut only_file = zip.by_index(0).unwrap();
    if only_file.is_dir() {
        return Err(IOError::DirectoryError);
    }
    only_file.read_to_string(&mut temp).unwrap();
    Ok(temp)
}
/// Read a remote filename into a temporary directory and return a string pointing to the path
///
/// This defers from [`read`] as it returns a path to the file and not the file itself
///
/// This is used in modules like [`excel`](/dami/io/excel/index.html) if the file is a remote url
#[cfg(feature = "remote")]
pub fn read_remote(url: &str) -> String {
    write_remote_to_file(url)
}
/// Write contents in a remote server to a file in the temporary directory
/// # Arguments:
/// - `url`: Url to fetch data from
/// # Returns
///  A string containing the path to the  temporary file
///  # Panics
///  - If the file cannot be created/opened
///  - If there is a problem with writing to the temporary file
/// -  If the transfer fails for some reason
#[cfg(feature = "remote")]
fn write_remote_to_file(url: &str) -> String {
    let mut temp_dir = temp_dir();
    let mut vec_ = Vec::new();
    let x = Url::from_str(url).unwrap();
    let extension: Vec<&str> = x.path().split('.').collect();
    temp_dir.push(format!(
        "dami_temp.{}",
        extension.get(extension.len()).unwrap_or(&"zip")
    ));
    let fd = OpenOptions::new()
        .create(true)
        .write(true)
        .open(temp_dir.clone())
        .unwrap_or_else(|_| panic!("Could not create {:?}", temp_dir));
    let mut fd = BufWriter::new(fd);
    let req = ureq::get(url).call();
    let mut reader = req.into_reader();
    reader
        .read_to_end(&mut vec_)
        .expect("Could not read to the buffer\n");
    fd.write_all(&vec_).unwrap();
    temp_dir.to_str().unwrap().to_string()
}

/// Check if the file is compressed
#[cfg(feature = "remote")]
fn is_compressed(file_name: &str) -> bool {
    let x = Url::from_str(file_name).unwrap();
    let path = x.path();
    if path.ends_with(".csv") || path.ends_with(".json") || path.ends_with(".html") {
        return false;
    }
    true
}
/// Check if the string is a url
fn is_url(path: &str) -> bool {
    if path.starts_with("http://")
        || path.starts_with("https://")
        || path.starts_with("file://")
        || path.starts_with("ftp://")
        || path.starts_with("ftps://")
    {
        return true;
    }
    false
}
