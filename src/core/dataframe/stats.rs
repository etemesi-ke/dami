#![cfg(feature="stats")]
use crate::core::dataframe::DataFrame;
use crate::core::series::Series;
use crate::enums::DataTypes;
use ndarray::Array2;
#[allow(unused_imports)]
use ndarray_stats::errors::{EmptyInput, MinMaxError, MultiInputError, QuantileError};
use ndarray_stats::CorrelationExt;
use std::collections::HashMap;

impl DataFrame {
    /// Find the mean of the series in the DataFrame.
    ///
    ///
    #[allow(clippy::cast_precision_loss)]
    pub fn mean(&self) -> Series<f64> {
        let mut map: HashMap<&str, f64> = HashMap::new();
        for i in &self.order {
            let dtype = self.dtypes.get(i).unwrap();
            match dtype {
                DataTypes::F64 => {
                    let series = self.container.get::<Series<f64>, _>(i).unwrap();
                    map.insert(i.as_str(), series.mean().unwrap());
                }
                DataTypes::F32 => {
                    let series = self.container.get::<Series<f32>, _>(i).unwrap();
                    map.insert(i.as_str(), f64::from(series.mean().unwrap()));
                }
                DataTypes::I32 => {
                    let series = self.container.get::<Series<i32>, _>(i).unwrap();
                    map.insert(i.as_str(), f64::from(series.mean().unwrap()));
                }
                DataTypes::I64 => {
                    let series = self.container.get::<Series<i64>, _>(i).unwrap();
                    map.insert(i.as_str(), series.mean().unwrap() as f64);
                }

                DataTypes::I128 => {
                    let series = self.container.get::<Series<i128>, _>(i).unwrap();
                    map.insert(i.as_str(), series.mean().unwrap() as f64);
                }
                _ => continue,
            }
        }
        Series::from(map)
    }
    // # Requires Feature
    ///  > * `stats`
    /// # Implemented for
    /// > * Floats => [`f32`],[`f64`]
    ///
    ///Finds the index of the minimum value of the array.
    ///
    /// Even if there are multiple (equal) elements that are minima, only one index is returned. (Which one is returned is unspecified and may depend on the memory layout of the array)
    /// # Example
    /// ```
    /// use crate::dami::core::dataframe::DataFrame;
    /// use dami::prelude::Series;
    /// fn main(){
    ///     let df = DataFrame::from(vec![vec![0.0,1.0,3.0,3.0,4.0,-56.0],vec![0.0,1.0,2.0,3.0,4.0,-56.0]]);
    ///     assert_eq!(df.argmin(),Series::from([0,0,1,0,0,0])) //index at 5 is -56.0
    /// }
    /// ```
    /// # Errors
    /// [`MinMaxError::UndefinedOrder`] if any of the pairwise orderings tested by the function are undefined. (For example, this occurs if there are any floating-point NaN values in the array.)
    ///
    /// [`MinMaxError::EmptyInput`] if the array is empty.
    pub fn argmin(&self) -> Series<usize> {
        let mut map: Vec<(&str, usize)> = Vec::new();
        for i in &self.order {
            let dtype = self.dtypes.get(i).unwrap();
            match dtype {
                DataTypes::F64 => {
                    let series = self.container.get::<Series<f64>, _>(i).unwrap();
                    map.push((i.as_str(), series.argmin().unwrap()));
                }
                DataTypes::F32 => {
                    let series = self.container.get::<Series<f32>, _>(i).unwrap();
                    map.push((i.as_str(), series.argmin().unwrap()));
                }
                DataTypes::I32 => {
                    let series = self.container.get::<Series<i32>, _>(i).unwrap();
                    map.push((i.as_str(), series.as_type::<f64>().argmin().unwrap()));
                }
                _ => continue,
            }
        }
        Series::from(map)
    }
    /// # Requires Feature
    ///  > * `stats`
    /// # Implemented for
    /// > * Floats => [`f32`],[`f64`],[`i32`]
    ///
    ///Finds the element wise minumum of the array.
    /// Even if there are multiple (equal) elements that are minima, only one is returned.
    /// (Which one is returned is unspecified and may depend on the memory layout of the array.)
    /// # Example
    /// ```
    /// use crate::dami::core::dataframe::DataFrame;
    /// use dami::prelude::Series;
    /// fn main(){
    ///    let df = DataFrame::from(vec![vec![1,2,3,4,5,6,7,8],vec![2,3,4,5,6,7,8,9]]);
    ///    assert_eq!(df.min(),Series::from([1.0,2.0,3.0,4.0,5.0,6.0,7.0,8.0]));
    /// }
    /// ```
    /// # Errors
    /// * [`MinMaxError::EmptyInput`] if the array is empty
    ///
    /// * [`MinMaxError::UndefinedOrder`] if any of the pairwise orderings tested by the function are undefined.
    /// (For example, this occurs if there are any floating-point NaN values in the array.)
    pub fn min(&self) -> Series<f64> {
        let mut map: Vec<(&str, f64)> = Vec::new();
        for i in &self.order {
            let dtype = self.dtypes.get(i).unwrap();
            match dtype {
                DataTypes::F64 => {
                    let series = self.container.get::<Series<f64>, _>(i).unwrap();
                    map.push((i.as_str(), *series.min().unwrap() as f64));
                }
                DataTypes::F32 => {
                    let series = self.container.get::<Series<f32>, _>(i).unwrap();
                    map.push((i.as_str(), f64::from(*series.min().unwrap())));
                }
                DataTypes::I32 => {
                    let series = self.container.get::<Series<i32>, _>(i).unwrap();
                    map.push((i.as_str(), *series.as_type::<f64>().min().unwrap()));
                }
                _ => continue,
            }
        }
        Series::from(map)
    }
    /// # Requires Feature
    ///  > * `stats`
    /// # Implemented for
    /// > * Floats => [`f32`],[`f64`],[`i32`]
    ///
    ///Finds the element wise maximum of the array.
    /// Even if there are multiple (equal) elements that are maxima, only one is returned.
    /// (Which one is returned is unspecified and may depend on the memory layout of the array.)
    /// # Panics
    /// * [`MinMaxError::EmptyInput`] if the array is empty
    ///
    /// * [`MinMaxError::UndefinedOrder`] if any of the pairwise orderings tested by the function are undefined.
    /// (For example, this occurs if there are any floating-point NaN values in the array.)
    pub fn max(&self) -> Series<f64> {
        let mut map: Vec<(&str, f64)> = Vec::new();
        for i in &self.order {
            let dtype = self.dtypes.get(i).unwrap();
            match dtype {
                DataTypes::F64 => {
                    let series = self.container.get::<Series<f64>, _>(i).unwrap();
                    map.push((i.as_str(), *series.max().unwrap()));
                }
                DataTypes::F32 => {
                    let series = self.container.get::<Series<f32>, _>(i).unwrap();
                    map.push((i.as_str(), f64::from(*series.max().unwrap())));
                }
                DataTypes::I32 => {
                    let series = self.container.get::<Series<i32>, _>(i).unwrap();
                    map.push((i.as_str(), *series.as_type::<f64>().max().unwrap()));
                }
                _ => continue,
            }
        }
        Series::from(map)
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
    /// index is returned for each series. (Which one is returned is unspecified and may depend on the memory layout of the array.)
    /// # Panics
    /// [`MinMaxError::UndefinedOrder`] if any of the pairwise orderings tested by the function are undefined. (For example, this occurs if there are any floating-point NaN values in the array.)
    ///
    /// [`MinMaxError::EmptyInput`] if the array is empty.
    pub fn argmax(&self) -> Series<usize> {
        let mut map: Vec<(&str, usize)> = Vec::new();
        for i in &self.order {
            let dtype = self.dtypes.get(i).unwrap();
            match dtype {
                DataTypes::F64 => {
                    let series = self.container.get::<Series<f64>, _>(i).unwrap();
                    map.push((i.as_str(), series.argmax().unwrap()));
                }
                DataTypes::F32 => {
                    let series = self.container.get::<Series<f32>, _>(i).unwrap();
                    map.push((i.as_str(), series.argmax().unwrap()));
                }
                DataTypes::I32 => {
                    let series = self.container.get::<Series<i32>, _>(i).unwrap();
                    map.push((i.as_str(), series.as_type::<f64>().argmax().unwrap()));
                }
                _ => continue,
            }
        }
        Series::from(map)
    }
    /// # Requires Feature
    ///  > * `stats`
    /// # Implemented for
    /// > * Floats => [`f32`],[`f64`]
    ///
    /// Returns the *p*-th [central moment] of all series in the DataFrame, μₚ:
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
    pub fn central_moment(&self, order: u16) -> Series<f64> {
        let mut map: Vec<(&str, f64)> = Vec::new();
        for i in &self.order {
            let dtype = self.dtypes.get(i).unwrap();
            match dtype {
                DataTypes::F64 => {
                    let series = self.container.get::<Series<f64>, _>(i).unwrap();
                    map.push((i.as_str(), series.central_moment(order).unwrap()));
                }
                DataTypes::F32 => {
                    let series = self.container.get::<Series<f32>, _>(i).unwrap();
                    map.push((i.as_str(), f64::from(series.central_moment(order).unwrap())));
                }
                DataTypes::I32 => {
                    let series = self.container.get::<Series<i32>, _>(i).unwrap();
                    map.push((
                        i.as_str(),
                        series.as_type::<f64>().central_moment(order).unwrap(),
                    ));
                }
                _ => continue,
            }
        }
        Series::from(map)
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
    /// [`EmptyInput`] if array is empty
    ///
    /// [central moments]: https://en.wikipedia.org/wiki/Central_moment
    /// [central moment]: #method.central_moment
    pub fn central_moments(&self, order: u16) -> DataFrame {
        let mut df = DataFrame::new();
        for i in &self.order {
            let dtype = self.dtypes.get(i).unwrap();
            match dtype {
                DataTypes::F64 => {
                    let series = self.container.get::<Series<f64>, _>(i).unwrap();
                    df.add_series(Series::from(series.central_moments(order).unwrap()), true)
                        .unwrap();
                }
                DataTypes::F32 => {
                    let series = self.container.get::<Series<f32>, _>(i).unwrap();
                    df.add_series(Series::from(series.central_moments(order).unwrap()), true)
                        .unwrap();
                }
                DataTypes::I32 => {
                    let series = self.container.get::<Series<i32>, _>(i).unwrap();
                    df.add_series(
                        Series::from(series.as_type::<f64>().central_moments(order).unwrap()),
                        true,
                    )
                    .unwrap();
                }
                _ => continue,
            }
        }
        df
    }
    /// Computes the pairwise covariance among the Series of the DataFrame
    ///
    /// The returned DataFrame is the covariance matrix of the columns of the DataFrame
    ///
    /// # Arguments
    /// `min_periods`:Minimum number of observations required per pair of columns to have a valid result.
    ///
    /// # Notes
    /// * As of now column names will be dropped and the DataFrame will return columns with numbering from 0 to N
    /// * For `f32` and `i32` series type the series is first converted to `f64`.
    /// * All the series values are stored in an intermediary vector and the vector is converted into a 2D array
    ///  and the covariance is then calculated
    ///
    /// # Panics
    /// * If the resulting array is empty
    pub fn cov(&self, min_periods: f64) -> DataFrame {
        // Store all items whose covariance can be computed
        // These are i32,f32 and f64
        let mut frames = Vec::new();
        let mut amt = 0;
        for i in &self.order {
            let dtype = self.dtypes.get(i).unwrap();
            match dtype {
                DataTypes::F64 => {
                    let series = self.container.get::<Series<f64>, _>(i).unwrap();
                    frames.extend_from_slice(series.to_vec().as_slice());
                    amt += 1
                }
                DataTypes::F32 => {
                    let series = self.container.get::<Series<f32>, _>(i).unwrap();
                    frames.extend_from_slice(series.as_type::<f64>().to_vec().as_slice());
                    amt += 1
                }
                DataTypes::I32 => {
                    let series = self.container.get::<Series<i32>, _>(i).unwrap();
                    frames.extend_from_slice(series.as_type::<f64>().to_vec().as_slice());
                    amt += 1
                }
                _ => continue,
            }
        }
        let arr = Array2::from_shape_vec((amt, self.len), frames).unwrap();
        DataFrame::from(arr.cov(min_periods).unwrap())
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
    /// # Panics
    /// [`EmptyInput`] If the Array contains no inputs
    pub fn corr(&self) -> DataFrame {
        // Store all items whose covariance can be computed
        // These are i32,f32 and f64
        let mut frames = Vec::new();
        let mut amt = 0;
        for i in &self.order {
            let dtype = self.dtypes.get(i).unwrap();
            match dtype {
                DataTypes::F64 => {
                    let series = self.container.get::<Series<f64>, _>(i).unwrap();
                    frames.extend_from_slice(series.to_vec().as_slice());
                    amt += 1
                }
                DataTypes::F32 => {
                    let series = self.container.get::<Series<f32>, _>(i).unwrap();
                    frames.extend_from_slice(series.as_type::<f64>().to_vec().as_slice());
                    amt += 1
                }
                DataTypes::I32 => {
                    let series = self.container.get::<Series<i32>, _>(i).unwrap();
                    frames.extend_from_slice(series.as_type::<f64>().to_vec().as_slice());
                    amt += 1
                }
                _ => continue,
            }
        }
        let arr = Array2::from_shape_vec((amt, self.len), frames).unwrap();
        DataFrame::from(arr.pearson_correlation().unwrap())
    }
    /// # Requires Feature
    ///  > * `stats`
    /// # Implemented for
    /// > * Floats => [`f32`],[`f64`]
    ///
    /// Returns the [`geometric mean`] `GM(X)` of the series in the DataFrame :
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
    pub fn geometric_mean(&self) -> Series<f64> {
        let mut map: Vec<(&str, f64)> = Vec::new();
        for i in &self.order {
            let dtype = self.dtypes.get(i).unwrap();
            match dtype {
                DataTypes::F64 => {
                    let series = self.container.get::<Series<f64>, _>(i).unwrap();
                    map.push((i.as_str(), series.geometric_mean().unwrap()));
                }
                DataTypes::F32 => {
                    let series = self.container.get::<Series<f32>, _>(i).unwrap();
                    map.push((i.as_str(), f64::from(series.geometric_mean().unwrap())));
                }
                DataTypes::I32 => {
                    let series = self.container.get::<Series<i32>, _>(i).unwrap();
                    map.push((
                        i.as_str(),
                        series.as_type::<f64>().geometric_mean().unwrap(),
                    ));
                }
                _ => continue,
            }
        }
        Series::from(map)
    }
    /// # Requires Feature
    ///  > * `stats`
    /// # Implemented for
    /// > * Floats => [`f32`],[`f64`]
    ///
    /// Returns the [kurtosis] `Kurt[X]` of all series  in the DataFrame:
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
    pub fn kurtosis(&self) -> Series<f64> {
        let mut map: Vec<(&str, f64)> = Vec::new();
        for i in &self.order {
            let dtype = self.dtypes.get(i).unwrap();
            match dtype {
                DataTypes::F64 => {
                    let series = self.container.get::<Series<f64>, _>(i).unwrap();
                    map.push((i.as_str(), series.kurtosis().unwrap()));
                }
                DataTypes::F32 => {
                    let series = self.container.get::<Series<f32>, _>(i).unwrap();
                    map.push((i.as_str(), f64::from(series.kurtosis().unwrap())));
                }
                DataTypes::I32 => {
                    let series = self.container.get::<Series<i32>, _>(i).unwrap();
                    map.push((i.as_str(), series.as_type::<f64>().kurtosis().unwrap()));
                }
                _ => continue,
            }
        }
        Series::from(map)
    }
    /// # Requires Feature
    ///  > * `stats`
    /// # Implemented for
    /// > * Floats => [`f32`],[`f64`]
    ///
    /// Returns the [Pearson's moment coefficient of skewness] γ₁ of all series in the DataFrame:
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
    pub fn skewness(&self) -> Series<f64> {
        let mut map: Vec<(&str, f64)> = Vec::new();
        for i in &self.order {
            let dtype = self.dtypes.get(i).unwrap();
            match dtype {
                DataTypes::F64 => {
                    let series = self.container.get::<Series<f64>, _>(i).unwrap();
                    map.push((i.as_str(), series.skewness().unwrap()));
                }
                DataTypes::F32 => {
                    let series = self.container.get::<Series<f32>, _>(i).unwrap();
                    map.push((i.as_str(), f64::from(series.skewness().unwrap())));
                }
                DataTypes::I32 => {
                    let series = self.container.get::<Series<i32>, _>(i).unwrap();
                    map.push((i.as_str(), series.as_type::<f64>().skewness().unwrap()));
                }
                _ => continue,
            }
        }
        Series::from(map)
    }
    /// # Requires Feature
    ///  > * `stats`
    /// # Implemented for
    /// > * Floats => [`f32`],[`f64`]
    ///
    /// Calculate the [standard deviation] of the series in the DataFrame;
    /// #  Rendered using $\KaTeX$
    /// Standard deviation *s* is defined as
    /// $$
    /// s = \sqrt{\frac{1}{N-1}\sum_{i=1}^{n}(x_1-\overline{x})^2}
    /// $$
    ///
    /// # Panics
    /// If the mean of the array cannot be calculated
    ///
    ///  /// [standard deviation]:https://en.wikipedia.org/wiki/Standard_deviation
    pub fn stdev(&self) -> Series<f64> {
        let mut map: Vec<(&str, f64)> = Vec::new();
        for i in &self.order {
            let dtype = self.dtypes.get(i).unwrap();
            match dtype {
                DataTypes::F64 => {
                    let series = self.container.get::<Series<f64>, _>(i).unwrap();
                    map.push((i.as_str(), series.stdev()));
                }
                DataTypes::F32 => {
                    let series = self.container.get::<Series<f32>, _>(i).unwrap();
                    map.push((i.as_str(), f64::from(series.stdev())));
                }
                DataTypes::I32 => {
                    let series = self.container.get::<Series<i32>, _>(i).unwrap();
                    map.push((i.as_str(), series.as_type::<f64>().stdev()));
                }
                _ => continue,
            }
        }
        Series::from(map)
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
    pub fn variance(&self) -> Series<f64> {
        let mut map: Vec<(&str, f64)> = Vec::new();
        for i in &self.order {
            let dtype = self.dtypes.get(i).unwrap();
            match dtype {
                DataTypes::F64 => {
                    let series = self.container.get::<Series<f64>, _>(i).unwrap();
                    map.push((i.as_str(), series.variance()));
                }
                DataTypes::F32 => {
                    let series = self.container.get::<Series<f32>, _>(i).unwrap();
                    map.push((i.as_str(), f64::from(series.variance())));
                }
                DataTypes::I32 => {
                    let series = self.container.get::<Series<i32>, _>(i).unwrap();
                    map.push((i.as_str(), series.as_type::<f64>().variance()));
                }
                _ => continue,
            }
        }
        Series::from(map)
    }
}
