use crate::core::series::errors::SeriesErrors;
use crate::core::series::Series;
use noisy_float::types::{n64, N64};
use num_traits::{Float, FromPrimitive, Num, One, Zero};
use std::cmp::{max, min};
use std::f64::NAN;
use std::iter::Sum;
use std::ops::{Add, AddAssign, Div, Mul, MulAssign, Sub};

impl<T> Series<T>
where
    T: Default + Clone + 'static,
{
    /// Returns whether all elements are true
    /// # Example
    /// ```
    /// use crate::dami::core::series::Series;
    ///
    ///fn main(){
    ///    let series:Series<f64> = Series::from(vec![0.0,4.0,9.0,8.0]);
    ///    assert_eq!(series.all(),false) // since there is a zero which is false
    /// }
    /// ```
    pub fn all(&self) -> bool
    where
        T: FromPrimitive + Num,
    {
        self.array.iter().all(|f| f.clone() != T::default())
    }
    /// Returns whether all elements are true
    /// # Example
    /// ```
    /// use crate::dami::core::series::Series;
    ///fn main(){
    ///    let series:Series<f64> = Series::from(vec![0.0,4.0,9.0,8.0]);
    ///    assert_eq!(series.any(),true) // since there are non-zero elements;
    /// }
    /// ```
    pub fn any(&self) -> bool
    where
        T: FromPrimitive + Num,
    {
        self.array.iter().any(|f| f.clone() != T::default())
    }
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
    /// fn main(){
    ///     let series:Series<f64> = Series::from(vec![0.0,1.1,2.2,4.4]);
    ///     let b= series.between(1.0,3.3,true);
    ///     assert_eq!(b,Series::from(vec![false,true,true,false]))
    /// }
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    pub fn between(&self, left: T, right: T, inclusive: bool) -> Series<bool>
    where
        T: PartialOrd,
    {
        let name = self.name.clone();
        let mut new_series = Series::from(self.array.mapv(|f| {
            if inclusive {
                left <= f && f <= right
            } else {
                left < f && f < right
            }
        }));
        new_series.set_name(name.as_str());
        new_series
    }
    /// Return a boolean scalar value of a single element in a Series
    ///
    /// # Panics
    /// In `debug mode`
    /// if the len of the Series is not 1 and that element isn't boolean
    pub fn bool(&self) -> bool
    where
        T: FromPrimitive + PartialEq,
    {
        assert_eq!(self.len(), 1, "Series doesn't contain a scalar value");
        *self.array.get(0).unwrap() != FromPrimitive::from_usize(0).unwrap()
    }
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
    /// fn main(){
    ///     let series:Series<f64> =  Series::from(vec![1.0,2.,3.,4.,5.]);
    ///     let series_b = series.clip(0.,3.);
    ///     assert_eq!(series_b,Series::from(vec![1.,2.,3.,3.,3.]))
    /// }
    /// ````
    #[allow(clippy::needless_pass_by_value)]
    pub fn clip(&self, lower: T, upper: T) -> Series<T>
    where
        T: PartialOrd,
    {
        let name = self.name.clone();
        let mut series = Series::from(self.array.mapv(|f| {
            if f < lower {
                lower.clone()
            } else if f > upper {
                upper.clone()
            } else {
                f
            }
        }));
        series.name = name;
        series
    }
    /// Count the number of non-NA observation values in the series
    /// # Returns
    /// Number of null values in the series
    /// # Example
    /// ```
    /// use crate::dami::core::series::Series;
    /// use std::f64::NAN;
    /// fn main(){
    ///
    ///     let series:Series<f64> = Series::from([1.,0.,NAN,3.,7.,NAN]);
    ///     assert_eq!(series.count(),2);
    /// }
    /// ```
    pub fn count(&self) -> usize
    where
        T: Float,
    {
        let mut count: usize = 0;
        self.array.iter().for_each(|f| {
            if f.is_nan() {
                count += 1
            }
        });
        count
    }
    /// Calculate and return the cumulative sum of a series
    /// # Example
    /// ```
    /// use crate::dami::core::series::Series;
    /// fn main(){
    ///     let series = Series::from([0.,1.,3.,4.]);
    ///     assert_eq!(series.cum_sum(),Series::from([0.,1.,4.,8.]));
    /// }
    /// ```
    pub fn cum_sum(&self) -> Series<T>
    where
        T: Sum + AddAssign,
    {
        let mut prev_sum = T::default();
        let mut vector = Vec::with_capacity(self.len());
        self.array.iter().enumerate().for_each(|(len, f)| {
            if len == 0 {
                prev_sum = f.to_owned();
                vector.push(prev_sum.clone());
            } else {
                prev_sum += f.to_owned();
                vector.push(prev_sum.clone());
            }
        });
        let mut series = Series::from(vector);
        series.name = self.name.clone();
        series
    }
    /// Calculate and return the cumulative minimum of a series
   /// # Example
   /// ```
   /// use crate::dami::core::series::Series;
   /// fn main(){
   ///     let series = Series::from([0,1,3,4,2,2]);
   ///     assert_eq!(series.cum_max(),Series::from([0,1,3,4,4,4]));
   /// }
   /// ```
    pub fn cum_max(&self) -> Series<T>
    where
        T: Ord,
    {
        let mut prev = T::default();
        let mut cum_max = Vec::with_capacity(self.len());
        for (len, f) in self.array.into_iter().enumerate() {
            if len == 0 {
                prev = f.to_owned();
            }
            prev = max(prev.clone(), f.clone());
            cum_max.push(prev.clone());
        }
        Series::from(cum_max)
    }
    /// Calculate and return the cumulative min of a series
    /// # Example
    /// ```
    /// use crate::dami::core::series::Series;
    /// fn main(){
    ///     let series = Series::from([0,1,3,4,2,4]);
    ///     assert_eq!(series.cum_min(),Series::from([0,0,0,0,0,0]));
    /// }
    /// ```
    pub fn cum_min(&self) -> Series<T>
    where
        T: Ord,
    {
        let mut prev = T::default();
        let mut cum_min = vec![];
        //TODO: Add support for NaN options without actually dropping it
        for (len, f) in self.array.into_iter().enumerate() {
            if len == 0 {
                prev = f.to_owned();
            }
            prev = min(prev.clone(), f.to_owned());
            cum_min.push(prev.clone());
        }
        Series::from(cum_min)
    }
    /// Calculate and return the cumulative product over a series
    /// # Arguments

    /// # Example
    /// ```
    /// use crate::dami::core::series::Series;
    /// fn main(){
    ///     let series = Series::from([1.,2.,3.,4.,]);
    ///     assert_eq!(series.cum_prod(),Series::from([1.,2.,6.,24.]))
    /// }
    /// ```
    pub fn cum_prod(&self) -> Series<T>
    where
        T: MulAssign,
    {
        let mut prev = T::default();
        // Hold the result
        let mut cum_prod = vec![];
        for (len, f) in self.array.into_iter().enumerate() {
            if len == 0 {
                prev = f.to_owned();
            }
            prev *= f.to_owned();
            cum_prod.push(prev.clone());
        }
        Series::from(cum_prod)
    }

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
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    pub fn diff(&self, period: i64) -> Series<T>
    where
        T: Float + Sub<Output = T> + From<f64>,
    {
        // Okay rust does not allow negative index
        // a[-1] in python should be a[a.len()-1]
        let mut holder = vec![];
        // Used in negatives to tell us when to stop in order not to overflow
        let fixed = (self.len() as i64) + period - 1;
        for (len, i) in self.array.iter().enumerate() {
            if period.is_negative() {
                if len as i64 > fixed {
                    holder.push(NAN.into());
                } else {
                    // Bad arithmetic
                    // But it works: <>
                    // at pos 8,period -1 eg it becomes a[8]-a[7--1] which is a[8]-a[9]
                    holder.push(*i - self.array[(len as i64 - period) as usize]);
                }
            } else {
                let new_period = len as i64 - period;
                // 0-1 fetch elm 1
                if new_period.is_negative() {
                    holder.push(NAN.into());
                } else {
                    holder.push(*i - self.array[new_period as usize])
                }
            }
        }
        let mut series = Series::from(holder);
        series.name = self.name.clone();
        series
    }
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
    pub fn dot(&self, other: &Series<T>) -> Result<T, SeriesErrors>
    where
        T: 'static
            + Copy
            + Zero
            + One
            + Add<Output = T>
            + Sub<Output = T>
            + Mul<Output = T>
            + Div<Output = T>,
    {
        let me_arr = &self.array;
        let other_arr = &other.array;
        if self.len() == other.len() {
            // Use ndarray's backend
            Ok(me_arr.dot(other_arr))
        }
        // if lengths misalign raise an error
        else {
            Err(SeriesErrors::MatrixUnaligned(self.len(), other.len()))
        }
    }
    /// Return a series with NaN values dropped
    pub fn drop_na(&self) -> Series<T>
    where
        T: Float,
    {
        let mut arr = vec![];
        for i in self.array.iter() {
            if i.is_nan() {
                continue;
            }
            // dereference and push
            arr.push(*i);
        }
        let mut series = Series::from(arr);
        series.name = self.name.clone();
        series
    }
    /// Fill NAN values with the specified values
    pub fn fillna(&self, value: T) -> Series<T>
    where
        T: Float,
    {
        Series::from(self.array.mapv(|f| if f.is_nan() { value } else { f }))
    }
    /// Fill NaN values with the specified values but d not return a new series
    /// but modify the current series
    pub fn fillna_inplace(&mut self, value: T)
    where
        T: Float,
    {
        self.array
            .mapv_inplace(|f| if f.is_nan() { value } else { f })
    }
    /// Returns the first index for a non-NA value
    ///
    /// If all elements are null/Na returns None
    pub fn first_valid_index(&self) -> Option<String>
    where
        T: Float,
    {
        for i in self.clone().into_iter().enumerate() {
            if !i.1.is_nan() {
                return Some(self.index[i.0].clone());
            }
        }
        None
    }
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
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    pub fn pct_change(&self, period: i64) -> Series<T>
    where
        T: Copy + Div<Output = T> + From<f64>,
    {
        // Okay rust does not allow negative index
        // a[-1] in python should be a[a.len()-1]
        let mut holder = Vec::with_capacity(self.len());
        // Used in negatives to tell us when to stop in order not to overflow
        let fixed = (self.len()) as i64 + period - 1;
        for (len, i) in self.array.iter().enumerate() {
            if period.is_negative() {
                if len as i64 > fixed {
                    holder.push(NAN.into())
                } else {
                    // Bad arithmetic
                    // But it works: <>
                    // at pos 8,period -1 eg it becomes a[8]-a[7--1] which is a[8]-a[9]
                    holder.push(*i / self.array[(len as i64 - period) as usize]);
                }
            } else {
                let new_period = len as i64 - period;
                // 0-1 fetch elm 1
                if new_period.is_negative() {
                    holder.push(NAN.into());
                } else {
                    holder.push(*i / self.array[new_period as usize])
                }
            }
        }
        let mut series = Series::from(holder);
        series.name = self.name.clone();
        series
    }
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
    pub fn round(&self) -> Series<T>
    where
        T: Float,
    {
        let mut series = Series::from(self.array.mapv(num_traits::Float::round));
        series.name = self.name.clone();
        series
    }
}
impl<T: Default + Clone + 'static + Float> Series<T> {
    // Calculate and return the cumulative max of a float series
    /// # Arguments
    ///   `skip_na`: `bool` If set to true NaN values will be skipped resulting in a much smaller Series
    ///     than the initial one
    /// # Example
    /// ```
    /// use crate::dami::core::series::Series;
    /// fn main(){
    ///     let series = Series::from([0.,1.,3.,4.,2.,4.]);
    ///     assert_eq!(series.cum_max_f(),Series::from([0.,1.,3.,4.,4.,4.]));
    /// }
    /// ```
    pub fn cum_max_f(&self) -> Series<T> {
        let mut prev = T::default();
        let mut cum_max = Vec::with_capacity(self.len());
        for (len, f) in self.array.into_iter().enumerate() {
            if len == 0 {
                prev = *f;
            }
            // Skip nan values
            if f.is_nan() {
                continue;
            }
            prev = prev.max(*f);
            cum_max.push(prev);
        }
        let mut series = Series::from(cum_max);
        series.name = self.name.clone();
        series
    }
    /// Calculate and return the cumulative min of a series
    /// # Example
    /// ```
    /// use crate::dami::core::series::Series;
    /// fn main(){
    ///     let series = Series::from([0.,1.,3.,4.,2.,4.]);
    ///     assert_eq!(series.cum_min_f(),Series::from([0.,0.,0.,0.,0.,0.]));
    /// }
    /// ```
    pub fn cum_min_f(&self) -> Series<T> {
        let mut prev = T::default();
        let mut cum_min = Vec::with_capacity(self.len());
        for (len, f) in self.array.into_iter().enumerate() {
            if len == 0 {
                prev = *f;
            }
            // Skip nan values
            if f.is_nan() {
                continue;
            }
            prev = prev.min(*f);
            cum_min.push(prev);
        }
        let mut series = Series::from(cum_min);
        series.name = self.name.clone();
        series
    }
}
#[cfg(feature = "stats")]
/// Generate descriptive characteristics
pub trait Describe {
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
    /// use dami::prelude::*;
    ///
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
}
#[cfg(feature = "stats")]
impl Describe for Series<f64> {
    #[allow(clippy::cast_precision_loss)]
    fn describe(&self) -> Series<f64> {
        // The names according to how they will be stored
        let names = vec![
            "count", "mean", "stdev", "pstdev", "min", "25%", "50%", "75%", "max",
        ];
        let mut described_data: Vec<f64> = Vec::with_capacity(8);
        // count
        described_data.push(self.len() as f64);
        // mean
        described_data.push(self.mean().unwrap());
        // standard deviation
        described_data.push(self.stdev());
        // Population standard deviation
        described_data.push(self.pstdev());
        // minimum
        described_data.push(*self.min().unwrap());
        // Quantiles
        let mut convert: Vec<N64> = vec![];
        for i in self.array.iter() {
            if i.is_nan() {
                continue;
            }
            {
                convert.push(n64(*i));
            }
        }
        let mut quantiles = Series::from(convert);
        described_data.push(
            quantiles
                .quantile_axis_mut(n64(0.25))
                .unwrap()
                .first()
                .unwrap()
                .to_owned()
                .into(),
        );
        // We could do this better :| One day...
        described_data.push(
            quantiles
                .quantile_axis_mut(n64(0.5))
                .unwrap()
                .first()
                .unwrap()
                .to_owned()
                .into(),
        );
        // Don't cry its gonna be alright...
        described_data.push(
            quantiles
                .quantile_axis_mut(n64(0.75))
                .unwrap()
                .first()
                .unwrap()
                .to_owned()
                .into(),
        );
        // Maximum
        described_data.push(*self.max().unwrap());
        // Series
        let mut series = Series::from(described_data);
        series.name = self.name.clone();
        series.reindex(names, false);
        series
    }
}
#[cfg(feature = "stats")]
impl Describe for Series<f32> {
    #[cfg(feature = "stats")]
    fn describe(&self) -> Series<f64> {
        self.as_type::<f64>().describe()
    }
}
#[cfg(feature = "stats")]
impl Describe for Series<i32> {
    fn describe(&self) -> Series<f64> {
        self.as_type::<f64>().describe()
    }
}
