//! This module contains the DataFrame Struct and its associated capabilities
use crate::enums::{DataFrameErrors, DataTypes};
use crate::prelude::*;
use baggie::Baggie;
use ndarray::{Array1, Array2};
use prettytable::format::consts::FORMAT_CLEAN;
use prettytable::{Cell, Row, Table};
use std::collections::hash_map::RandomState;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::fmt;
use textwrap::fill;

mod generics;
mod ints;
mod ops;
mod stats;
/// A container for heterogeneous data
///
/// A DataFrame is a special structure that can hold heterogeneous series types and
/// supporting higher level functions for series
///
/// Data is stored in a row-column format.
/// ```text
/// | index| col1| col2|....|coln|
/// |------|-----|-----|    |----|
/// |0     |val1 |val1 |    |val1|
/// |1     |val2 |val2 |    |val2|
/// |------|-----|-----|    |----|
/// |n     |valn |valn |    |valn|
/// ```
/// A column represents a series while the first column contains the index which *currently has no use*
///
///  # Limitations
/// * ## Some Functions require explicit types to be specified
/// > Since the DataFrame can hold heterogeneous data **AND** the Rust compiler needs to know
/// the type of Data is contained in the Series a lot of pattern matching goes on under the hood.
/// (Sometimes that's the only thing that goes on).
/// The DataFrame will hold any type of Series<T> but operations on `f64`,`f32` and `i32` are the ones fully supported
/// with partial support for `i64`, `i128`, `str` and `String` types.
/// > To apply certain functions in the DataFrame eg square all floats using apply the turbofish syntax is needed
/// ```ignore
/// df.apply::<f64,_>(|f| f.powi(2)) //Square all floats in the dataframe
/// ```
///
/// * ## Implicit conversion of Data may occur at times for certain operations
/// > On computational functions like `covariance` or `kurtosis()`, `f32` and `i32` Series types are converted to `f64`
/// and then the corresponding computation function is invoked, the rest types are ignored
///
/// * ## DataFrames cannot be indexed
/// ```python
/// df["new_col"]=df["col1"]+df["col2"]
/// ```
///> might be something that is possible in Python.
///
///> But not in Rust as Rust cannot determine during compilation
/// the type `new_col` or what `col1` or `col2` is, so here to implement such an operation you might do something like this
/// ```ignore
/// let added_col = df.get_series::<i32>("col1").unwrap()+df.get_series::<i32>("col2").unwrap();
/// df.add_col("new_col",added_col);
/// ```
pub struct DataFrame {
    container: Baggie<String>,
    index: Vec<String>,
    // Keep a track on orders. As HashMaps do not maintain insertion order so sometimes names may be
    // printed wrongly.
    order: Vec<String>,
    len: usize,
    dtypes: HashMap<String, DataTypes>,
}

impl fmt::Debug for DataFrame {
    #[allow(clippy::match_on_vec_items, clippy::too_many_lines)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Defaults to 80 if not determined
        let mut table = Table::new();
        // Use clean format
        table.set_format(*FORMAT_CLEAN);
        let mut title = vec![Cell::new("  ")];
        for i in &self.order {
            title.push(Cell::new(i))
        }
        let fields = self.order.len();
        table.set_titles(Row::new(title));
        // Thanks marcos
        macro_rules! print_frame {
            ($length:ident) => {
                let mut new_row = vec![];
                for item in self.order.iter() {
                    match self.dtypes.get(item).unwrap() {
                        DataTypes::STRING => {
                            let val = fill(
                                &format!(
                                    "{:?}",
                                    self.container
                                        .get::<Series<String>, _>(item.as_str())
                                        .unwrap()[$length]
                                ),
                                200 / fields,
                            );
                            new_row.push(Cell::new(&val))
                        }
                        DataTypes::I32 => {
                            let val = format!(
                                "{:?}",
                                self.container.get::<Series<i32>, _>(item.as_str()).unwrap()
                                    [$length]
                            );
                            new_row.push(Cell::new(&val))
                        }
                        DataTypes::I64 => {
                            let val = format!(
                                "{:?}",
                                self.container.get::<Series<i64>, _>(item.as_str()).unwrap()
                                    [$length]
                            );
                            new_row.push(Cell::new(&val))
                        }
                        DataTypes::I128 => {
                            let val = format!(
                                "{:?}",
                                self.container
                                    .get::<Series<i128>, _>(item.as_str())
                                    .unwrap()[$length]
                            );
                            new_row.push(Cell::new(&val))
                        }
                        DataTypes::F32 => {
                            let val = format!(
                                "{:0<3.3}",
                                self.container.get::<Series<f32>, _>(item.as_str()).unwrap()
                                    [$length]
                            );
                            new_row.push(Cell::new(&val))
                        }
                        DataTypes::F64 => {
                            let val = format!(
                                "{:0<3.3}",
                                self.container.get::<Series<f64>, _>(item.as_str()).unwrap()
                                    [$length]
                            );
                            new_row.push(Cell::new(&val))
                        }
                        DataTypes::BOOL => {
                            let val = format!(
                                "{:0<3.3}",
                                self.container
                                    .get::<Series<bool>, _>(item.as_str())
                                    .unwrap()[$length]
                            );
                            new_row.push(Cell::new(&val))
                        }
                        DataTypes::STR => {
                            let val = format!(
                                "{:?}",
                                self.container
                                    .get::<Series<&'static str>, _>(item.as_str())
                                    .unwrap()[$length]
                            );
                            new_row.push(Cell::new(&val))
                        }
                        // We don't know you ..
                        DataTypes::OBJECT => continue,
                    }
                }
                new_row.insert(0, Cell::new(&self.index[$length]));
                table.add_row(Row::new(new_row));
            };
        }
        if self.len <= 10 {
            for i in 0..self.len {
                print_frame!(i);
            }
        } else {
            for i in 0..5 {
                print_frame!(i);
            }
            let length = self.len - 5;
            // Add empty row to separate the first n and the last N items
            table.add_empty_row();
            for i in 0..5 {
                let ln = i + length;
                print_frame!(ln);
            }
        }
        write!(f, "{}", table.to_string())
    }
}

