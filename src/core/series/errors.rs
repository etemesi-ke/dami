//! This module contains main error types for Series
use std::fmt;
/// Errors Originating from various functions in Series
pub enum SeriesErrors {
    /// Matrix unaligned error
    MatrixUnaligned(usize, usize),
}

impl fmt::Debug for SeriesErrors {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MatrixUnaligned(ref me, other) => write!(
                f,
                "Matrices unaligned. Length for me {},length for other {}",
                me, other
            ),
        }
    }
}
