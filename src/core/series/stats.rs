//! Statistical functions for Series
//!
//! This module implements common features from the [ndarray-stats] crate like mean
//! and weighted_sum for a series.
//!
//! Most implementations are for Float types like f64
//!
//! [ndarray-stats]: https://docs.rs/ndarray-stats/
use super::ndarray::{Array0, Array2, Axis};
use crate::core::series::Series;
use ndarray_stats::errors::{EmptyInput, MinMaxError, MultiInputError, QuantileError};
use ndarray_stats::interpolate::Nearest;
use ndarray_stats::CorrelationExt;
use ndarray_stats::MaybeNan;
use ndarray_stats::QuantileExt;
use ndarray_stats::SummaryStatisticsExt;
use noisy_float::types::N64;
use num_traits::{Float, FromPrimitive, Zero};
use std::iter::Sum;
use std::ops::{Div, Mul};

impl<T: Clone + Float + Default> Series<T> {
    /// # Requires Feature
    ///  > * `stats`
    /// # Implemented for
    /// > * Floats => [`f32`],[`f64`],[`i32`]
    ///
    ///Finds the element wise maximum of the array.
    /// Even if there are multiple (equal) elements that are maxima, only one is returned.
    /// (Which one is returned is unspecified and may depend on the memory layout of the array.)
    /// # Example
    /// ```
    /// use crate::dami::core::series::Series;
    /// fn main(){
    ///    let series:Series<f64>= Series::from(vec![0.0,4.0,9.0,8.0]);
    ///    assert_eq!(series.max().unwrap().to_owned(),9.0)
    /// }
    /// ```
    /// # Errors
    /// * [`MinMaxError::EmptyInput`] if the array is empty
    ///
    /// * [`MinMaxError::UndefinedOrder`] if any of the pairwise orderings tested by the function are undefined.
    /// (For example, this occurs if there are any floating-point NaN values in the array.)
    pub fn max(&self) -> Result<&T, MinMaxError> {
        self.array.max()
    }
    /// # Requires Feature
    ///  > * `stats`
    /// # Implemented for
    /// > * Floats => [`f32`],[`f64`]
    ///
    ///Finds the element wise minumum of the array.
    /// Even if there are multiple (equal) elements that are minima, only one is returned.
    /// (Which one is returned is unspecified and may depend on the memory layout of the array.)
    /// # Example
    /// ```
    /// use crate::dami::core::series::Series;
    /// fn main(){
    ///    let series:Series<f64>= Series::from(vec![0.0,4.0,9.0,8.0]);
    ///    assert_eq!(series.min().unwrap().to_owned(),0.0)
    /// }
    /// ```
    /// # Errors
    /// * [`MinMaxError::EmptyInput`] if the array is empty
    ///
    /// * [`MinMaxError::UndefinedOrder`] if any of the pairwise orderings tested by the function are undefined.
    /// (For example, this occurs if there are any floating-point NaN values in the array.)
    pub fn min(&self) -> Result<&T, MinMaxError> {
        self.array.min()
    }
    /// # Requires Feature
    ///  > * `stats`
    /// # Implemented for
    /// > * Floats => [`f32`],[`f64`]
    ///
    ///Finds the index of the minimum value of the array.
    ///
    /// Even if there are multiple (equal) elements that are minima, only one index is returned. (Which one is returned is unspecified and may depend on the memory layout of the array)
    /// # Example
    /// ```
    /// use crate::dami::core::series::Series;
    /// fn main(){
    ///     let series:Series<f64>= Series::from(vec![0.0,1.0,2.0,3.0,4.0,-56.0]);
    ///     assert_eq!(series.argmin().unwrap(),5) //index at 5 is -56.0
    /// }
    /// ```
    /// # Errors
    /// [`MinMaxError::UndefinedOrder`] if any of the pairwise orderings tested by the function are undefined. (For example, this occurs if there are any floating-point NaN values in the array.)
    ///
    /// [`MinMaxError::EmptyInput`] if the array is empty.
    pub fn argmin(&self) -> Result<usize, MinMaxError> {
        self.array.argmin()
    }
    /// # Requires Feature
    ///  > * `stats`
    /// # Implemented for
    /// > * Floats => [`f32`],[`f64`]
    ///
    ///Finds the index of the maximum value of the array.
    ///
    // Returns
    ///
    /// Even if there are multiple (equal) elements that are maxima, only one
    /// index is returned. (Which one is returned is unspecified and may depend on the memory layout of the array.)
    /// # Example
    /// ```
    /// use crate::dami::core::series::Series;
    /// fn main(){
    ///    let series = Series::from([0.,1.,3.,5.,2.,1.,4.,6.,5.,4.,34.].to_vec());
    ///     assert_eq!(series.argmax().unwrap(),10);
    /// }
    /// ```
    /// # Errors
    /// [`MinMaxError::UndefinedOrder`] if any of the pairwise orderings tested by the function are undefined. (For example, this occurs if there are any floating-point NaN values in the array.)
    ///
    /// [`MinMaxError::EmptyInput`] if the array is empty.
    pub fn argmax(&self) -> Result<usize, MinMaxError> {
        self.array.argmax()
    }
}

