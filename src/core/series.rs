//! A one dimensional ndarray with axis labels
extern crate ndarray;

use crate::core::series::generic::create_index;
use ndarray::prelude::*;
use ndarray::Array1;
use std::any::Any;
use std::borrow::Borrow;
use std::collections::hash_map::RandomState;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::fmt;

use crate::enums::DataTypes;
use prettytable::{format::consts::FORMAT_CLEAN, Cell, Row, Table};
use std::ops::{Index, IndexMut};

mod impls;

mod generic;
mod ops;
#[cfg(feature = "stats")]
mod stats;

pub mod traits;

pub mod errors;

/// Error types for Series
pub enum Error {
    ///A series was initialized with a Hashmap containing more than one key/value
    HashMapError(usize),
    ///Duplicate labels
    LabelError,
}
impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::HashMapError(ref usize) => write!(
                f,
                "Expected a hashmap of length 1\n\
            Found a Hashmap of length {}",
                usize
            ),
            Self::LabelError => write!(f, "found duplicate labels"),
        }
    }
}

/// A one dimensional array with axis labels
///
///```text
/// __________________
/// |index  | elm    |
/// |-----------------
/// |index2 | elm2   |
/// ------------------
///```
/// The above shows the structure of a Series
///
/// An index is basically a vector of strings, each index points to the element in the array
/// and can be used for Indexing the array
///
/// The elements are stored in a one dimensional [ndarray] which supports slicing, splitting and other
/// cool stuff
///
/// To create a new Series, use  *from* methods currently supported ones are from a [`HashMap`] with a len of 1
/// a [`Vec`] and  a slice generic slice `T`
/// # Warning
/// In order to determine the dtype of the Array, we need the array to have at least one value of the series
///
/// When a series is built using `from` methods the methods will try to get the first element and determine its type
///supported types are [`f32`],[`f64`],[`i128`],[`i64`],[`i32`],[`String`] and [`str`]
///
/// # Methods
/// The methods listed below do not expose all functions for Series but expose global functions
/// (functions that do not depend on the Series type).
///
/// For localized functions. eg is_nan() which is implemented for floats only, check the documentation on traits module
///
/// [ndarray]: https://docs.rs/ndarray/
#[derive(Clone, Eq, PartialEq)]
pub struct Series<T: Sized> {
    array: Array1<T>,
    name: String,
    index: Vec<String>,
    dtype: DataTypes,
}
#[doc(hidden)]
impl<T: Clone + Any + Default> Default for Series<T> {
    fn default() -> Self {
        Self {
            array: arr1(&[]),
            name: "series".to_string(),
            index: Vec::new(),
            dtype: get_type(&T::default()),
        }
    }
}
#[doc(hidden)]
impl<T: Clone + 'static + Default> From<Vec<T>> for Series<T> {
    /// Create a series from a vector
    ///
    /// This takes a pointer to a vector and not the vector itself and returns a Series
    /// with the underlying array being a vector
    fn from(vector: Vec<T>) -> Self {
        let length = vector.len();
        let dtype = get_type(vector.get(0).unwrap_or(&T::default()));
        Self {
            array: Array1::from(vector),
            name: "series".to_string(),
            index: create_index(length, "", ""),
            dtype,
        }
    }
}
impl<T: 'static + Clone + Default> From<Array1<T>> for Series<T> {
    fn from(array: Array1<T>) -> Self {
        let len = array.len();
        let dtype = get_type(array.get(0).unwrap_or(&T::default()));
        Self {
            array,
            name: "series".to_string(),
            index: create_index(len, "", ""),
            dtype,
        }
    }
}
impl<T: 'static + Clone + Default> From<&[T]> for Series<T> {
    fn from(slice: &[T]) -> Self {
        let len = slice.len();
        let dtype = get_type(slice.get(0).unwrap_or(&T::default()));
        Self {
            array: arr1(slice),
            name: "series".to_string(),
            index: create_index(len, "", ""),
            dtype,
        }
    }
}
impl<T: 'static + Clone + Default> TryFrom<HashMap<String, Vec<T>>> for Series<T> {
    type Error = Error;
    /// Try creating an array from a Hashmap (Dictionary)
    ///
    /// # Panics
    ///if the length of the [`HashMap`] isn't one. since a series holds data for a one dimensional array
    /// we cannot create a Series from multiple hashmaps Consider using a DataFrame for such cases
    fn try_from(value: HashMap<String, Vec<T>, RandomState>) -> Result<Self, Self::Error> {
        if value.len() == 1 {
            let key = value.keys().next().unwrap().to_owned();
            // TODO: Maybe bind this with a default
            let value = value.values().next().unwrap();
            let dtype = get_type(value.get(0).unwrap_or(&T::default()));
            Ok(Self {
                name: key,
                array: arr1(value),
                index: create_index(value.len(), "", ""),
                dtype,
            })
        } else {
            Err(Error::HashMapError(value.len()))
        }
    }
}
#[doc(hidden)]
impl<T: Clone + 'static + Default> From<HashMap<&str, T>> for Series<T> {
    fn from(hashmap: HashMap<&str, T, RandomState>) -> Self {
        let index = hashmap
            .keys()
            .map(|f| (*f).to_string())
            .collect::<Vec<String>>();
        let mut array = Series::from(
            hashmap
                .values()
                .map(std::borrow::ToOwned::to_owned)
                .collect::<Array1<T>>(),
        );
        array.dtype = get_type(array.get(0).unwrap_or(&T::default()));
        // No need to verify index since HashMaps do not allow duplicate keys
        array.reindex(index, false);
        array
    }
}
impl<T: Clone + 'static + Default> From<Vec<(&str, T)>> for Series<T> {
    fn from(vector: Vec<(&str, T)>) -> Self {
        let names = vector
            .iter()
            .map(|f| f.0.to_owned())
            .collect::<Vec<String>>();
        let mut series = Series::from(vector.iter().map(|f| f.1.to_owned()).collect::<Vec<T>>());
        series.reindex(names, false);
        series
    }
}
impl<T: Clone + 'static + Default> From<Vec<(String, T)>> for Series<T> {
    fn from(vector: Vec<(String, T)>) -> Self {
        let names = vector
            .iter()
            .map(|f| f.0.to_owned())
            .collect::<Vec<String>>();
        let mut series = Series::from(vector.iter().map(|f| f.1.to_owned()).collect::<Vec<T>>());
        series.reindex(names, false);
        series
    }
}
#[doc(hidden)]
impl<T: 'static + fmt::Debug + Default + Clone> fmt::Debug for Series<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Ensure index row is equal to series
        debug_assert!(
            self.array.len() == self.index.len(),
            "Array length and index length are different"
        );
        let mut table = Table::new();
        table.set_format(*FORMAT_CLEAN);
        // Smaller arrays print everything
        if self.len() <= 10 {
            for (index, elm) in self.array.iter().enumerate() {
                let row = vec![
                    Cell::new(&self.index[index]),
                    Cell::new(&format!("{:>.4?}", elm)),
                ];
                table.add_row(Row::new(row));
            }
            table.add_empty_row();
            let final_row = vec![
                Cell::new(&format!("name:{}", self.name)),
                Cell::new(&format!("dtype:{:?}", self.dtype)),
            ];
            table.add_row(Row::new(final_row));
            table.insert_row(0, Row::new(vec![Cell::new("index"), Cell::new("values")]));
            write!(f, "{}", table.to_string())
        }
        // Larger arrays print first 5 add ... and print the last 5
        else {
            (0..5).for_each(|f| {
                let row = vec![
                    Cell::new(&self.index[f]),
                    Cell::new(&format!("{:.4?}", &self[f])),
                ];
                table.add_row(Row::new(row));
            });
            table.add_empty_row();
            let length = self.len() - 5;
            (0..5).for_each(|f| {
                let row = vec![
                    Cell::new(&self.index[length + f]),
                    Cell::new(&format!("{:.4?}", &self[length + f])),
                ];
                table.add_row(Row::new(row));
            });

            let final_row = vec![
                Cell::new(&format!("name:{}", self.name)),
                Cell::new(&format!("dtype:{:?}", self.dtype)),
                Cell::new(&format!("length:{:?}", self.len())),
            ];

            table.add_row(Row::new(final_row));
            table.insert_row(0, Row::new(vec![Cell::new("index"), Cell::new("values")]));
            write!(f, "{}", table.to_string())
        }
    }
}
#[doc(hidden)]
impl<T: Default> Index<&str> for Series<T> {
    type Output = T;
    /// Get the index at a particular column
    ///
    /// # Panics
    ///  If the item doesn't exist in the index
    fn index(&self, index: &str) -> &Self::Output {
        if self.index.contains(&index.to_string()) {
            self.array
                .get(self.index.iter().position(|x| index == x).unwrap())
                .unwrap()
        } else {
            panic!("The Series does not contain a value at label {}", index);
        }
    }
}
impl<T: Default> IndexMut<usize> for Series<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.array.index_mut(index)
    }
}
impl<T: Default> IndexMut<&str> for Series<T> {
    /// Get the index at a particular column
    ///
    /// # Panics
    ///  If the item doesn't exist in the index
    fn index_mut(&mut self, index: &str) -> &mut Self::Output {
        if self.index.contains(&index.to_string()) {
            self.array
                .index_mut(self.index.iter().position(|x| index == x).unwrap())
        } else {
            panic!("The Series does not contain a value at label {}", index);
        }
    }
}
impl<T: Clone + 'static + Default> IntoIterator for Series<T> {
    type Item = T;
    type IntoIter = SeriesIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        SeriesIter {
            series: self,
            index: 0,
        }
    }
}
/// Implements an iterator
#[doc(hidden)]
pub struct SeriesIter<T: Default> {
    series: Series<T>,
    index: usize,
}
impl<T: Clone + 'static + Default> Iterator for SeriesIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.series.array.get(self.index) {
            Some(arr) => {
                self.index += 1;
                Some(arr.clone())
            }
            None => std::option::Option::None,
        }
    }
}
#[doc(hidden)]
impl<T: Default> Index<usize> for Series<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        self.array[index].borrow()
    }
}
macro_rules! array_impl{
    ($($len:expr)+) => {
    $(  #[doc(hidden)]
        impl<T:Clone+?Sized+'static+Default> From<[T;$len]> for Series<T>{
            fn from(array:[T;$len])->Series<T>{
                let array = array.to_vec();
                let dtype = get_type(array.get(0).unwrap_or(&T::default()));
                Self {
                    array: Array1::from(array.to_vec()),
                    name: "series".to_string(),
                    index: create_index($len, "", ""),
                    dtype,
                }
            }
        }
    )+
   }
}
// thanks serde team :)
// WARN: Do not change this above 32, the Default impl will fail
array_impl! {1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25 26 27 28 29 30 31 32}

/// Get the type of an array
fn get_type<T: Any>(value: &T) -> DataTypes {
    let value_any = value as &dyn Any;
    if value_any.is::<f64>() {
        DataTypes::F64
    } else if value_any.is::<f32>() {
        DataTypes::F32
    } else if value_any.is::<i64>() {
        DataTypes::I64
    } else if value_any.is::<i32>() {
        DataTypes::I32
    } else if value_any.is::<String>() {
        DataTypes::STRING
    } else if value_any.is::<&str>() {
        DataTypes::STR
    } else if value_any.is::<bool>() {
        DataTypes::BOOL
    } else {
        DataTypes::OBJECT
    }
}
