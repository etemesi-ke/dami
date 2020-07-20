//! Contains most used functions,traits and structs in dami

pub use crate::core::series::Series;

pub use crate::core::dataframe::DataFrame;
#[cfg(feature = "stats")]
pub use crate::core::series::Describe;
pub use crate::io::parser::{read_csv, read_fwf, read_json};
pub use num_traits::float::Float;

#[cfg(feature = "clipboard")]
pub use crate::io::parser::read_clipboard;

#[cfg(feature = "hdf5")]
pub use crate::io::parser::read_hdf5_to_series;