impl<T: Clone + MaybeNan + Float + Ord + Default> Series<T> {
    /// # Requires Feature
    ///  > * `stats`
    /// # Implemented for
    /// > * Floats => [`f32`],[`f64`]
    ///
    ///Finds the index of the minimum value of the array skipping NaN values.
    ///
    /// Even if there are multiple (equal) elements that are minima, only one index is returned. (Which one is returned is unspecified and may depend on the memory layout of the
    /// # Errors
    /// [`EmptyInput`] if the array is empty or none of the values in the array are non-NaN values.
    pub fn argmin_skipnan(&self) -> Result<usize, EmptyInput>
    where
        <T as MaybeNan>::NotNan: std::cmp::Ord,
    {
        self.array.argmin_skipnan()
    }
    ///Finds the element wise maximum of the array, skipping NaN values.
    ///
    /// Even if there are multiple (equal) elements that are maxima, only one is returned. (Which one is returned is unspecified and may depend on the memory layout of the array.)
    ///
    /// Warning This method will return a NaN value if none of the values in the array are non-NaN values. Note that the NaN value might not be in the array.
    ///
    /// # Errors
    /// [`MinMaxError::EmptyInput`] if the array is empty

    pub fn max_skipnan(&self) -> Result<usize, EmptyInput>
    where
        <T as MaybeNan>::NotNan: std::cmp::Ord,
    {
        self.array.argmax_skipnan()
    }
}
impl<T: Ord + Clone + Default> Series<T> {
    /// Return the qth quantile of the data along the first axis.
    ///
    /// `q` needs to be a float between 0 and 1, bounds included.
    /// The qth quantile for a 1-dimensional lane of length `N` is defined
    /// as the element that would be indexed as `(N-1)q` if the lane were to be sorted
    /// in increasing order.
    /// If `(N-1)q` is not an integer the desired quantile lies between
    /// two data points: we return the lower, nearest, higher or interpolated
    /// value depending on the `interpolate` strategy.
    ///
    /// Some examples:
    /// - `q=0.` returns the minimum along each 1-dimensional lane;
    /// - `q=0.5` returns the median along each 1-dimensional lane;
    /// - `q=1.` returns the maximum along each 1-dimensional lane.
    /// (`q=0` and `q=1` are considered improper quantiles)
    ///
    /// The array is shuffled **in place** along each 1-dimensional lane in
    /// order to produce the required quantile without allocating a copy
    /// of the original array. Each 1-dimensional lane is shuffled independently
    /// from the others.
    /// No assumptions should be made on the ordering of the array elements
    /// after this computation.
    ///
    /// Complexity ([quickselect](https://en.wikipedia.org/wiki/Quickselect)):
    /// - average case: O(`m`);
    /// - worst case: O(`m`^2);
    /// where `m` is the number of elements in the array.
    /// # Errors
    /// Returns `Err(EmptyInput)` when the specified axis has length 0.
    ///
    /// Returns `Err(InvalidQuantile(q))` if `q` is not between `0.` and `1.` (inclusive).
    ///
    /// **Panics** if `axis` is out of bounds.
    pub fn quantile_axis_mut(&mut self, q: N64) -> Result<Array0<T>, QuantileError>
    where
        T: Ord,
    {
        self.array.quantile_axis_mut(Axis(0), q, &Nearest)
    }
}
impl<T: Copy + FromPrimitive + Div<Output = T> + Zero + Default> Series<T> {
    /// # Requires Feature
    ///  > * `stats`
    /// # Implemented for
    /// > * Floats => [`f32`],[`f64`]
    ///
    /// Returns the [`arithmetic mean`] x̅ of all elements in the array:
    ///
    /// # Rendered using $\KaTeX$
    /// The arithmetic mean is defined as..
    /// $$
    /// \overline{x}=\frac{1}{n}\sum_{i=1}^n{x_1}
    /// $$
    ///
    /// If the array is empty, `Err(EmptyInput)` is returned.
    ///
    /// **Panics** if `A::from_usize()` fails to convert the number of elements in the array.
    ///
    /// [`arithmetic mean`]: https://en.wikipedia.org/wiki/Arithmetic_mean
    pub fn mean(&self) -> Option<T> {
        self.array.mean()
    }
}
impl<T: Copy + Div<Output = T> + Mul<Output = T> + Zero + Default> Series<T> {
    /// # Requires Feature
    ///  > * `stats`
    /// # Implemented for
    /// > * Floats => [`f32`],[`f64`]
    ///
    /// Returns the [arithmetic weighted mean] x̅ of all elements in the array. Use `weighted_sum`
    /// if the `weights` are normalized (they sum up to 1.0).
    ///
    /// # Rendered using $\KaTeX$
    /// The arithmetic weighted mean is defined as..
    ///
    /// $$
    ///  \overline{x} = \frac{\displaystyle\sum_{i=1}^{n}w_1x_1}{\displaystyle\sum_{i=1}^{n}w_1}
    /// $$
    ///
    /// # Panics
    /// if division by zero panics for type T.
    ///
    ///
    /// # Errors
    /// * `MultiInputError::EmptyInput` if `self` is empty
    /// * `MultiInputError::ShapeMismatch` if `self` and `weights` don't have the same shape
    ///
    /// [arithmetic weighted mean]: https://en.wikipedia.org/wiki/Weighted_arithmetic_mean
    pub fn weighted_mean(&self, weights: &Self) -> Result<T, MultiInputError> {
        self.array.weighted_mean(&weights.array)
    }
}
impl<T: Copy + Mul<Output = T> + Zero + Default> Series<T> {
    /// # Requires Feature
    ///  > * `stats`
    /// # Implemented for
    /// > * Floats => [`f32`],[`f64`]
    ///
    /// Returns the weighted sum of all elements in the array, that is, the dot product of the
    /// arrays `self` and `weights`. Equivalent to `weighted_mean` if the `weights` are normalized.
    /// $$
    /// \overline{x} = \displaystyle\sum_{i=1}^{n}w_1x_1
    /// $$
    /// # Errors
    ///
    /// * `MultiInputError::ShapeMismatch` if `self` and `weights` don't have the same shape
    pub fn weighted_sum(&self, weights: &Self) -> Result<T, MultiInputError> {
        self.array.weighted_sum(&weights.array)
    }
}
impl<T: Float + FromPrimitive + Default + 'static> Series<T> {
    /// # Requires Feature
    ///  > * `stats`
    /// # Implemented for
    /// > * Floats => [`f32`],[`f64`]
    ///
    /// Calculate the covariance of the Series and another series
    ///
    /// The covariance of two random variables is defined as
    /// $$
    /// cov(X,Y)=\frac {1}{N-periods}\sum_{i=1}^{N}(x_i-\overline{x})(y_1-\overline{y})
    /// $$
    ///
    /// where
    ///```text
    ///     1   n
    /// x̅ = ―   ∑ xᵢ
    ///     n  i=1
    /// ```
    /// and similarly for $\overline{y}$
    /// # Arguments
    ///   * `min_periods`: Minimum number of observations needed to have a valid result
    /// # Warning
    ///  This function is not optimised for speed due to some workarounds needed to convert
    /// a one dimensional array to 2-D. Sometimes it may be relatively slow for large Series(partly because
    /// of how vec distributes its elements everywhere in memory)
    /// # Panics
    /// In `debug mode` if the length of two arrays are not equal
    pub fn cov(self, other: &Series<T>, min_periods: T) -> T {
        debug_assert_eq!(self.len(), other.len(), "Lengths not equal");
        let mut my_vec = self.to_vec();
        my_vec.extend_from_slice(other.to_vec().as_slice());
        let array2 = Array2::from_shape_vec((2, self.len()), my_vec).unwrap();
        array2.cov(min_periods).unwrap()[[0, 1]]
    }
    /// Calculate the Pearson correlation coefficients for this series and another
    ///
    /// Pearson's correlation coefficient is the covariance
    /// of the two variables divided by the product of their standard deviations.
    ///
    /// Or can be expressed as...
    ///
    /// $$
    /// \rho_x,_y= \frac{cov(X,Y)}{\sigma_X\sigma_Y}
    /// $$
    ///
    /// where `cov` is the covariance, σx is the covariance of X and σy is the covariance of Y
    ///
    /// # Warning
    ///  This function is not optimised for speed due to some workarounds needed to convert
    /// a one dimensional array to 2-D. Sometimes it may be relatively slow for large Series(partly because
    /// of how vec distributes its elements everywhere in memory)
    pub fn corr(self, other: &Series<T>) -> T {
        debug_assert_eq!(self.len(), other.len(), "Lengths not equal");
        let mut my_vec = self.to_vec();
        my_vec.extend_from_slice(other.to_vec().as_slice());
        let array2 = Array2::from_shape_vec((2, self.len()), my_vec).unwrap();
        array2.pearson_correlation().unwrap()[[0, 1]]
    }
    /// # Requires Feature
    ///  > * `stats`
    /// # Implemented for
    /// > * Floats => [`f32`],[`f64`]
    ///
    /// Returns the [`geometric mean`] `GM(X)` of all elements in the array:
    ///
    /// $$
    /// GM(X)=\left( \prod_{i=1}^{N}x_1 \right)^{\frac{1}{N}}
    /// $$
    ///
    /// # Panics
    /// if `A::from_usize()` fails to convert the number of elements in the array.
    ///
    /// # Errors
    ///  [`EmptyInput`]  If the array is empty
    ///
    /// [`geometric mean`]: https://en.wikipedia.org/wiki/Geometric_mean
    pub fn geometric_mean(&self) -> Result<T, EmptyInput> {
        self.array.geometric_mean()
    }
    /// # Requires Feature
    ///  > * `stats`
    /// # Implemented for
    /// > * Floats => [`f32`],[`f64`]
    ///
    /// Returns the [kurtosis] `Kurt[X]` of all elements in the array:
    ///
    /// ```text
    /// Kurt[X] = μ₄ / σ⁴
    /// ```
    ///
    /// where μ₄ is the fourth central moment and σ is the standard deviation of
    /// the elements in the array.
    ///
    /// This is sometimes referred to as _Pearson's kurtosis_. Fisher's kurtosis can be
    /// computed by subtracting 3 from Pearson's kurtosis.
    /// # Errors
    /// [`EmptyInput`] If the array is empty,
    ///
    /// # Panics
    /// if `A::from_usize()` fails to convert the number of elements in the array.
    ///
    /// [kurtosis]: https://en.wikipedia.org/wiki/Kurtosis
    pub fn kurtosis(&self) -> Result<T, EmptyInput> {
        self.array.kurtosis()
    }
    /// # Requires Feature
    ///  > * `stats`
    /// # Implemented for
    /// > * Floats => [`f32`],[`f64`]
    ///
    /// Returns the [Pearson's moment coefficient of skewness] γ₁ of all elements in the array:
    ///
    /// ```text
    /// γ₁ = μ₃ / σ³
    /// ```
    ///
    /// where μ₃ is the third central moment and σ is the standard deviation of
    /// the elements in the array.
    /// # Errors
    /// [`EmptyInput`] is returned.
    /// # Panics
    /// if `A::from_usize()` fails to convert the number of elements in the array.
    ///
    /// [Pearson's moment coefficient of skewness]: https://en.wikipedia.org/wiki/Skewness
    pub fn skewness(&self) -> Result<T, EmptyInput> {
        self.array.skewness()
    }
    /// # Requires Feature
    ///  > * `stats`
    /// # Implemented for
    /// > * Floats => [`f32`],[`f64`]
    ///
    /// Returns the *p*-th [central moment] of all elements in the array, μₚ:
    ///
    /// $$
    /// \upsilon_p=\frac{1}{N}\sum_{i=1}^{N}(x_1-\overline{x})^p
    /// $$
    ///
    ///
    ///
    /// The *p*-th central moment is computed using a corrected two-pass algorithm (see Section 3.5
    /// in [Pébay et al., 2016]). Complexity is *O(np)* when *n >> p*, *p > 1*.
    /// # Panics
    /// if `A::from_usize()` fails to convert the number of elements
    /// in the array or if `order` overflows `i32`.
    /// # Errors
    /// [`EmptyInput`] If the array is empty
    ///
    /// [central moment]: https://en.wikipedia.org/wiki/Central_moment
    /// [Pébay et al., 2016]: https://www.osti.gov/pages/servlets/purl/1427275
    pub fn central_moment(&self, order: u16) -> Result<T, EmptyInput> {
        self.array.central_moment(order)
    }
    /// # Requires Feature
    ///  > * `stats`
    /// # Implemented for
    /// > * Floats => [`f32`],[`f64`]
    ///
    /// Returns the first *p* [central moments] of all elements in the array, see [central moment]
    /// for more details.
    ///
    ///
    ///
    /// This method reuses the intermediate steps for the *k*-th moment to compute the *(k+1)*-th,
    /// being thus more efficient than repeated calls to [central moment] if the computation
    /// of central moments of multiple orders is required.
    /// # Panics
    ///  if `A::from_usize()` fails to convert the number of elements
    /// in the array or if `order` overflows `i32`.
    ///
    /// # Errors
    /// [`EmptyInput`] if array is empty
    ///
    /// [central moments]: https://en.wikipedia.org/wiki/Central_moment
    /// [central moment]: #method.central_moment
    pub fn central_moments(&self, order: u16) -> Result<Vec<T>, EmptyInput> {
        self.array.central_moments(order)
    }
}
impl<T: Float + FromPrimitive + Default + 'static> Series<T> {
    /// # Requires Feature
    ///  > * `stats`
    /// # Implemented for
    /// > * Floats => [`f32`],[`f64`]
    ///
    /// Calculate the [population standard deviation] of the array;
    ///
    ///  # Rendered using $\KaTeX$
    /// population standard deviation *s* is defined as
    /// $$
    /// s = \sqrt{\frac{1}{N}\sum_{i=1}^{N}(x_1-\overline{x})^2}
    /// $$
    ///
    ///
    /// # Panics
    /// If the mean of the array cannot be calculated
    ///
    /// [population standard deviation]:https://en.wikipedia.org/wiki/Standard_deviation
    pub fn pstdev(&self) -> T
    where
        T: Sum,
    {
        // Note. This is a simple implementation as we wait for standardised function from the ndarray stats
        // crate so this is the ugly hack i have
        let len =
            T::from_usize(self.len()).expect("Converting length from usize should never fail");
        let variance = self.variance() / len;
        variance.sqrt()
    }
    /// # Requires Feature
    ///  > * `stats`
    /// # Implemented for
    /// > * Floats => [`f32`],[`f64`]
    ///
    /// Calculate the [standard deviation] of the array;
    /// #  Rendered using $\KaTeX$
    /// Standard deviation *s* is defined as
    /// $$
    /// s = \sqrt{\frac{1}{N-1}\sum_{i=1}^{n}(x_1-\overline{x})^2}
    /// $$
    ///
    /// [standard deviation]:https://en.wikipedia.org/wiki/Standard_deviation
    ///
    /// # Panics
    /// If the mean of the array cannot be calculated
    pub fn stdev(&self) -> T
    where
        T: Sum,
    {
        // Note. This is a simple implementation as we wait for standardised function from the ndarray stats
        // crate so this is the ugly hack i have
        let variance = self.variance();
        let len =
            T::from_usize(self.len()).expect("Converting length from usize should never fail");

        let new = variance / (len - T::from(1).unwrap());
        new.sqrt()
    }
    /// Calculate the [population variance] of an array
    ///
    /// # Rendered using $\KaTeX$
    /// Population variance $\sigma^2$ is defined as
    /// $$
    /// \sigma^2= \frac{1}{N}\sum_{i=1}^{N}(x_i-\upsilon)^2
    /// $$
    ///
    /// where N is the number of elements in the array, and $\upsilon$ is the mean of the array
    /// # Panics
    /// If the mean of the array cannot be calculated
    pub fn variance(&self) -> T
    where
        T: Sum,
    {
        let mean = self.mean().unwrap();
        let variance = self
            .array
            .iter()
            .map(|value| {
                let diff = (*value) - mean;
                diff * diff
            })
            .sum::<T>();
        variance
    }
}
