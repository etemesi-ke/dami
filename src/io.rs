//! Exports the io operations dami supports
//!
//! - [`clipboard`](clipboard/index.html):`(needs feature clipboard)` provides support for reading clipboard
//! and parsing `csv` data only (for the meantime) one day support for json may be included.
//! - [`csv`](csv/index.html):provides support for reading ad writing csv  data
//! - [`fwf`](fwf/index.html):provides support for reading fixed width files
//! - [`json`](json/index.html):provides support for  reading json formatted files
//! - [`utils`](utils/index.html):provides utilities used by the modules above like `read`

#[cfg(feature = "clipboard")]
pub mod clipboard;
mod csv;
pub mod excel;
mod fwf;
#[cfg(feature = "hdf5")]
pub mod hdf5;
mod json;
pub mod parser;
mod utils;

mod dtypes;
