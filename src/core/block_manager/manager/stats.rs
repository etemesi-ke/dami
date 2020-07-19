use crate::core::block_manager::manager::Block;
use crate::core::dataframe::DataFrame;
use crate::core::series::Series;
use num_traits::{Float, FromPrimitive};
use rayon::prelude::*;
use std::iter::Sum;

impl<T: Float + Clone + FromPrimitive + 'static + Sync + Send + Default> Block<T> {
    pub fn mean(&self) -> Series<T> {
        let values: Vec<T> = self
            .data
            .clone()
            .into_par_iter()
            .map(|f| f.mean().unwrap())
            .collect();
        Series::from(values)
    }
    pub fn max(&self) -> Series<T> {
        let values: Vec<T> = self
            .data
            .clone()
            .into_par_iter()
            .map(|f| *f.max().unwrap())
            .collect();
        Series::from(values)
    }
    pub fn min(&self) -> Series<T> {
        let values: Vec<T> = self
            .data
            .clone()
            .into_par_iter()
            .map(|f| *f.min().unwrap())
            .collect();
        Series::from(values)
    }
    pub fn argmin(&self) -> Series<usize> {
        let values: Vec<usize> = self
            .data
            .clone()
            .into_par_iter()
            .map(|f| f.argmin().unwrap())
            .collect();
        Series::from(values)
    }
    pub fn kurtosis(&self) -> Series<T> {
        let values: Vec<T> = self
            .data
            .clone()
            .into_par_iter()
            .map(|f| f.kurtosis().unwrap())
            .collect();
        Series::from(values)
    }
    pub fn geometric_mean(&self) -> Series<T> {
        let values: Vec<T> = self
            .data
            .clone()
            .into_par_iter()
            .map(|f| f.geometric_mean().unwrap())
            .collect();
        Series::from(values)
    }
    pub fn skewness(&self) -> Series<T> {
        let values: Vec<T> = self
            .data
            .clone()
            .into_par_iter()
            .map(|f| f.mean().unwrap())
            .collect();
        Series::from(values)
    }
    pub fn central_moment(&self, order: u16) -> Series<T> {
        let values: Vec<T> = self
            .data
            .clone()
            .into_par_iter()
            .map(|f| f.central_moment(order).unwrap())
            .collect();
        Series::from(values)
    }
    pub fn central_moments(&self, order: u16) -> DataFrame {
        let values: Vec<Vec<T>> = self
            .data
            .clone()
            .into_par_iter()
            .map(|f| f.central_moments(order).unwrap())
            .collect();
        DataFrame::from(values)
    }
    pub fn ptsdev(&self) -> Series<T>
    where
        T: Sum,
    {
        let values: Vec<T> = self
            .data
            .clone()
            .into_par_iter()
            .map(|f| f.pstdev())
            .collect();
        Series::from(values)
    }
    pub fn stdev(&self) -> Series<T>
    where
        T: Sum,
    {
        let values: Vec<T> = self
            .data
            .clone()
            .into_par_iter()
            .map(|f| f.stdev())
            .collect();
        Series::from(values)
    }
    pub fn variance(&self) -> Series<T>
    where
        T: Sum,
    {
        let values: Vec<T> = self
            .data
            .clone()
            .into_par_iter()
            .map(|f| f.variance())
            .collect();
        Series::from(values)
    }
}
