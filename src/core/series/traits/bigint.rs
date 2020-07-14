//! This module contains traits for rust [`i64`]  and [i128] int type Series
use crate::core::series::errors::SeriesErrors;
use crate::core::series::Series;

/// This trait exports methods for Integer type Series
/// # Note
/// This trait does not expose all methods for Series eg `is_nan` as these functions act on floats
pub trait BigIntSeries<T: Default> {
    /// Returns whether all elements are true
    /// ```
    /// use crate::dami::core::series::Series;
    /// use crate::dami::core::series::traits::bigint::BigIntSeries;
    ///fn main(){
    ///    let series:Series<i64>= Series::from(vec![0,4,9,8]);
    ///    assert_eq!(series.all(),false) // since there is a zero which is false
    /// }
    /// ```

    fn all(&self) -> bool;
    /// Returns whether all elements are true
    /// ```
    /// use crate::dami::core::series::Series;
    /// use crate::dami::core::series::traits::bigint::BigIntSeries;
    ///fn main(){
    ///    let series:Series<i64>= Series::from(vec![0,4,9,8]);
    ///    assert_eq!(series.any(),true) // since there are non-zero elements;
    /// }
    /// ```
    fn any(&self) -> bool;
    /// Return a boolean series equivalent to left <= series <= right
    ///
    /// # Arguments
    /// * `left`: Left boundary
    /// > * type:int
    /// * `right`: Right boundary
    /// > * type:int
    /// * `inclusive`: If set to true series equivalent becomes left <= series <= right
    /// if false  left < series < right
    ///  # Returns
    ///  [`Series`] representing whether each element is between left and right
    /// # Example
    /// ```
    /// use dami::core::series::Series;
    /// use crate::dami::core::series::traits::bigint::BigIntSeries;
    /// fn main(){
    ///     let series:Series<i64> = Series::from(vec![0,1,2,4]);
    ///     let b= series.between(1,3,true);
    ///     assert_eq!(b,Series::from(vec![false,true,true,false]))
    /// }
    /// ```
    fn between(&self, left: T, right: T, inclusive: bool) -> Series<bool>;
    /// Return a boolean scalar value of a single element in a Series
    ///
    /// # Panics
    /// if the len of the Series is not 1 and that element isn't boolean
    fn bool(&self) -> bool;
    /// Trim values at input thresholds
    ///
    /// Assigns values outside the boundary to boundary values
    ///
    /// # Arguments
    /// * `lower`: Minimum threshold value. All values below this  threshold will be set to it
    /// > * `type`: int
    /// * `upper` : Maximum threshold value. All values above this threshold will be set to it
    /// > * `type`: int
    ///
    /// # Returns
    /// [`Series`] With the same type as calling object with values outside the clip boundaries replaced
    ///
    /// # Example
    /// ````
    /// use crate::dami::core::series::Series;
    /// use crate::dami::core::series::traits::bigint::BigIntSeries;
    /// fn main(){
    ///     let series:Series<i64> =  Series::from(vec![1,2,3,4,5]);
    ///     let series_b = series.clip(0,3);
    ///     assert_eq!(series_b,Series::from(vec![1,2,3,3,3]))
    /// }
    /// ````
    fn clip(&self, lower: T, upper: T) -> Series<T>;

    /// Calculate and return the cumulative sum of a series
    /// # Example
    /// ```
    /// use crate::dami::core::series::Series;
    /// use crate::dami::core::series::traits::ints::SeriesInt;
    /// fn main(){
    ///     let series = Series::from([0,1,3,4]);
    ///     assert_eq!(series.cum_sum(),Series::from([0,1,4,8]));
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
    /// use crate::dami::core::series::traits::bigint::BigIntSeries;
    /// fn main(){
    ///     let series:Series<i64> = Series::from([0,1,3,4,2,4]);
    ///     assert_eq!(series.cum_max(),Series::from([0,1,3,4,4,4]));
    /// }
    /// ```
    fn cum_max(&self) -> Series<T>;
    /// Calculate and return the cumulative min of a series
    /// # Arguments
    ///   `skip_na`: `bool` If set to true NaN values will be skipped resulting in a much smaller Series
    ///     than the initial one
    /// # Example
    /// ```
    /// use crate::dami::core::series::Series;
    /// use crate::dami::core::series::traits::bigint::BigIntSeries;
    /// fn main(){
    ///     let series:Series<i64> = Series::from([0,1,3,4,2,4]);
    ///     assert_eq!(series.cum_min(),Series::from([0,0,0,0,0,0]));
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
    /// use crate::dami::core::series::traits::bigint::BigIntSeries;
    /// fn main(){
    ///     let series:Series<i64> = Series::from([1,2,3,4,]);
    ///     assert_eq!(series.cum_prod(),Series::from([1,2,6,24]))
    /// }
    /// ```
    fn cum_prod(&self) -> Series<T>;
    /// Calculate the dot product of this series and another
    ///
    /// # Rendered using [$\KaTeX$].
    /// The dot product of to vector is defined as
    /// $$
    /// a.b = \sum_{i=1}^n a_ib_i=a_1b_1+a_2b_2+....+a_nb_n
    /// $$
    /// The dot of an array  is calculated as..
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
}