impl Default for DataFrame {
    fn default() -> Self {
        Self::new()
    }
}
impl DataFrame {
    /// Create a new DataFrame
    pub fn new() -> DataFrame {
        DataFrame {
            container: Baggie::default(),
            index: Vec::new(),
            order: Vec::new(),
            len: 0,
            dtypes: HashMap::new(),
        }
    }
    /// Add a new series to a DataFrame
    ///
    /// The series to be added should have the same length as the DataFrame
    ///
    /// Any type of a series can be added, but supported types are [`f32`], [`f64`],[`i32`],[`i64`]
    /// [`i128`],[`String`] and [`str`] type Series. Other series will be added. And will be in memory
    /// but will not be printed or seen or any calculations carried out on them but can be fetched and indexed
    ///
    /// # Errors
    /// * `DifferentLength` : The series to be added has a different length with the corresponding dataframe
    /// * `ColumnNameError`: The series contains two similar column types
    /// >  Note: first if two columns conflict. We will append a number to it corresponding to the number of items
    /// so `series` will become `series2` if there is already a series present if there is a conflicting name still
    /// the above error is raised
    /// # Example
    /// ````
    /// use crate::dami::prelude::DataFrame;
    /// use dami::core::series::Series;
    /// fn main(){
    ///     // Create a new DataFrame
    ///     let mut df = DataFrame::new();
    ///     let s1=df.add_series(Series::from([1,2,3,4,6,9,8,4,2]),true);
    ///     assert_eq!(s1.is_ok(),true);
    ///     // Try adding a smaller series. This returns an error
    ///     let s2=df.add_series(Series::from([1]),true);
    ///     assert!(s2.is_err());
    /// }
    /// ````
    pub fn add_series<T: Default + 'static + Clone>(
        &mut self,
        other: Series<T>,
        preserve_names: bool,
    ) -> Result<(), DataFrameErrors> {
        if !self.container.is_empty() && other.len() != self.len {
            return Err(DataFrameErrors::DifferentLength(other.len(), self.len));
        }
        if self.container.is_empty() {
            self.len = other.len();
            self.index = other.get_index();
        }
        // Clone names which become the header
        let mut new_name = other.get_name();

        if self.container.contains_key(new_name.as_str()) || !preserve_names {
            new_name = format!("{}", self.container.len());
            // Check now there isn't naming conflicts if still, raise an error
            if self.container.contains_key(new_name.as_str()) {
                return Err(DataFrameErrors::ColumnNameErrors(new_name));
            }
        }
        // DataType
        let new_type = other.get_dtype();
        self.order.push(new_name.clone());
        self.container.insert(new_name.clone(), other);
        self.dtypes.insert(new_name, new_type);
        Ok(())
    }
    /// Create a new column with the given name
    pub fn add_col<T: Default + 'static + Clone>(
        &mut self,
        name: &str,
        col: Series<T>,
    ) -> Result<(), DataFrameErrors> {
        let mut series = col;
        series.set_name(name);
        self.add_series(series, true)
    }
    /// # Requires Feature
    ///  > * `stats`
    ///
    /// Generate descriptive characteristics of an f64,f32 or i32 Series' in the DataFrame
    ///
    /// These includes those that summarize central tendency, dispersion and shape
    ///
    /// NAN values are by default going to be skipped
    /// # For Numeric Data
    /// The results index will include `count`,`mean`,`std`,`pstdev`,`min`,`max` as well as lower, 50 and upper
    /// percentiles
    ///
    /// For more information. See the documentation on the function [describe] in SeriesFloat Trait
    /// # Returns
    ///  A DataFrame containing descriptive characteristics. If no float or i32 Series exists returns an empty DataFrame
    ///
    /// [describe]: /dami/core/series/traits/floats/trait.SeriesFloat.html#tymethod.describe
    #[cfg(feature = "stats")]
    pub fn describe(&self) -> DataFrame {
        let mut frame = DataFrame::new();
        for i in &self.order {
            let dtype = self.dtypes.get(i).unwrap();
            match dtype {
                DataTypes::F64 => {
                    let series = self.container.get::<Series<f64>, _>(i).unwrap();
                    frame.add_series(series.describe(), true).unwrap();
                }
                DataTypes::F32 => {
                    let series = self.container.get::<Series<f32>, _>(i).unwrap();
                    frame.add_series(series.describe(), true).unwrap();
                }
                DataTypes::I32 => {
                    let series = self.container.get::<Series<i32>, _>(i).unwrap();
                    frame.add_series(series.describe(), true).unwrap();
                }
                _ => continue,
            }
        }
        frame
    }
    ///  Generate descriptive characteristics for String and str DataTypes in the DataFrame
    ///
    /// The resulting DataFrame will contain a series with `count`, `top`, `freq` and `unique`
    /// For more. See [describe]
    ///
    /// [describe]: /dami/core/series/traits/strings/trait.Strings.html#tymethod.describe
    pub fn describe_str(&self) -> DataFrame {
        let mut frame = DataFrame::new();
        for i in self.container.keys() {
            let dtype = self.dtypes.get(i).unwrap();
            match dtype {
                DataTypes::STRING => {
                    let series = self.container.get::<Series<String>, _>(i).unwrap();
                    frame.add_series(series.describe(), true).unwrap();
                }
                DataTypes::STR => {
                    let series = self.container.get::<Series<&'static str>, _>(i).unwrap();
                    frame.add_series(series.describe(), true).unwrap();
                }
                _ => continue,
            }
        }
        frame
    }
    /// Get the Data types for the series
    pub fn dtypes(&self) -> HashMap<String, DataTypes> {
        self.dtypes.clone()
    }
}

