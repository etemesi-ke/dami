//! This module contains common enum types for the crate
//!
//! Te following enums are found here
//! * [`DataTypes`] : Contains the rust types officially supported by the crate
//!  *[`DataFrameErrors`] : Contains errors that may occur when parsing DataFrames
use std::fmt;
/// This enum contains officially supported types in the series and DataFrames
/// For unsupported types, they default to OBJECT variant here.
/// Such types wont benefit from some type specific functions eg Series.describe()
#[derive(Clone, Eq, PartialEq, Hash)]
pub enum DataTypes {
    ///[`i32`] type
    I32,
    ///[`i64`] type
    I64,
    ///[`f32`] type
    F32,
    ///[`f64`] type
    F64,
    ///[`String`] type
    STRING,
    ///[`str`] type
    STR,
    /// Boolean type
    BOOL,
    /// Any other type that is not supported
    OBJECT,
}

impl fmt::Debug for DataTypes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::F32 => write!(f, "f32"),
            Self::F64 => write!(f, "f64"),
            Self::I32 => write!(f, "i32"),
            Self::I64 => write!(f, "i64"),
            Self::STR => write!(f, "str"),
            Self::BOOL => write!(f, "bool"),
            Self::STRING => write!(f, "string"),
            Self::OBJECT => write!(f, "object"),
        }
    }
}
/// This provides Error methods for DataFrames
pub enum DataFrameErrors {
    /// A Series is being inserted into a DataFrame whose length is different
    DifferentLength(usize, usize),
    /// Conflicting column names
    ColumnNameErrors(String),
    /// Key  Errors
    KeyError(String),
}

impl fmt::Debug for DataFrameErrors {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DifferentLength(ref me, ref other) => write!(
                f,
                "Arrays must be of same \
            length. me {}. Other array {}",
                me, other
            ),
            Self::ColumnNameErrors(ref column) => write!(
                f,
                "Name {} already in DataFrame, consider changing the series name",
                column
            ),
            Self::KeyError(ref err) => write!(f, "{}", err),
        }
    }
}
