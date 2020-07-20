use crate::core::series::{get_type, Series};

use crate::core::series::Error;

use super::ndarray::arr1;
use ndarray::Array1;

use std::collections::HashSet;

use std::hash::Hash;

use std::iter::FromIterator;
use std::iter::Iterator;
use std::ops::Index;

use crate::enums::DataTypes;
use prettytable::format::consts::FORMAT_CLEAN;
use prettytable::{Cell, Row, Table};
#[cfg(feature = "regex")]
use regex::Regex;
use std::fmt::Display;

impl<T: Clone + 'static + Default> Series<T> {
    /// Prefix labels with string *prefix*.
    ///
    /// This modifies the row label for series
    /// # Arguments
    ///  `prefix`: [`str`]
    /// > The string to add before each label
    ///
    /// This does not return a new Series but actually modifies the current series' index
    ///
    /// See also [`add_suffix`](#method.add_suffix)
    pub fn add_prefix(&mut self, prefix: &str) {
        self.index = create_index(self.len(), prefix, "");
    }
    /// Suffix labels with string *suffix*.
    /// # Arguments
    /// `suffix`:[`str`]
    /// > The string to add after each label
    ///
    /// Like [`add_prefix`](#method.add_prefix) this doesn't return a
    /// new series but modifies the current series
    ///
    /// See also [`add_prefix`](#method.add_prefix)
    pub fn add_suffix(&mut self, suffix: &str) {
        self.index = create_index(self.len(), "", suffix);
    }
    /// Concatenate two series
    /// # Arguments
    /// `other`: [`Series`]
    /// > Series to append to Self
    ///
    /// `ignore_index`:[`bool`]
    /// > If true do not use index labels.
    ///
    /// `verify_integrity`:[`bool`]
    /// > If True, Raise an exception on creating index with duplicates
    /// # Example
    /// ```
    /// use crate::dami::core::series::Series;
    /// fn main(){
    /// let mut series1 = Series::from(vec![1,2,3]);
    ///     let series2 = Series::from(vec![4,5,6]);
    ///     let mut series3 = Series::from(vec![1,2,3,4,5,6]);
    ///     // We have to change the index since the append operation does not validate indexes
    ///     series3.reindex(vec!["0","1","2","0","1","2"],false);
    ///     // Ignore index and do not validate integrity
    ///     series1.append(series2,false,true);
    ///     assert_eq!(series1,series3);
    /// }
    /// ```
    /// # Warning
    /// Appending data is an expensive operation as ndarray does not support extending arrays at the moment
    ///
    /// See [this issue](https://github.com/rust-ndarray/ndarray/issues/433) so what internally happens is
    /// that we create a whole new array and overwrite the existing one with the new one
    ///
    /// This can be memory intensive leading to slow programs. I recommend not doing this
    pub fn append(&mut self, other: Self, ignore_index: bool, verify_integrity: bool) {
        let a = self.index.clone();
        if verify_integrity {
            validate_names(a, other.index.clone()).unwrap();
        }

        if ignore_index {
            let mut new_array = self.array.to_vec();
            other
                .array
                .iter()
                .for_each(|f| new_array.push(f.to_owned()));
            self.array = Array1::from(new_array);
            other.index.into_iter().for_each(|f| self.index.push(f))
        } else {
            let mut new_array = self.array.to_vec();
            other
                .array
                .iter()
                .for_each(|f| new_array.push(f.to_owned()));
            let mut names = self.index.clone();
            other.index.into_iter().for_each(|f| names.push(f));
            self.array = Array1::from(new_array);
            self.index = names;
        }
    }
    /// Apply a function to a series
    ///
    /// The function takes a value T and returns a new value T which is used to create
    /// a new series
    ///
    /// # Example
    /// ```
    /// use dami::prelude::Series;
    /// use num_traits::Float;
    /// fn square<T:Float>(x:T)->T{
    ///     // Calculate the square
    ///     x.powi(2)
    /// }
    /// fn main(){
    ///     let series = Series::from([1.,2.,3.]);
    ///     // Also accepts closures
    ///     let squared = series.apply(square);
    ///     assert_eq!(squared,Series::from([1.,4.,9.]));
    /// }
    /// ```
    pub fn apply<F: Fn(T) -> T>(&self, func: F) -> Series<T> {
        let mut series = Series::from(self.array.mapv(|f| func(f)));
        series.name = self.name.clone();
        series
    }
    /// Apply a function to a series inplace
    ///
    /// The function takes a value T and returns a new value T which is used to as the value of the new series
    ///
    /// # Note
    /// This does not retrn a new series. For that use the `apply` method
    ///
    /// # Example
    /// ```
    /// use dami::prelude::Series;
    /// use num_traits::Float;
    /// fn square<T:Float>(x:T)->T{
    ///     // Calculate the square
    ///     x.powi(2)
    /// }
    /// fn main(){
    ///     let mut  series  = Series::from([1.,2.,3.]);
    ///     // Also accepts closures
    ///     series.apply_inplace(square);
    ///     assert_eq!(series,Series::from([1.,4.,9.]));
    /// }
    /// ```
    pub fn apply_inplace<F: Fn(T) -> T>(&mut self, func: F)
    where
        T: Send + Sync,
        F: Sync,
    {
        self.array.mapv_inplace(|f| func(f))
    }
    /// Convert a series to another Series type
    ///
    /// This can be used to cast a series of `i32's` array to an `i64` Series
    ///
    /// Explicit type parameters are needed for the casting. The type to be cast into should implement the
    /// [`From`] trait
    ///  # Example
    /// ```
    /// use crate::dami::core::series::Series;
    /// fn main(){
    ///     let series = Series::from(vec![0,1,0,1,0,1]);
    ///     let new_series:Series<f64>=series.as_type();
    ///     assert_eq!(new_series,Series::from(vec![0.0,1.0,0.0,1.0,0.0,1.0]));
    /// }
    /// ```
    /// # Panics
    ///  If the [`From`] trait is not implemented for The type conversion
    pub fn as_type<P: From<T> + Clone + 'static + Default>(&self) -> Series<P> {
        let mut series = Series::from(self.array.mapv(|f| f.into()));
        let new_type = series.array.get(0).unwrap();
        series.dtype = get_type(new_type);
        series
    }
    /// Access an item at label `index`
    ///
    /// # Panics
    ///  If the index does not exist in the Series
    pub fn at(&self, index: &str) -> T {
        self[index].clone()
    }
    /// Change the whole index of a Series
    ///
    /// In debug mode, this tries to assert that the current index and the new index are equal in length
    /// this check is dropped in release mode to give a boost to speed
    ///
    /// # Arguments
    /// * `new_index`:A vec containing elements that support  [`Into<String>`]
    /// * `verify_integrity`:`bool` Confirm whether they're no duplicates in the `new_index` argument
    ///
    /// # Panics
    /// * If in debug mode and old index length and new index length are not equal
    ///
    /// * If verify integrity is set to true and they're duplicates in the index
    pub fn reindex<P: Into<String>>(&mut self, new_index: Vec<P>, verify_integrity: bool) {
        debug_assert_eq!(new_index.len(), self.array.len());
        let sanitized_vec = new_index
            .into_iter()
            .map(std::convert::Into::into)
            .collect::<Vec<String>>();
        if verify_integrity {
            validate_names(self.index.clone(), sanitized_vec.clone()).unwrap();
        }
        self.index = sanitized_vec
    }
    ///Combine the series and another using function `func` to perform elementwise selection for
    ///combined series
    /// # Arguments
    /// `other`: A [`Series`] whose length and type is equal to `self`.
    ///  func: An `FnMut` instance which accepts two arguments T and returns one argument back
    /// an example is [max] or [min]
    /// # Panics
    /// If the self array and other array have differrent lengths
    /// # Example
    /// ```
    /// use crate::dami::core::series::Series;
    /// use std::cmp::max;
    /// fn main(){
    ///
    ///     let series = Series::from(vec![1,2,3,4,5,6]);
    ///     let series_c = series.combine(&series,max);
    ///     // This becomes true since max of 2,2 is 2 so no change in the array. But still pretty cool
    ///     assert_eq!(series_c,series);
    /// }
    /// ```
    /// [max]: /std/cmp/fn.max.html
    /// [min]: /std/cmp/fn.min.html
    pub fn combine<F: FnMut(T, T) -> T>(&self, other: &Series<T>, mut func: F) -> Series<T> {
        // Lengths should be equal
        // TODO: Allow for series with non-equal lengths to be combined
        debug_assert_eq!(
            self.len(),
            other.len(),
            "self length and other length are diff,self:{},other:{}",
            self.len(),
            other.len()
        );
        Series::from(
            self.array
                .iter()
                .enumerate()
                .map(|(len, f)| func(f.to_owned(), other.array[len].clone()))
                .collect::<Array1<T>>(),
        )
    }
    /// Return a Series with specific index removed.
    ///
    /// This remove elements of a series based on the index label
    ///
    /// # Arguments
    /// `labels`: `Vec<&str>` index label to drop
    ///
    /// # Example
    /// ```
    /// use dami::core::series::Series;
    /// fn main(){
    ///     let series = Series::from([1,2,3,4,5]);
    ///     let dropped = series.drop(&["0","1","2"]);
    ///     println!("{:?}",dropped);
    /// }
    /// ```
    /// Prints
    /// ```text
    /// 3   4
    /// 4   5
    ///
    /// name: series, dtype:i32
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    pub fn drop(&self, labels: &[&str]) -> Series<T> {
        let (retained, names) = self.drop_(labels.as_ref());
        let mut series = Series::from(retained);
        series.reindex(names, false);
        series
    }
    /// Like [drop](#method.drop) but actually modifies the current series and index and does not return a
    /// new series
    #[allow(clippy::needless_pass_by_value)]
    pub fn drop_inplace(&mut self, labels: &[&str]) {
        let (retained, names) = self.drop_(labels.as_ref());
        self.drop_array(arr1(&retained));
        self.index = names
    }
    fn drop_(&self, labels: &[&str]) -> (Vec<T>, Vec<String>) {
        let mut retained = vec![];
        let mut names = vec![];
        let me_clone = self.index.clone();
        me_clone.iter().for_each(|f|
            // If its not in the labels let it remain
            if !labels.contains(&f.as_str()){
                retained.push(self.index(f.as_str()).to_owned());
                names.push(f.to_string());
            });
        (retained, names)
    }
    fn drop_array(&mut self, new_arr: Array1<T>) {
        self.array = new_arr
    }

    /// Indicate duplicated values in series
    ///
    /// Duplicated values are indicated as true in the resulting Series
    /// # Returns
    ///  [`Series`] made of `bool` values indicating whether the item at `I` in the old series is
    /// a duplicate or not
    /// # Example
    /// ```
    /// use crate::dami::core::series::Series;
    /// fn main(){
    ///     let dup_series=Series::from(vec![1,2,1,2,1]);
    ///     let find_dup = dup_series.duplicated();
    ///     assert_eq!(find_dup,Series::from(vec![false,false,true,true,true]))
    /// }
    /// ```
    pub fn duplicated(&self) -> Series<bool>
    where
        T: Eq + Hash,
    {
        let mut temp_map = HashSet::new();
        let mut new_values = vec![];
        for i in self.array.to_vec() {
            if temp_map.contains(&i) {
                new_values.push(true)
            } else {
                temp_map.insert(i);
                new_values.push(false)
            }
        }
        Series::from(new_values)
    }
    /// Test whether two series contain the same elements
    ///
    /// # Arguments
    ///  `other`: A [`Series`] whose length and type are similar to self
    ///
    /// # Panics
    /// * In `debug mode` If the series have different lengths
    pub fn equals(&self, other: &Self) -> bool
    where
        T: Clone + PartialEq,
    {
        debug_assert_eq!(
            self.len(),
            other.len(),
            "Self length and other length are not equal self=>{},other=>{}",
            self.len(),
            other.len()
        );
        for (len, i) in self.array.into_iter().enumerate() {
            if *i == other[len] {
                return false;
            }
        }
        true
    }
    /// Filter the series using a function
    ///
    /// The function accepts a String reference and returns a true or false.
    ///
    /// If it returns true, the value at the index is put in the returned series otherwise its dropped
    ///
    /// # Example
    /// ```
    /// use crate::dami::prelude::Series;
    /// use std::collections::HashMap;
    /// fn main(){
    ///     let mut values= HashMap::new();
    ///     values.insert("first_idx",1);
    ///     values.insert("random_idx",3);
    ///     values.insert("next",4);
    ///     let series = Series::from(values);
    ///     // Use a closure to filter
    ///     let filtered=series.filter_by_func(|f|f.starts_with("d"));
    ///     // Series should be empty
    ///     assert!(filtered.is_empty());
    /// }
    /// ```
    pub fn filter_by_func<F: Fn(&String) -> bool>(&self, func: F) -> Series<T> {
        let mut items: Vec<T> = vec![];
        let mut names = vec![];
        for idx in self.index.clone() {
            if func(&idx) {
                items.push(self[idx.as_str()].clone());
                names.push(idx);
            }
        }
        let mut series = Series::from(items);
        series.reindex(names, false);
        series
    }
    /// Filter the series using a regex string to obtain rows
    ///
    /// The filter is carried out on the index labels and not on the series itself
    /// # Returns
    ///  A Series<T>
    ///
    /// # Example
    /// ```
    /// use dami::prelude::Series;
    /// fn main(){
    ///     let series = Series::from([1,2,3,4,5]);
    ///     // Match only the row 2-4
    ///     let new=series.filter_by_regex("1|2|3");
    ///
    ///     let mut proof = Series::from([2,3,4]);
    ///     proof.reindex(vec!["1","2","3"],false);
    ///     assert_eq!(new,proof);
    /// }
    /// ```
    #[cfg(feature = "regex")]
    pub fn filter_by_regex(&self, regex: &str) -> Series<T> {
        let regex = Regex::new(regex).expect("Could not use regex filter");
        let mut items: Vec<T> = vec![];
        let mut names = vec![];
        for idx in self.index.clone() {
            if regex.is_match(idx.as_str()) {
                items.push(self[idx.as_str()].clone());
                names.push(idx);
            }
        }
        let mut series = Series::from(items);
        series.reindex(names, false);
        series
    }
    /// Get the item at idx
    ///
    /// Returns None if the item doesn't exist
    pub fn get(&self, idx: usize) -> Option<&T> {
        self.array.get(idx).to_owned()
    }
    ///  Get the name of the series0
    pub fn get_name(&self) -> String {
        self.name.clone()
    }
    /// Get the underlying indexer
    pub fn get_index(&self) -> Vec<String> {
        self.index.clone()
    }
    /// Get the data type of the Series
    pub fn get_dtype(&self) -> DataTypes {
        self.dtype.clone()
    }
    /// Print the first n items in the series.
    ///
    /// # Panics
    /// * If n is greater than the number of items in the array
    pub fn head(&self, n: usize)
    where
        T: Display,
    {
        let mut table = Table::new();
        table.set_format(*FORMAT_CLEAN);
        for i in 0..n {
            table.add_row(Row::new(vec![
                Cell::new(&self.index[i]),
                Cell::new(&format!("{}", &self.array[i])),
            ]));
        }
        println!("{}", table.to_string());
    }
    /// Returns a boolean indicating the array is empty
    /// # Example
    /// ```
    /// use dami::core::series::Series;
    /// fn main(){
    ///     let series:Series<i32> = Series::default();
    ///     assert_eq!(series.is_empty(),true)
    /// }
    /// ````
    pub fn is_empty(&self) -> bool {
        self.array.is_empty()
    }
    /// Return the first element of the underlying data as an [`Option`\
    ///
    /// # Returns
    /// [`None`] if the element doesn't exist in the first index (0)
    pub fn item(&self) -> Option<&T> {
        self.array.get(0)
    }
    /// Returns the underlying length of the array
    pub fn len(&self) -> usize {
        self.array.len()
    }
    /// Replace values where condition is true
    /// # Arguments
    /// * `value`: Value to replace when `cond` becomes true
    /// * `cond` a function that takes a generic T and returns either true or false.
    /// # Example
    /// ```
    /// use dami::prelude::*;
    /// let series =  Series::from([1,-1,2,5,3]);
    /// let s2=series.mask(5,|f| f< 20); //Change all to 5
    /// assert_eq!(s2,Series::from([5,5,5,5,5]));
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    pub fn mask<F>(&self, value: T, cond: F) -> Series<T>
    where
        F: Fn(T) -> bool,
    {
        let mut series =
            Series::from(
                self.array
                    .mapv(|f| if cond(f.clone()) { value.clone() } else { f }),
            );
        series.set_name(&self.get_name());
        series
    }
    /// Like [mask](#method.mask) but does the conversion in place
    #[allow(clippy::needless_pass_by_value)]
    pub fn mask_inplace<F>(&mut self, value: T, cond: F)
    where
        F: Fn(T) -> bool,
    {
        self.array
            .mapv_inplace(|f| if cond(f.clone()) { value.clone() } else { f })
    }

    /// Set a global name for the series which is used as a row identifier in the DataFrame
    pub fn set_name(&mut self, name: &str) {
        self.name = name.to_string()
    }
    /// Set a new indexerfot the series
    pub fn set_index(&mut self, index: Vec<String>) {
        self.reindex(index, false)
    }
    #[doc(hidden)]
    pub fn set_dtype(&mut self, dtype: DataTypes) {
        self.dtype = dtype
    }
    /// Print the last n items of the Series
    ///
    /// # Panics
    /// * If n is greater than the number of items in the array
    pub fn tail(&self, n: usize)
    where
        T: Display,
    {
        let mut table = Table::new();
        table.set_format(*FORMAT_CLEAN);
        let start = self.len() - n - 1;
        for _ in 0..n {
            table.add_row(Row::new(vec![
                Cell::new(&self.index[start]),
                Cell::new(&format!("{}", &self.array[start])),
            ]));
        }
        println!("{}", table.to_string());
    }
    /// Transform the values of a Series into a new value.
    /// # Arguments
    /// > `func`: A function that produces a new value for each value its called on
    /// # Example
    /// ```
    /// // Do not do this on production code
    /// use dami::core::series::Series;
    /// pub fn i32_to_f64(val:i32)->f64{
    ///     f64::from(val)
    /// }
    /// let series = Series::from([1,2,3,4,5,6]);
    /// let new_s=series.transform::<f64,_>(i32_to_f64);
    /// assert_eq!(new_s,Series::from([1.,2.,3.,4.,5.,6.]))
    /// ```
    pub fn transform<M,F>(&self, func: F) -> Series<M>
    where
        F:Fn(T) -> M,
        M: Default+ 'static + Clone
    {
        let mut series = Series::from(self.array.mapv(|f| func(f)));
        series.set_name(&self.get_name());
        series
    }
    /// Return the underlying ndarray of the Series;
    pub fn to_ndarray(&self) -> Array1<T> {
        self.array.clone()
    }
    /// Return the underlying ndarray as a vector of type T elements
    pub fn to_vec(&self) -> Vec<T> {
        self.array.to_vec()
    }
    /// Returns unique values of the Series object.
    ///
    /// Uniques are `not` returned in order of appearance since a HashSet is used to filter non-unique elements
    pub fn unique(&self) -> HashSet<T>
    where
        T: Hash + Eq,
    {
        HashSet::from_iter(self.to_vec().into_iter())
    }
}

///Check to ensure there are no duplicates in names
fn validate_names(me: Vec<String>, other: Vec<String>) -> Result<(), Error> {
    let me_set: HashSet<String> = HashSet::from_iter(me);
    let other_set: HashSet<String> = HashSet::from_iter(other);
    if me_set.len() != other_set.len() {
        return Err(Error::LabelError);
    }
    Ok(())
}

pub fn create_index(len: usize, prefix: &str, suffix: &str) -> Vec<String> {
    let mut index = Vec::with_capacity(len);
    (0..len).for_each(|f| index.push(format!("{}{}{}", prefix, f, suffix)));
    index
}
