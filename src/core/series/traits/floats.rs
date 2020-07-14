//!This module contains traits for rust [`f32`] and [`f64`] Series
use crate::core::series::errors::SeriesErrors;
use crate::core::series::Series;

/// This trait exports functions for Series' [`f64`] and [`f32`] types

pub trait SeriesFloat<T: Default> {
    /// Returns whether all elements are true
    /// # Example
    /// ```
    /// use crate::dami::core::series::Series;
    /// use crate::dami::core::series::traits::floats::SeriesFloat;
    ///
    ///fn main(){
    ///    let series:Series<f64> = Series::from(vec![0.0,4.0,9.0,8.0]);
    ///    assert_eq!(series.all(),false) // since there is a zero which is false
    /// }
    /// ```
    fn all(&self) -> bool;
    /// Returns whether all elements are true
    /// # Example
    /// ```
    /// use crate::dami::core::series::Series;
    /// use crate::dami::core::series::traits::floats::SeriesFloat;
    ///fn main(){
    ///    let series:Series<f64> = Series::from(vec![0.0,4.0,9.0,8.0]);
    ///    assert_eq!(series.any(),true) // since there are non-zero elements;
    /// }
    /// ```
    fn any(&self) -> bool;
    /// Return a boolean series equivalent to left <= series <= right
    /// # Arguments
    /// * `left`: Left boundary
    /// > * type:&str
    /// * `right`: Right boundary
    /// > * type:&str
    /// * `inclusive`: If set to true series equivalent becomes left <= series <= right
    /// if false  left < series < right
    ///  # Returns
    ///  [`Series`] representing whether each element is between left and right
    /// # Example
    /// ```
    /// use dami::core::series::Series;
    /// use crate::dami::core::series::traits::floats::SeriesFloat;
    /// fn main(){
    ///     let series:Series<f64> = Series::from(vec![0.0,1.1,2.2,4.4]);
    ///     let b= series.between(1.0,3.3,true);
    ///     assert_eq!(b,Series::from(vec![false,true,true,false]))
    /// }
    /// ```
    fn between(&self, left: T, right: T, inclusive: bool) -> Series<bool>;
    /// Return a boolean scalar value of a single element in a Series
    ///
    /// # Panics
    /// In `debug mode`
    /// if the len of the Series is not 1 and that element isn't boolean
    fn bool(&self) -> bool;
    /// Trim values at input thresholds
    ///
    /// Assigns values outside the boundary to boundary values
    ///
    /// # Arguments
    /// * `lower`: Minimum threshold value. All values below this  threshold will be set to it
    /// > * `type`: &str
    /// * `upper` : Maximum threshold value. All values above this threshold will be set to it
    /// > * `type`: &str
    ///
    /// # Returns
    /// [`Series`] With the same type as calling object with values outside the clip boundaries replaced
    ///
    /// # Example
    /// ````
    /// use crate::dami::core::series::Series;
    /// use crate::dami::core::series::traits::floats::SeriesFloat;
    /// fn main(){
    ///     let series:Series<f64> =  Series::from(vec![1.0,2.,3.,4.,5.]);
    ///     let series_b = series.clip(0.,3.);
    ///     assert_eq!(series_b,Series::from(vec![1.,2.,3.,3.,3.]))
    /// }
    /// ````
    fn clip(&self, lower: T, upper: T) -> Series<T>;

