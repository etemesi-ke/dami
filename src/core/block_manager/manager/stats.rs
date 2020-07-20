#![cfg(feature="stats")]
#![allow(clippy::needless_pass_by_value)]
use crate::core::block_manager::manager::Block;
use crate::core::dataframe::DataFrame;
use crate::core::series::Series;
use num_traits::{Float, FromPrimitive, Num};
use plotly::common::Mode;
use plotly::{BoxPlot, Plot};
use rayon::prelude::*;
use serde::Serialize;
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

impl<T> Block<T>
where
    T: Num + Serialize + Default + Clone + 'static,
{
    pub fn plot_bar(&self, plot: &mut Plot, index: Vec<String>) {
        for (len, individual) in self.data.iter().enumerate() {
            plot.add_trace(individual.plot_bar(&self.names[len], index.clone()));
        }
    }
    pub fn plot_line(&self, plot: &mut Plot, index: Vec<String>) {
        for (len, individual) in self.data.iter().enumerate() {
            plot.add_trace(individual.plot_line(Mode::Lines, &self.names[len], index.clone()));
        }
    }
    pub fn plot_hist(&self, plot: &mut Plot) {
        for (len, individual) in self.data.iter().enumerate() {
            plot.add_trace(individual.plot_histogram(&self.names[len]));
        }
    }

    pub fn plot_h_hist(&self, plot: &mut Plot) {
        for (len, individual) in self.data.iter().enumerate() {
            plot.add_trace(individual.plot_horizontal_histogram(&self.names[len]));
        }
    }
    pub fn plot_dots(&self, plot: &mut Plot, index: Vec<String>) {
        for (len, individual) in self.data.iter().enumerate() {
            plot.add_trace(individual.plot_line(Mode::Markers, &self.names[len], index.clone()));
        }
    }
    pub fn plot_box(&self, plot: &mut Plot) {
        for (len, individual) in self.data.iter().enumerate() {
            let box_plot = BoxPlot::new(individual.to_vec()).name(&self.names[len]);
            plot.add_trace(box_plot)
        }
    }
}
