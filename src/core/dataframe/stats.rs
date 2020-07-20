#![cfg(feature="stats")]

use crate::core::dataframe::DataFrame;
use crate::core::series::Series;
#[allow(unused_imports)]
use ndarray_stats::errors::{EmptyInput, MinMaxError, MultiInputError, QuantileError};
impl DataFrame {
    /// # Requires Feature
    ///  > * `stats`
    /// # Implemented for
    /// > * Floats => [`f32`],[`f64`]
    ///
    /// Returns the [`arithmetic mean`] x̅ of all elements in each Series:
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
    pub fn mean(&self) -> Series<f64> {
        self.block.mean()
    }
    /// # Requires Feature
    ///  > * `stats`
    /// # Implemented for
    /// > * Floats => [`f32`],[`f64`]
    ///
    ///Finds the element wise minumum of the series in the DataFrame.
    /// Even if there are multiple (equal) elements that are minima, only one is returned.
    /// (Which one is returned is unspecified and may depend on the memory layout of the array.)
    ///
    /// # Panics
    /// * [`MinMaxError::EmptyInput`] if the array is empty
    ///
    /// * [`MinMaxError::UndefinedOrder`] if any of the pairwise orderings tested by the function are undefined.
    /// (For example, this occurs if there are any floating-point NaN values in the array.)
    pub fn min(&self) -> Series<f64> {
        self.block.min()
    }
    /// # Requires Feature
    ///  > * `stats`
    /// # Implemented for
    /// > * Floats => [`f32`],[`f64`],[`i32`]
    ///
    ///Finds the element wise maximum of each series in the DataFrame.
    /// Even if there are multiple (equal) elements that are maxima, only one is returned.
    /// (Which one is returned is unspecified and may depend on the memory layout of the array.)
    /// # Panics
    /// * [`MinMaxError::EmptyInput`] if the array is empty
    ///
    /// * [`MinMaxError::UndefinedOrder`] if any of the pairwise orderings tested by the function are undefined.
    /// (For example, this occurs if there are any floating-point NaN values in the array.)
    pub fn max(&self) -> Series<f64> {
        self.block.max()
    }
    /// # Requires Feature
    ///  > * `stats`
    /// # Implemented for
    /// > * Floats => [`f32`],[`f64`]
    ///
    /// Returns the [`geometric mean`] `GM(X)` of all Series in the DataFrame:
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
    pub fn skewness(&self) -> Series<f64> {
        self.block.skewness()
    }
    /// Calculate the [population variance] of arrays in the DataFrame
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
        self.block.variance()
    }
    /// # Requires Feature
    ///  > * `stats`
    /// # Implemented for
    /// > * Floats => [`f32`],[`f64`]
    ///
    /// Returns the [kurtosis] `Kurt[X]` of all Series in the DataFrame:
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
        self.block.kurtosis()
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
    pub fn central_moment(&self, order: u16) -> Series<f64> {
        self.block.central_moment(order)
    }
    /// Calculate the Pearson correlation coefficients for the series in the DataFrame
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
    pub fn corr(&self) -> DataFrame {
        self.block.corr()
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
        self.block.cov(min_periods)
    }
}

impl DataFrame {
    /// # Requires Feature
    ///  > * `stats`
    ///
    /// Plot data in the series
    ///
    /// This function provides higher order methods for plotting data in a series
    /// The series should contain numerical data (usize, floats,ints) ie anything implementing the Num trait
    ///
    /// # Arguments
    /// * `kind`: The type of plot to draw.
    /// > Supported plots are
    /// > > * "hist"-> histogram
    ///
    /// > > * "h_hist" -> horizontal histogram
    ///
    /// > > * "bar"->bar graph
    ///
    /// > > * "box"->box plot
    ///
    /// > > * "scatter" -> scatter graph
    ///
    /// > > * "line" -> line graph
    ///
    /// If the string passed to `kind` argument doesn't match the above values. A line plot is drown
    ///
    /// # Note
    /// This is backed by [plotly.js] using the [plotly] crate, so the resulting graph is opened in
    /// your default browser.
    ///
    /// [plotly.js]: https://plot.ly/javascript/
    /// [plotly]: https://docs.rs/plotly
    pub fn plot(&self, kind: &str) {
        self.block.plot(kind)
    }
    /// Plot a graph into a jupyter notebook using rust repl environment which can be downloaded and installed
    /// from  [here](https://github.com/google/evcxr)
    ///
    /// Plots supported are the same as `plot()` method
    ///
    /// # READ THIS!
    /// You should have already `set up a conda environment` this [website](https://shahinrostami.com/posts/programming/rust-notebooks/setup-anaconda-jupyter-and-rust/)
    /// has some nice instructions on how to do this.
    ///
    /// > This embeds a whole HTML file ( about 10kb) to the notebook. And this grows linearly for every graph rendered.
    ///
    /// To make it work run
    /// ```bash
    /// $ jupyter labextension install jupyterlab-plotly@4.8.2
    /// ```
    /// to install plotly extension in your `conda environment`
    /// # Tip
    /// This is a large library and pulls in a lot of dependencies. It takes long to compile
    /// so it is recommended that you use your jupyter environment configured to use sccache.
    /// Instructions for setting that up can be found [here](https://github.com/google/evcxr/tree/master/evcxr_jupyter)
    pub fn plot_evcxr(&self, kind: &str) {
        self.block.plot_evcxr(kind)
    }
}