    /// Count the number of non-NA observation values in the series
    /// # Returns
    /// Number of null values in the series
    /// # Example
    /// ```
    /// use crate::dami::core::series::Series;
    /// use crate::dami::core::series::traits::floats::SeriesFloat;
    /// use std::f64::NAN;
    /// fn main(){
    ///
    ///     let series:Series<f64> = Series::from([1.,0.,NAN,3.,7.,NAN]);
    ///     assert_eq!(series.count(),2);
    /// }
    /// ```
    fn count(&self) -> usize;
    /// Calculate and return the cumulative sum of a series
    /// # Example
    /// ```
    /// use crate::dami::core::series::Series;
    /// use crate::dami::core::series::traits::floats::SeriesFloat;
    /// fn main(){
    ///     let series = Series::from([0.,1.,3.,4.]);
    ///     assert_eq!(series.cum_sum(),Series::from([0.,1.,4.,8.]));
    /// }
    /// ```
    fn cum_sum(&self) -> Series<T>;
    /// Calculate and return the cumulative max of a series
    /// # Arguments
    ///   `skip_na`: `bool` If set to true NaN values will be skipped resulting in a much smaller Series
    ///     than the initial one
    /// # Example
    /// ```
    /// use crate::dami::core::series::Series;
    /// use crate::dami::core::series::traits::floats::SeriesFloat;
    /// fn main(){
    ///     let series = Series::from([0.,1.,3.,4.,2.,4.]);
    ///     assert_eq!(series.cum_max(),Series::from([0.,1.,3.,4.,4.,4.]));
    /// }
    /// ```
    fn cum_max(&self) -> Series<T>;
    /// Calculate and return the cumulative min of a series
    /// # Example
    /// ```
    /// use crate::dami::core::series::Series;
    /// use crate::dami::core::series::traits::floats::SeriesFloat;
    /// fn main(){
    ///     let series = Series::from([0.,1.,3.,4.,2.,4.]);
    ///     assert_eq!(series.cum_min(),Series::from([0.,0.,0.,0.,0.,0.]));
    /// }
    /// ```
    fn cum_min(&self) -> Series<T>;
    /// Calculate and return the cumulative product over a series
    /// # Arguments
    /// `skip_na`: `bool` If set to true, NaN Values will be skipped, resulting in a smaller series
    /// than the initial one
    /// # Example
    /// ```
    /// use crate::dami::core::series::Series;
    /// use crate::dami::core::series::traits::floats::SeriesFloat;
    /// fn main(){
    ///     let series = Series::from([1.,2.,3.,4.,]);
    ///     assert_eq!(series.cum_prod(true),Series::from([1.,2.,6.,24.]))
    /// }
    /// ```
    fn cum_prod(&self, skip_na: bool) -> Series<T>;
    /// # Requires Feature
    ///  > * `stats`
    ///
    /// Generate descriptive characteristics
    ///
    /// These includes those that summarize central tendency, dispersion and shape
    ///
    /// NAN values are by default going to be skipped
    /// # For Numeric Data
    /// The results index will include `count`,`mean`,`std`,`pstdev`,`min`,`max` as well as lower, 50 and upper
    /// percentiles
    /// # Warning
    /// For quantiles ie ["25%","50%","75%"] floats are converted to an n64 type as float types in Rust
    /// do not implement [`Ord`] trait (due to NaN values being both max and min)
    ///
    /// Therefore NaN values are skipped
    /// the float numbers are converted to an N64 (see [noisy_float](https://docs.rs/noisy_float/0.1.11/noisy_float/types/type.N64.html)) and then the quantiles are calculated and the integer converted
    /// back to floats.
    ///
    /// This is computationally expensive but the only way it may be implemented(currently)
    /// any ideas are welcome to improve this
    /// # Panics
    /// * If the mean cannot be calculated
    /// * If the minimum value cannot be calculated
    /// * If the quantiles cannot calculated
    /// * If the maximum value cannot be calculated
    ///
    /// # Example
    /// ```
    /// #![cfg(feature="stats")]
    /// use dami::core::series::Series;
    /// use dami::core::series::traits::floats::SeriesFloat;
    /// fn main(){
    ///     let series = Series::from([1.,2.,3.]);
    ///     let described = series.describe();
    ///     println!("{:?}",series);
    /// }
    /// ```
    /// Prints
    /// ```text
    /// count       3.0
    /// mean        2.0
    /// std         1.0
    /// ptsdev      0.86
    /// min         1.0
    /// 25%         1.5
    /// 50%         2.0
    /// 75%         2.5
    /// max         3.0
    /// ```
    #[cfg(feature = "stats")]
    fn describe(&self) -> Series<f64>;
    /// Calculate the first discrete difference of an element
    ///
    /// Calculates the difference of a Series element compared with another element in the Series
    ///
    /// # Arguments
    ///  `periods`: [`i32`] Periods to shift for calculating difference, accepts negative values
    /// # Returns
    /// [`Series`] with First differences of the Series
    /// ```
    /// use crate::dami::core::series::Series;
    /// use crate::dami::core::series::traits::floats::SeriesFloat;
    /// fn main(){
    ///     let series = Series::from([0.,1.,2.,3.,5.,8.,]);
    ///     println!("{:?}",series.diff(1));
    /// }
    /// ```
    /// Prints
    /// ```text
    /// 0   NAN
    /// 1   0.0
    /// 2   1.0
    /// 3   2.0
    /// 4   2.0
    /// 5   3.0
    ///
    ///name: series, dtype: f64
    /// ```
    /// Difference with the following row
    /// ```text
    /// s.diff(-1)
    /// ```
    /// Prints
    /// ```text
    ///0    0.0
    /// 1   -1.0
    /// 2   -1.0
    /// 3   -2.0
    /// 4   -3.0
    /// 5    NaN
    /// name:series, dtype: float64
    /// ```
    fn diff(&self, periods: i32) -> Series<T>;
    /// Calculate the dot product of this series and another
    ///
    /// # Rendered using [$\KaTeX$].
    /// The dot product of to vector is defined as
    /// $$
    /// a.b = \sum_{i=1}^n a_ib_i=a_1b_1+a_2b_2+....+a_nb_n
    /// $$
    ///
    /// The dot of an array  is calculated as..
    ///
    /// $$
    /// a.b = \begin {bmatrix} a_1 & a_2 & a_3 & a_4\end {bmatrix}
    /// \begin {bmatrix} b_1 \\\ b_2 \\\ b_3 \\\ b_4 \\\\
    /// \end {bmatrix}
    /// $$
    ///
    /// # Errors
    /// * `MatrixUnalignedError`: If the array have different lengths
    ///
    /// [$\KaTeX$]: https://katex.org/
    fn dot(&self, other: &Series<T>) -> Result<T, SeriesErrors>;
    /// Return a series with NaN values dropped
    fn drop_na(&self) -> Series<T>;
    /// Fill NAN values with the specified values
    fn fillna(&self, value: T) -> Series<T>;
    /// Fill NaN values with the specified values but d not return a new series
    /// but modify the current series
    fn fillna_inplace(&mut self, value: T);
    /// Returns the first index for a non-NA value
    ///
    /// If all elements are null/Na returns None
    fn first_valid_index(&self) -> Option<String>;
    /// Return a new boolean same sized Series indicating whether the Series values as NAN
    ///
    /// Characters like NaN get mapped to true while the rest are mapped to false
    fn isnull(&self) -> Series<bool>;
    /// Alias of [`is_null`](#tymethod.isnull)
    fn notna(&self) -> Series<bool>;
    /// Calculate percentage change between the current and the prior element
    /// # Arguments
    /// `periods`: If positive it computes the percentage change  with the `n` previous column
    /// If negative it computes the percentage change with the `n` forward array
    ///# Example
    ///```
    /// use dami::prelude::*;
    /// fn main(){
    ///     let  series = Series::from([1.,2.,4.,8.]);
    ///      println!("{:?}",series.pct_change(1));
    /// }
    ///```
    /// Prints
    /// ```text
    /// index        values
    ///  0            NaN
    ///  1            2.0000
    ///  2            2.0000
    ///  3            2.0000
    ///
    ///  name:series  dtype:f64
    /// ```
    fn pct_change(&self, periods: i32) -> Series<T>;
    /// Returns the nearest integer to a floating point number
    ///
    /// # Example
    /// ```
    /// use dami::prelude::*;
    /// fn main(){
    ///
    ///     let series = Series::from([1.323,2.345,6.789,4.432,9.99]);
    ///     assert_eq!(series.round(),Series::from([1.,2.,7.,4.,10.]));
    /// }
    /// ```
    fn round(&self) -> Series<T>;
}
