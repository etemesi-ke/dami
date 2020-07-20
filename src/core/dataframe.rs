//!  DataFrame functionalities.
//!
use crate::core::block_manager::BlockManager;
use crate::core::series::Series;
use crate::enums::{DataFrameErrors, DataTypes};
use ndarray::{Array1, Array2};
use std::collections::hash_map::RandomState;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::fmt;
use std::ops::{Add, Div, Mul, Sub};
mod stats;
/// The DataFrame struct
#[derive(Default, Clone)]
pub struct DataFrame {
    block: BlockManager,
}
impl fmt::Debug for DataFrame {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.block)
    }
}

impl fmt::Display for DataFrame {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.block)
    }
}
impl DataFrame {
    /// Create a new `DataFrame`
    pub fn new() -> DataFrame {
        Self::default()
    }
    /// Add a series to the DataFrame
    ///
    /// # Arguments
    /// * `other`: A Series to be added
    /// * `preserve_names`: If `true`, the name of the series will be preserved
    ///  if `false` the series will be referred in the DataFrame using numbers from 0..N
    /// # Errors
    /// * `DifferentLength`: A series to be added contains different length than the initial series
    /// * `ColumnNameErrors`: If the series contains a name already in the DataFrame. to prevent overwriting the original series
    /// > **Note**: Initially if a name conflict occurs, we will reassign the series with a number and if the same number is still in the DataFrame
    ///  then we panic
    pub fn add_series<T>(
        &mut self,
        other: Series<T>,
        preserve_names: bool,
    ) -> Result<(), DataFrameErrors>
    where
        T: Clone + Default + 'static,
    {
        self.block.add_series(other, preserve_names)
    }
    /// Apply a function either Row or column-wise and Returns a Scalar for each row or column
    /// # Arguments
    /// * `axis`: If set to true the function is applied Row wise if false the function is applied column wise.
    /// * `func`: A function to apply to the DataFrame
    /// # Returns
    /// * `Some(Series<T>)` if the type `T` Exists in the DataFrame
    /// * `None` if otherwise
    /// # Example
    /// ```
    /// use dami::prelude::*;
    /// use ndarray::{Array2,Array1};
    /// use std::ops::Add;
    /// use num_traits::Zero;
    /// fn sum_axis<T:Add<Output=T>+Clone+Zero>(arr:Array1<T>)->T{
    ///     arr.sum()
    /// }
    ///
    /// let ones:Array2<f64> = Array2::ones((4,8)); // Create a 4 by 8 array
    /// let df = DataFrame::from(ones);
    ///
    /// df.apply::<f64,_>(true,sum_axis).unwrap(); // This looks like [8,8,8,8]
    /// df.apply::<f64,_>(false,sum_axis).unwrap(); // This looks like [4,4,4,4,4,4,4,4]
    /// ```
    pub fn apply<T, F>(&self, axis: bool, func: F) -> Option<Series<T>>
    where
        T: Clone + Default + 'static,
        F: Clone + Fn(Array1<T>) -> T,
    {
        self.block.apply(func, axis)
    }
    ///  Apply a function elementwise and create a new DataFrame from the Result
    ///
    /// # Notes
    /// If the current DataFrame contains Heterogeneous Data, the elements not matching generic type `T`
    /// will not be included in the new DataFrame.
    ///
    /// # Example
    /// ```
    /// use dami::prelude::*;
    /// use ndarray::Array2;
    /// let ones:Array2<f64> = Array2::ones((4,4)); // Create a 4*4 matrix
    /// let df = DataFrame::from(ones);
    /// df.apply_map::<f64,_>(f64::sqrt); // Square root all numbers which is still 1 though :)
    /// ```
    pub fn apply_map<T, F>(&self, func: F) -> DataFrame
    where
        T: Clone + Default + 'static,
        F: Clone + Fn(T) -> T,
    {
        self.block.apply_map::<T, _>(func)
    }
    /// Creates a new column from another column and clones the DataFrame
    ///
    /// Existing columns that are reassigned will be preserved.
    /// # Errors
    /// `KeyError` The key does not exist
    /// # Example
    /// ```
    /// use dami::prelude::DataFrame;
    /// use std::collections::HashMap;
    /// use std::convert::TryFrom;
    ///
    /// fn convert_to_f(value:f64)->f64{
    ///     value*9.0/5.0+32.0
    /// }
    /// let mut values:HashMap<&str,Vec<f64>> = [("temp_c",vec![17.0,25.0])].iter().cloned().collect();
    /// let df = DataFrame::try_from(values).unwrap();
    /// /// State the name of the column already in the DataFrame and the new column name.
    /// let temp_f=df.assign("temp_c","temp_f",convert_to_f);
    /// println!("{:?}",temp_f.unwrap());
    /// ```
    /// The above example prints
    /// ```text
    ///     temp_c  temp_f
    /// 0   17.0    62.6
    /// 1   25.0    77.0
    /// ```
    pub fn assign<T, F>(&self, key: &str, name: &str, func: F) -> Result<DataFrame, DataFrameErrors>
    where
        T: Clone + Default + 'static,
        F: Fn(T) -> T,
    {
        match self.block.assign(key, name, func) {
            Ok(block) => Ok(DataFrame::from(block)),
            Err(err) => Err(err),
        }
    }
    /// Like [assign](#method.assign), but performs the assignment in the DataFrame
    #[allow(clippy::missing_errors_doc)]
    pub fn assign_inplace<T, F>(
        &mut self,
        key: &str,
        name: &str,
        func: F,
    ) -> Result<(), DataFrameErrors>
    where
        T: Clone + Default + 'static,
        F: Fn(T) -> T,
    {
        match self.block.assign_inplace(key, name, func) {
            Ok(()) => Ok(()),
            Err(err) => Err(err),
        }
    }
    /// Access a single value for a row/column pair.
    ///
    /// # Arguments
    /// `loc` : A tuple containing the column and row respectively,to access data from
    ///
    /// # Returns
    ///  * `Some(T)` if the value is found
    ///  * `None` if the column name is not found or there exists no type `T` in the DataFrame
    ///
    /// # Panics
    /// If the index does not exist
    pub fn at<T>(&self, loc: (&str, &str)) -> Option<T>
    where
        T: Clone + Default + 'static,
    {
        self.block.at(loc)
    }
    /// Combine this DataFrame with another DataFrame using a func to element-wise combine columns.
    ///
    /// # Arguments
    /// * `other`: A reference to a DataFrame
    /// *`func`: A function that takes two arguments and returns one back
    ///
    /// # Warning
    /// Columns no matching generic type T are by default skipped
    /// and their series will not be included in the resulting DataFrame.
    /// # Example
    /// ```
    /// use dami::prelude::*;
    ///
    /// use std::cmp::max;
    ///
    /// use ndarray::arr2;
    ///
    /// let a = arr2(&[[1, 2, 32],
    ///               [4, 5, 6]]);
    /// let b = arr2(&[[1, 6, 3],
    ///               [42, 5, 6]]);
    /// let df1 = DataFrame::from(a);
    ///
    /// let df2 = DataFrame::from(b);
    /// // Combine the two taking the max values first
    /// let df3=df2.combine::<i32,_>(&df1,max);
    /// println!("{:?}",df3);
    /// ```
    /// Prints
    /// ```text
    ///     0   1   2
    /// 0   1   6   32
    /// 1   42  5   6
    /// ```
    pub fn combine<T, F>(&self, other: &DataFrame, func: F) -> DataFrame
    where
        T: Clone + Default + 'static,
        F: Clone + Fn(T, T) -> T,
    {
        self.block.clone().combine(other, func)
    }
    /// Get the DataTypes  of the underlying block.
    pub fn dtypes(&self) -> HashMap<String, DataTypes, RandomState> {
        self.block.dtypes()
    }
    /// Similar to `apply_map`, but uses parallel iterators to speed up the operation
    ///
    /// # Notes
    /// * If the current DataFrame contains Heterogeneous Data, the elements not matching generic type `T`
    /// will not be included in the new DataFrame.
    /// * The function needs to implement `Send`+`Sync` in order to be shared by multiple threads
    /// * The two traits are implemented by default when the compiler sees fit.
    ///  most functions implement send and sync except those that use raw pointers, Cell, and Rc
    /// see [here]( https://doc.rust-lang.org/nomicon/send-and-sync.html)
    ///
    /// # Example
    /// ```
    /// use dami::prelude::*;
    /// use ndarray::Array2;
    /// let ones:Array2<f64> = Array2::ones((4,4)); // Create a 4*4 matrix
    /// let df = DataFrame::from(ones);
    /// df.par_apply_map::<f64,_>(f64::sqrt); // Square root all numbers which is still 1 though :)
    /// ```
    pub fn par_apply_map<T, F>(&self, func: F) -> DataFrame
    where
        T: Clone + Default + 'static + Send + Sync,
        F: Send + Sync + Clone + Fn(T) -> T,
    {
        self.block.par_apply_map::<T, _>(func)
    }
    /// Get the series at column `col`
    ///
    /// This can be used to fetch individual Series from the DataFrame
    /// # Returns
    /// * Some(series): If the Series\<T> exists in the DataFrame
    /// * None: If the series doesn't exist
    pub fn get<T>(&self, col: &str) -> Option<Series<T>>
    where
        T: Clone + Default + 'static,
    {
        self.block.get(col)
    }
    /// Prints the first `n` elements of the series
    ///
    /// # Example
    /// ```ignore
    /// use ndarray::Array2;
    /// use ndarray_rand::RandomExt;
    /// use ndarray_rand::rand_distr::Uniform;
    /// use ndarray_rand::rand::SeedableRng;
    /// use dami::prelude::*;
    /// use rand_isaac::isaac64::Isaac64Rng;
    ///
    /// let seed = 42;
    /// let mut rng = Isaac64Rng::seed_from_u64(seed);
    /// let df = DataFrame::from(Array2::random_using((50,4),Uniform::new(0., 10.),&mut rng)); // Create a DataFrame from a 2 dimensional array;
    /// df.head(3);
    /// ```
    /// The above code prints
    /// ```text
    ///      0      1      2      3
    ///  0   6.238  6.238  6.238  6.238
    ///  1   1.670  1.670  1.670  1.670
    ///  2   3.508  3.508  3.508  3.508
    /// ```
    /// # Panics
    /// if `n` is greater than the values in the DataFrame
    pub fn head(&self, n: usize) {
        self.block.head(n);
    }
    /// Similar to [`head`](#method.head) but prints to an ecvxr environment
    pub fn head_ecvxr(&self, n: usize) {
        self.block.head_evcxr(n)
    }
    /// Replace values where condition is True
    /// # Syntax
    /// ```ignore
    /// df.mask<type,_>(value:T,cond:F)
    /// ```
    /// # Arguments
    /// * `value`: The new value to add where the `cond` becomes true
    /// * `cond`: A function or closure that takes a value `T` and returns `true` or `false`
    pub fn mask<T, F>(&mut self, value: T, cond: F)
    where
        F: Clone + Fn(T) -> bool,
        T: Clone + Default + 'static,
    {
        self.block.mask(value, cond)
    }
    /// Prints the last `n` elements of the DataFrame
    /// # Panics
    /// If `n` is larger than items in the DataFrame
    /// # Example
    /// ```ignore
    /// use ndarray::Array2;
    /// use ndarray_rand::RandomExt;
    /// use ndarray_rand::rand_distr::Uniform;
    /// use ndarray_rand::rand::SeedableRng;
    /// use dami::prelude::*;
    /// use rand_isaac::isaac64::Isaac64Rng;
    ///
    /// let seed = 42;
    /// let mut rng = Isaac64Rng::seed_from_u64(seed);
    /// let df = DataFrame::from(Array2::random_using((50,4),Uniform::new(0., 10.),&mut rng)); // Create a DataFrame from a 2 dimensional array;
    /// df.tail(4);
    /// ```
    /// The above code prints
    /// ```text
    ///   0      1      2      3
    ///  46  4.852  4.852  4.852  4.852
    ///  47  0.084  0.084  0.084  0.084
    ///  48  2.659  2.659  2.659  2.659
    ///  49  5.218  5.218  5.218  5.218
    /// ```
    pub fn tail(&self, n: usize) {
        self.block.tail(n);
    }
    /// Similar to [`tail`](#method.tail) but prints formatted output in a evcxr environment.
    pub fn tail_evcxr(&self, n: usize) {
        self.block.tail_evcxr(n)
    }
    /// Call `func` on the DataFrame. Producing a DataFrame with transformed values
    ///
    /// # Syntax
    /// ```ignore
    /// df.transform::<current_type,new_type,_>(func:F, axis: bool) // eg df.transform::<f64,f64_>(sqrt,true)
    /// ```
    /// # Arguments
    /// * `T`: Current type in the DataFrame to act on
    /// * `P`: New type generated by function
    /// * `F`: A function that takes an array and returns another array
    /// * `axis`: Whether the function will receive a column or a row as its input array
    /// > If `false`,function is applied column-wise.
    /// > If `true`,function is applied row wise.
    ///  # Returns
    ///  * Some(DataFrame) if the type `T` exists in the DataFrame none if otherwise
    ///
    /// # Notes
    /// * All other columns not of type `T` will be dropped in the new DataFrame
    /// * By Default, column-wise iteration leverages rayon's parallel iterators while row-wise iteration
    /// does not.
    ///
    /// # Panics
    /// if  the function returns different lengths for different columns/rows
    pub fn transform<T, P, F>(&self, axis: bool, func: F) -> Option<DataFrame>
    where
        T: Send + Sync + Default + Clone + 'static,
        F: Clone + Fn(Array1<T>) -> Array1<P> + Sync + Send,
        P: Send + Sync + Clone + Default + 'static,
    {
        self.block.transform(func, axis)
    }
    /// Converts a DataFrame into a 2 dimensional ndarray
    ///
    /// # Returns
    /// * `Array2<T>` if the elements of type `T` exist in the DataFrame
    /// * `None` if there no elements of type `T`
    /// # Example
    /// ```
    /// use ndarray::Array2;
    /// use dami::prelude::*;
    ///
    /// let ones:Array2<f64>=Array2::ones((4,4));// Create a 4 by 4 matrix of ones
    /// let df = DataFrame::from(ones);
    /// println!("{}",df.to_ndarray::<f64>().unwrap()); // Floats are printed as ints in display mode
    /// ```
    /// The above code prints
    /// ```text
    /// [[1, 1, 1, 1],
    ///  [1, 1, 1, 1],
    ///  [1, 1, 1, 1],
    ///  [1, 1, 1, 1]]
    /// ```
    pub fn to_ndarray<T>(&self) -> Option<Array2<T>>
    where
        T: Clone + Default + 'static,
    {
        self.block.to_ndarray()
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
impl From<BlockManager> for DataFrame {
    fn from(mgr: BlockManager) -> Self {
        DataFrame { block: mgr }
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

impl Add for DataFrame {
    type Output = DataFrame;

    fn add(self, rhs: Self) -> Self::Output {
        DataFrame::from(self.block + rhs.block)
    }
}
impl Sub for DataFrame {
    type Output = DataFrame;

    fn sub(self, rhs: Self) -> Self::Output {
        DataFrame::from(self.block - rhs.block)
    }
}
impl Mul for DataFrame {
    type Output = DataFrame;

    fn mul(self, rhs: Self) -> Self::Output {
        DataFrame::from(self.block * rhs.block)
    }
}
impl Div for DataFrame {
    type Output = DataFrame;

    fn div(self, rhs: Self) -> Self::Output {
        DataFrame::from(self.block / rhs.block)
    }
}
