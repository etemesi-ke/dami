use crate::core::series::Series;
use crate::enums::DataFrameErrors::KeyError;
use crate::enums::{DataFrameErrors, DataTypes};
use crate::prelude::*;
use ndarray::Array2;
use num_traits::Zero;
use prettytable::format::consts::FORMAT_CLEAN;
use prettytable::{Cell, Row, Table};
use std::any::Any;

impl DataFrame {
    /// Apply a function to the underlying series and modify it in-place
    ///
    /// # Arguments
    /// * `T`: Generic type
    /// > Since Rust needs to know what type the function/closure is acting on T: becomes that type
    /// this causes some quirky syntax when calling apply on a DataFrame...
    /// > > *but it's worth it*
    /// # Note
    /// This does not return a new DataFrame. But modifies the underlying series of type T
    /// This should be the proffered method for large DataFrames as it avoids expensive cloning of underlying
    /// series
    ///  # Example
    /// ```
    /// use dami::prelude::DataFrame;
    /// use dami::prelude::Series;
    /// use num_traits::real::Real;
    /// fn square_ints(num:i32)->i32{
    ///    num.pow(2)
    /// }
    /// fn main(){
    ///     let s1 = Series::from([0,1,2,3,4,5]);
    ///     let s2 = Series::from([2,3,4,5,6,7]);
    ///     let mut df = DataFrame::new();
    ///     df.add_series(s1,true).unwrap();
    ///     df.add_series(s2,true).unwrap();
    ///     // Quirky syntax
    ///     df.apply::<i32,_>(square_ints);
    /// }
    /// ```
    pub fn apply<T: Default + Clone + 'static, F>(&mut self, func: F)
    where
        F: Clone + Fn(T) -> T,
    {
        for i in &self.get_order() {
            if self.get_mut_series::<T>(i).is_some() {
                self.get_mut_series::<T>(i)
                    .unwrap()
                    .apply_inplace(func.clone());
            }
        }
    }
    /// Creates a new column from another column and clones the DataFrame
    ///
    /// Existing columns that are reassigned will be preserved.
    /// # Errors
    /// [`KeyError`] The key does not exist
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
    pub fn assign<T: Clone + Default + 'static, F: Fn(T) -> T>(
        &self,
        key: &str,
        name: &str,
        func: F,
    ) -> Result<DataFrame, DataFrameErrors> {
        match self.get_series::<T>(key) {
            Some(series) => {
                let mut new_series = series.apply(func);
                new_series.set_name(name);
                let mut df = self.clone();
                df.add_series(new_series, true).unwrap();
                Ok(df)
            }
            None => Err(KeyError(format!("key {} does not exist", key))),
        }
    }
    /// Like [assign](#method.assign) but performs the assignment in the current DataFrame
    pub fn assign_inplace<T: Clone + Default + 'static, F: Fn(T) -> T>(
        &mut self,
        key: &str,
        name: &str,
        func: F,
    ) -> Result<(), DataFrameErrors> {
        match self.get_series::<T>(key) {
            Some(series) => {
                let mut new_series = series.apply(func);
                new_series.set_name(name);
                self.add_series(new_series, true).unwrap();
                Ok(())
            }
            None => Err(KeyError(format!("key {} does not exist", key))),
        }
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
    pub fn combine<T: Clone + Default + 'static, F: Clone + Fn(T, T) -> T>(
        &self,
        other: &DataFrame,
        func: F,
    ) -> DataFrame {
        let mut df = DataFrame::new();
        for i in &self.get_order() {
            let me = self.get_series::<T>(i).unwrap();
            if let Some(series) = other.get_series::<T>(i) {
                df.add_series(me.combine(series, func.clone()), true)
                    .unwrap();
                continue;
            };
        }
        df
    }
    /// Counts the number on non-NA values for each column
    ///
    /// # Returns
    ///  A Series containing Na values for each column.
    pub fn count(&self) -> Series<usize> {
        let mut map: Vec<(String, usize)> = Vec::new();
        for i in self.get_order() {
            let dtype = self.dtypes.get(i.as_str()).unwrap();
            match dtype {
                DataTypes::F64 => map.push((
                    i.clone(),
                    self.get_series::<f32>(i.as_str()).unwrap().count(),
                )),
                DataTypes::F32 => map.push((
                    i.clone(),
                    self.get_series::<f32>(i.as_str()).unwrap().count(),
                )),
                _ => continue,
            }
        }
        Series::from(map)
    }
    /// Returns a DataFrame with specific indexes dropped.
    /// This doesn't modify the underlying series but creates a new dataframe.
    ///
    /// # Panics
    /// If a series contains different labels causing drop to return different lengths for different series.
    /// The subsequent `add_series()` function will fail
    pub fn drop(&self, labels: &[&str]) -> DataFrame {
        let mut frame = DataFrame::new();
        for i in &self.order {
            let dtype = self.dtypes.get(i).unwrap();
            match dtype {
                DataTypes::F64 => {
                    let series = self.get_series::<f64>(i).unwrap();
                    frame.add_series(series.drop(labels), true).unwrap();
                }
                DataTypes::BOOL => {
                    let series = self.get_series::<bool>(i).unwrap();
                    frame.add_series(series.drop(labels), true).unwrap();
                }
                DataTypes::F32 => {
                    let series = self.get_series::<f32>(i).unwrap();
                    frame.add_series(series.drop(labels), true).unwrap();
                }
                DataTypes::I32 => {
                    let series = self.get_series::<i32>(i).unwrap();
                    frame.add_series(series.drop(labels), true).unwrap();
                }
                DataTypes::I64 => {
                    let series = self.get_series::<i64>(i).unwrap();
                    frame.add_series(series.drop(labels), true).unwrap();
                }
                DataTypes::I128 => {
                    let series = self.get_series::<i128>(i).unwrap();
                    frame.add_series(series.drop(labels), true).unwrap();
                }
                DataTypes::STR => {
                    let series = self.get_series::<&str>(i).unwrap();
                    frame.add_series(series.drop(labels), true).unwrap();
                }
                DataTypes::STRING => {
                    let series = self.get_series::<String>(i).unwrap();
                    frame.add_series(series.drop(labels), true).unwrap();
                }
                DataTypes::OBJECT => continue,
            }
        }
        frame
    }
    /// Like [drop] but actually modifies the series in place.
    ///
    /// [drop]: #method.drop
    pub fn drop_inplace(&mut self, labels: &[&str]) {
        for i in &self.get_order() {
            let dtype = self.dtypes.get(i).unwrap();
            match dtype {
                DataTypes::F64 => {
                    self.get_mut_series::<f64>(i).unwrap().drop_inplace(labels);
                }
                DataTypes::BOOL => {
                    self.get_mut_series::<bool>(i).unwrap().drop_inplace(labels);
                }
                DataTypes::F32 => {
                    self.get_mut_series::<f32>(i).unwrap().drop_inplace(labels);
                }
                DataTypes::I32 => {
                    self.get_mut_series::<i32>(i).unwrap().drop_inplace(labels);
                }
                DataTypes::I64 => {
                    self.get_mut_series::<i64>(i).unwrap().drop_inplace(labels);
                }
                DataTypes::I128 => {
                    self.get_mut_series::<i128>(i).unwrap().drop_inplace(labels);
                }
                DataTypes::STR => {
                    self.get_mut_series::<&str>(i).unwrap().drop_inplace(labels);
                }
                DataTypes::STRING => {
                    self.get_mut_series::<String>(i)
                        .unwrap()
                        .drop_inplace(labels);
                }
                DataTypes::OBJECT => continue,
            }
        }
        self.modify_len_and_index();
    }
    /// Modify both the length and the index of the DataFrame
    fn modify_len_and_index(&mut self) {
        for i in &self.get_order() {
            let dtype = self.dtypes.get(i).unwrap();
            match dtype {
                DataTypes::F64 => {
                    let series = self.container.get::<Series<f64>, _>(i).unwrap();
                    self.len = series.len();
                    self.index = series.get_index();
                    break;
                }
                DataTypes::BOOL => {
                    let series = self.container.get::<Series<bool>, _>(i).unwrap();
                    self.len = series.len();
                    self.index = series.get_index();
                    break;
                }
                DataTypes::F32 => {
                    let series = self.container.get::<Series<f32>, _>(i).unwrap();
                    self.len = series.len();
                    self.index = series.get_index();
                    break;
                }
                DataTypes::I32 => {
                    let series = self.container.get::<Series<i32>, _>(i).unwrap();
                    self.len = series.len();
                    self.index = series.get_index();
                    break;
                }
                DataTypes::I64 => {
                    let series = self.container.get::<Series<i64>, _>(i).unwrap();
                    self.len = series.len();
                    self.index = series.get_index();
                    break;
                }
                DataTypes::I128 => {
                    let series = self.container.get::<Series<i128>, _>(i).unwrap();
                    self.len = series.len();
                    self.index = series.get_index();
                    break;
                }
                DataTypes::STR => {
                    let series = self.container.get::<Series<&str>, _>(i).unwrap();
                    self.len = series.len();
                    self.index = series.get_index();
                    break;
                }
                DataTypes::STRING => {
                    let series = self.container.get::<Series<String>, _>(i).unwrap();
                    self.len = series.len();
                    self.index = series.get_index();
                    break;
                }
                DataTypes::OBJECT => continue,
            }
        }
    }
    /// Get the underlying index
    pub fn get_index(&self) -> Vec<String> {
        self.index.clone()
    }
    /// Get the underlying DataType
    pub fn get_dtype_at_key(&self, key: &str) -> Option<&DataTypes> {
        self.dtypes.get(key)
    }
    /// Get the underlying order of how the items were inserted.
    pub fn get_order(&self) -> Vec<String> {
        self.order.clone()
    }
    /// Get a series from the DataFrame
    pub fn get_series<T: Default + 'static + Clone>(&self, key: &str) -> Option<&Series<T>> {
        self.container.get::<Series<T>, _>(key)
    }

    /// Get the series at the given index.
    ///
    /// The type of the returned series is specified by the parameter T
    ///  # Example
    /// ```
    /// use dami::prelude::*;
    /// let df = DataFrame::from(vec![vec![1,2,3,4,5,6],vec![7,8,9,10,11,12]]);
    /// // The compiler will infer the type;
    /// let col_1:Series<i32> = df.index(0);
    ///    // Alternatively,
    /// let col_2 = df.index::<i32>(2);
    /// assert_eq!(col_1,Series::from([1,7]));
    /// ```
    pub fn index<T: Clone + Default + 'static>(&self, pos: usize) -> Series<T> {
        let idx = self
            .order
            .get(pos)
            .unwrap_or_else(|| panic!("No index at position {:?}", pos));
        self.get_series::<T>(idx).unwrap().clone()
    }
    /// Get a mutable reference to the series in the DataFrame
    pub fn get_mut_series<T: Default + 'static + Clone>(
        &mut self,
        index: &str,
    ) -> Option<&mut Series<T>> {
        self.container.get_mut::<Series<T>, _>(index)
    }
    fn print_frame(&self, start: usize, end: usize) {
        let mut table = Table::new();
        table.set_format(*FORMAT_CLEAN);
        let mut title = vec![Cell::new("  ")];
        for i in &self.order {
            title.push(Cell::new(i))
        }
        table.set_titles(Row::new(title));
        for i in start..end {
            let mut new_row = vec![];
            for item in &self.order {
                match self.dtypes.get(item).unwrap() {
                    DataTypes::STRING => {
                        let val =
                            format!("{:?}", self.get_series::<String>(item.as_str()).unwrap()[i]);
                        new_row.push(Cell::new(&val))
                    }
                    DataTypes::I32 => {
                        let val =
                            format!("{:?}", self.get_series::<i32>(item.as_str()).unwrap()[i]);
                        new_row.push(Cell::new(&val))
                    }
                    DataTypes::I64 => {
                        let val =
                            format!("{:?}", self.get_series::<i64>(item.as_str()).unwrap()[i]);
                        new_row.push(Cell::new(&val))
                    }
                    DataTypes::I128 => {
                        let val =
                            format!("{:?}", self.get_series::<i128>(item.as_str()).unwrap()[i]);
                        new_row.push(Cell::new(&val))
                    }
                    DataTypes::F32 => {
                        let val = format!(
                            "{:0<3.3}",
                            self.get_series::<f32>(item.as_str()).unwrap()[i]
                        );
                        new_row.push(Cell::new(&val))
                    }
                    DataTypes::F64 => {
                        let val = format!(
                            "{:0<3.3}",
                            self.get_series::<f64>(item.as_str()).unwrap()[i]
                        );
                        new_row.push(Cell::new(&val))
                    }
                    DataTypes::BOOL => {
                        let val = format!("{}", self.get_series::<bool>(item.as_str()).unwrap()[i]);
                        new_row.push(Cell::new(&val))
                    }
                    DataTypes::STR => {
                        let val = format!(
                            "{:?}",
                            self.get_series::<&'static str>(item.as_str()).unwrap()[i]
                        );
                        new_row.push(Cell::new(&val))
                    }
                    // We don't know you ..
                    DataTypes::OBJECT => continue,
                }
            }
            new_row.insert(0, Cell::new(&self.index[i]));
            table.add_row(Row::new(new_row));
        }
        println!("{}", table.to_string());
    }
    /// Prints the first N items in the array
    ///
    /// # Panics
    /// If n is larger than the number of elements in the array
    pub fn head(&self, n: usize) {
        self.print_frame(0, n)
    }
    /// Remove a Series from the array.
    pub fn remove(&mut self, value: &str) {
        self.container.remove(value);
        self.dtypes.remove(value);
        self.order.retain(|f| f.as_str() != value);
    }
    /// Prints the last N items in the array
    ///
    /// # Panics
    /// If n is larger than the number of elements in the array
    pub fn tail(&self, n: usize) {
        let start = self.len - n;
        self.print_frame(start, self.len);
    }
    /// Return an item and drop it from a Frame.
    pub fn pop<T: Clone + Default + 'static>(&mut self, name: &str) -> Option<Box<dyn Any>> {
        self.dtypes.remove(name);
        self.order.pop();
        self.container.remove(name)
    }

    /// For all type T's (including floats and ints) return an array2 of its elements
    /// # Arguments
    /// `cols`: The number of columns to preallocate the array for
    /// > This is useful when the DataFrame contains heterogeneous columns and you want only one type
    pub fn to_ndarray<T: Zero + Clone + Default + 'static>(&self, cols: usize) -> Array2<T> {
        let mut array2 = Array2::zeros((self.len, cols));
        let mut counter = 0;
        for i in &self.order {
            if let Some(series) = self.get_series::<T>(i) {
                for (len, item) in series.to_ndarray().iter().enumerate() {
                    array2[[len, counter]] = item.to_owned();
                }
                counter += 1;
            }
        }
        array2
    }
}
impl Clone for DataFrame {
    /// A lazy implementation of the default clone.
    /// #  Note
    /// This drops any Series whose dtype is `OBJECT` due to some limitations on the underlying traits
    fn clone(&self) -> Self {
        let mut frame = DataFrame::new();
        for i in &self.order {
            let dtype = self.dtypes.get(i).unwrap();
            match dtype {
                DataTypes::F64 => {
                    let series = self.get_series::<f64>(i).unwrap();
                    frame.add_series(series.clone(), true).unwrap();
                }
                DataTypes::BOOL => {
                    let series = self.get_series::<bool>(i).unwrap();
                    frame.add_series(series.clone(), true).unwrap();
                }
                DataTypes::F32 => {
                    let series = self.get_series::<f32>(i).unwrap();
                    frame.add_series(series.clone(), true).unwrap();
                }
                DataTypes::I32 => {
                    let series = self.get_series::<i32>(i).unwrap();
                    frame.add_series(series.clone(), true).unwrap();
                }
                DataTypes::I64 => {
                    let series = self.get_series::<i64>(i).unwrap();
                    frame.add_series(series.clone(), true).unwrap();
                }
                DataTypes::I128 => {
                    let series = self.get_series::<i128>(i).unwrap();
                    frame.add_series(series.clone(), true).unwrap();
                }
                DataTypes::STR => {
                    let series = self.get_series::<&str>(i).unwrap();
                    frame.add_series(series.clone(), true).unwrap();
                }
                DataTypes::STRING => {
                    let series = self.get_series::<String>(i).unwrap();
                    frame.add_series(series.clone(), true).unwrap();
                }
                DataTypes::OBJECT => continue,
            }
        }
        frame
    }
}
