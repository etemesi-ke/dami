use crate::core::series::traits::floats::SeriesFloat;
use crate::enums::DataTypes;
use crate::prelude::*;
impl DataFrame {
    /// Returns whether all elements are true
    pub fn all(&self) -> Series<bool> {
        let mut values: Vec<(&str, bool)> = Vec::new();
        for i in &self.order {
            let dtype = self.dtypes.get(i).unwrap();
            match dtype {
                DataTypes::F64 => {
                    let series = self.container.get::<Series<f64>, _>(i).unwrap();
                    values.push((i.as_str(), series.all()));
                }
                DataTypes::F32 => {
                    let series = self.container.get::<Series<f32>, _>(i).unwrap();
                    values.push((i.as_str(), series.all()));
                }
                DataTypes::I32 => {
                    let series = self.container.get::<Series<i32>, _>(i).unwrap();
                    values.push((i.as_str(), series.all()));
                }
                DataTypes::I64 => {
                    let series = self.container.get::<Series<i64>, _>(i).unwrap();
                    values.push((i.as_str(), series.all()));
                }
                DataTypes::I128 => {
                    let series = self.container.get::<Series<i128>, _>(i).unwrap();
                    values.push((i.as_str(), series.all()));
                }
                _ => continue,
            }
        }
        Series::from(values)
    }
    /// Returns whether any element is true
    pub fn any(&self) -> Series<bool> {
        let mut values: Vec<(&str, bool)> = Vec::new();
        for i in &self.order {
            let dtype = self.dtypes.get(i).unwrap();
            match dtype {
                DataTypes::F64 => {
                    let series = self.container.get::<Series<f64>, _>(i).unwrap();
                    values.push((i.as_str(), series.any()));
                }
                DataTypes::F32 => {
                    let series = self.container.get::<Series<f32>, _>(i).unwrap();
                    values.push((i.as_str(), series.any()));
                }
                DataTypes::I32 => {
                    let series = self.container.get::<Series<i32>, _>(i).unwrap();
                    values.push((i.as_str(), series.any()));
                }
                DataTypes::I64 => {
                    let series = self.container.get::<Series<i64>, _>(i).unwrap();
                    values.push((i.as_str(), series.any()));
                }
                DataTypes::I128 => {
                    let series = self.container.get::<Series<i128>, _>(i).unwrap();
                    values.push((i.as_str(), series.any()));
                }
                _ => continue,
            }
        }
        Series::from(values)
    }
    /// Calculate the Cumulative sum of the values in the DataFrame
    pub fn cum_sum(&self) -> DataFrame {
        let mut df = DataFrame::new();
        for i in &self.order {
            let dtype = self.dtypes.get(i).unwrap();
            match dtype {
                DataTypes::F64 => {
                    let series = self.container.get::<Series<f64>, _>(i).unwrap();
                    df.add_series(series.cum_sum(), true).unwrap();
                }
                DataTypes::F32 => {
                    let series = self.container.get::<Series<f32>, _>(i).unwrap();
                    df.add_series(series.cum_sum(), true).unwrap();
                }
                DataTypes::I64 => {
                    let series = self.container.get::<Series<i64>, _>(i).unwrap();
                    df.add_series(series.cum_sum(), true).unwrap();
                }
                DataTypes::I32 => {
                    let series = self.container.get::<Series<i32>, _>(i).unwrap();
                    df.add_series(series.cum_sum(), true).unwrap();
                }
                DataTypes::I128 => {
                    let series = self.container.get::<Series<i128>, _>(i).unwrap();
                    df.add_series(series.cum_sum(), true).unwrap();
                }
                _ => continue,
            }
        }
        df
    }
    /// Calculate the cumulative product of the values in the columns
    pub fn cum_prod(&self, skip_na: bool) -> DataFrame {
        let mut df = DataFrame::new();
        for i in &self.order {
            let dtype = self.dtypes.get(i).unwrap();
            match dtype {
                DataTypes::F64 => {
                    let series = self.container.get::<Series<f64>, _>(i).unwrap();
                    df.add_series(series.cum_prod(skip_na), true).unwrap();
                }
                DataTypes::F32 => {
                    let series = self.container.get::<Series<f32>, _>(i).unwrap();
                    df.add_series(series.cum_prod(skip_na), true).unwrap();
                }
                DataTypes::I64 => {
                    let series = self.container.get::<Series<i64>, _>(i).unwrap();
                    df.add_series(series.cum_prod(), true).unwrap();
                }
                DataTypes::I32 => {
                    let series = self.container.get::<Series<i32>, _>(i).unwrap();
                    df.add_series(series.cum_prod(), true).unwrap();
                }
                DataTypes::I128 => {
                    let series = self.container.get::<Series<i128>, _>(i).unwrap();
                    df.add_series(series.cum_prod(), true).unwrap();
                }
                _ => continue,
            }
        }
        df
    }
    ///Calculate the cumulative min of the series in the DataFrame.
    pub fn cum_min(&self) -> DataFrame {
        let mut df = DataFrame::new();
        for i in &self.order {
            let dtype = self.dtypes.get(i).unwrap();
            match dtype {
                DataTypes::F64 => {
                    let series = self.container.get::<Series<f64>, _>(i).unwrap();
                    df.add_series(series.cum_min(), true).unwrap();
                }
                DataTypes::F32 => {
                    let series = self.container.get::<Series<f32>, _>(i).unwrap();
                    df.add_series(series.cum_min(), true).unwrap();
                }
                DataTypes::I64 => {
                    let series = self.container.get::<Series<i64>, _>(i).unwrap();
                    df.add_series(series.cum_min(), true).unwrap();
                }
                DataTypes::I32 => {
                    let series = self.container.get::<Series<i32>, _>(i).unwrap();
                    df.add_series(series.cum_min(), true).unwrap();
                }
                DataTypes::I128 => {
                    let series = self.container.get::<Series<i128>, _>(i).unwrap();
                    df.add_series(series.cum_min(), true).unwrap();
                }
                _ => continue,
            }
        }
        df
    }
    /// Calculate the cumulative max of the Series in the DataFrame.
    pub fn cum_max(&self) -> DataFrame {
        let mut df = DataFrame::new();
        for i in &self.order {
            let dtype = self.dtypes.get(i).unwrap();
            match dtype {
                DataTypes::F64 => {
                    let series = self.container.get::<Series<f64>, _>(i).unwrap();
                    df.add_series(series.cum_max(), true).unwrap();
                }
                DataTypes::F32 => {
                    let series = self.container.get::<Series<f32>, _>(i).unwrap();
                    df.add_series(series.cum_max(), true).unwrap();
                }
                DataTypes::I64 => {
                    let series = self.container.get::<Series<i64>, _>(i).unwrap();
                    df.add_series(series.cum_max(), true).unwrap();
                }
                DataTypes::I32 => {
                    let series = self.container.get::<Series<i32>, _>(i).unwrap();
                    df.add_series(series.cum_max(), true).unwrap();
                }
                DataTypes::I128 => {
                    let series = self.container.get::<Series<i128>, _>(i).unwrap();
                    df.add_series(series.cum_max(), true).unwrap();
                }
                _ => continue,
            }
        }
        df
    }
}