#[allow(clippy::fallible_impl_from)]
impl<T: Default + 'static + Clone> From<Array2<T>> for DataFrame {
    fn from(array: Array2<T>) -> Self {
        // Get dimensions
        let (rows, cols) = array.dim();
        let mut frame = DataFrame::new();
        // This is fairly complex;
        for i in 1..=cols {
            // The iterator starts at 1 and goes to n+1 for slicing
            let mut one_col = Series::from(
                array
                    // take one row from zero element to its dim
                    // One read only for column n
                    .slice(s![0..rows, i - 1..i])
                    .iter()
                    .map(std::borrow::ToOwned::to_owned)
                    .collect::<Array1<T>>(),
            );
            one_col.set_name(&format!("{}", i - 1));
            frame
                .add_series(one_col, true)
                .expect("Could not add series");
        }
        // Basically what the above  code does is
        // [[1,2],
        // [3.4] ] . Fetch one and three and turn it to a 1dim array and build a series.
        // then fetch 2, 4 and turn it into a 1 dim array.
        // Then build a series
        frame
    }
}
impl<T: Default + 'static + Clone> From<Vec<Vec<T>>> for DataFrame {
    #[allow(clippy::needless_range_loop)]
    fn from(array: Vec<Vec<T>>) -> Self {
        let mut frame = DataFrame::new();
        // columns are , elm at 1, at 2 so on
        let rows = array.len();
        let cols = array[0].len();
        for i in 0..cols {
            let mut arr = vec![];
            for j in 0..rows {
                let val = &array[j][i];
                arr.push(val.to_owned());
            }
            frame
                .add_series(Series::from(arr), false)
                .expect("Could not add series");
        }
        frame
    }
}

impl<T: Default + 'static + Clone> TryFrom<HashMap<&str, Vec<T>>> for DataFrame {
    type Error = DataFrameErrors;

    fn try_from(value: HashMap<&str, Vec<T>, RandomState>) -> Result<Self, Self::Error> {
        let mut frame = DataFrame::new();
        let mut keys = value.keys().map(|f| f.to_owned()).collect::<Vec<&str>>();
        keys.sort();
        for i in keys {
            let mut series = Series::from(value.get(i).unwrap().to_owned());
            series.set_name(i);
            match frame.add_series(series, true) {
                Ok(_) => continue,
                Err(e) => return Err(e),
            }
        }
        Ok(frame)
    }
}

impl<T: Default + 'static + Clone> TryFrom<HashMap<&str, Array1<T>>> for DataFrame {
    type Error = DataFrameErrors;

    fn try_from(value: HashMap<&str, Array1<T>, RandomState>) -> Result<Self, Self::Error> {
        let mut frame = DataFrame::new();
        let mut keys = value.keys().map(|f| f.to_owned()).collect::<Vec<&str>>();
        keys.sort();
        for i in keys {
            let mut series = Series::from(value.get(i).unwrap().to_owned());
            series.set_name(i);
            match frame.add_series(series, true) {
                Ok(_) => continue,
                Err(e) => return Err(e),
            }
        }
        Ok(frame)
    }
}
