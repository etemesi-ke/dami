//! Contains most used functions,traits and structs in dami

pub use crate::core::series::Series;

pub use crate::core::dataframe::DataFrame;

pub use crate::core::series::traits::{
    bigint::BigIntSeries,
    floats::SeriesFloat,
    ints::SeriesInt,
    strings::{Str, Strings},
};
pub use crate::io::parser::{read_csv, read_fwf};
pub use num_traits::float::Float;

#[cfg(feature = "clipboard")]
pub use crate::io::parser::read_clipboard;

#[cfg(feature = "hdf5")]
pub use crate::io::parser::read_hdf5_to_series;
