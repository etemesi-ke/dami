use crate::core::series::Series;

use ndarray::{Array1, Array2};
use rayon::prelude::*;
mod stats;
#[derive(Default, Clone)]
pub struct Block<T> {
    pub data: Vec<Series<T>>,
    pub names: Vec<String>,
}
impl<T: Clone + 'static + Default> From<Vec<Series<T>>> for Block<T> {
    fn from(data: Vec<Series<T>>) -> Self {
        let names: Vec<String> = data.iter().map(Series::get_name).collect();
        Block { data, names }
    }
}
impl<T: Clone + 'static + Default> Block<T> {
    /// Apply a function to all the Series in the Block
    pub fn par_apply<F: Clone + Fn(T) -> T>(&self, func: F) -> Block<T>
    where
        T: Send,
        F: Sync + Send,
    {
        Block::from(
            self.data
                .clone()
                .into_par_iter()
                .map(|f| f.apply(func.clone()))
                .collect::<Vec<Series<T>>>(),
        )
    }
    /// Apply a function using parallel threads
    pub fn apply_map<F: Clone + Fn(T) -> T>(&self, func: F) -> Block<T> {
        Block::from(
            self.data
                .clone()
                .into_iter()
                .map(|f| f.apply(func.clone()))
                .collect::<Vec<Series<T>>>(),
        )
    }
    /// Apply a function axis_wise or column-wise
    pub fn apply<F: Clone + Fn(Array1<T>) -> T>(&self, func: F, axis: bool) -> Series<T> {
        // Row wise
        if axis {
            let mut results_vec = Vec::with_capacity(self.data[0].len());
            for i in 0..self.data[0].len() {
                // Hold values of an axis temporarily.
                let mut axis = vec![];
                for j in 0..self.data.len() {
                    axis.push(self.data[j][i].clone());
                }
                let arr_axis = Array1::from(axis);
                results_vec.push(func.clone()(arr_axis));
            }
            Series::from(results_vec)
        }
        // Column wise
        else {
            let mut results_vec = Vec::with_capacity(self.data.len());
            for i in 0..self.data.len() {
                results_vec.push(func.clone()(self.data[i].to_ndarray()));
            }
            Series::from(results_vec)
        }
    }

    /// Apply a function to a series in place using parralell iterators for speed
    pub fn apply_inplace<F: Clone + Fn(T) -> T>(&mut self, func: F)
    where
        T: Send,
        F: Sync + Send,
    {
        self.data = self
            .data
            .clone()
            .into_par_iter()
            .map(|f| f.apply(func.clone()))
            .collect::<Vec<Series<T>>>();
    }
    pub fn as_type<P: From<T> + Clone + 'static + Default>(&self) -> Block<P>
    where
        T: Send + Sync,
        P: Send + Sync,
    {
        let cols = self
            .data
            .clone()
            .into_par_iter()
            .map(|f| f.as_type())
            .collect::<Vec<Series<P>>>();
        let mut block = Block::from(cols);
        block.push_names(self.names.clone());
        block
    }
    pub fn drop_cols(&mut self, name: &str) {
        let idx = self.names.iter().position(|f| f == name).unwrap();
        self.names.remove(idx);
        self.data.remove(idx);
    }
    /// Push a new Series to the block
    pub fn push(&mut self, other: Series<T>) {
        if !self.data.is_empty() {
            assert_eq!(
                self.data[0].len(),
                other.len(),
                "This block contains data of length {} but new series contains data of length {}",
                self.data[0].len(),
                other.len()
            );
        }
        self.names.push(other.get_name());
        self.data.push(other)
    }
    pub fn push_names(&mut self, names: Vec<String>) {
        self.names = names
    }
    /// Get the value at the series `index2` in block `index1`
    /// # Panics
    /// If the value at either `index1` or `index2` doesn't exist
    pub fn get_value_at(&self, index1: usize, index2: usize) -> T {
        self.data[index1][index2].clone()
    }
    /// Mask values
    #[allow(clippy::needless_pass_by_value)]
    pub fn mask<F>(&mut self, value: T, func: F)
    where
        F: Clone + Fn(T) -> bool,
    {
        for series in &mut self.data {
            series.mask_inplace(value.clone(), func.clone())
        }
    }
    pub fn get(&self, idx: usize) -> Series<T> {
        self.data[idx].clone()
    }
    pub fn get_str_value_at(&self, idx: usize, idx2: &str) -> T {
        self.data[idx][idx2].clone()
    }
    /// Get a series at a particular name
    pub fn get_series_at_name(&self, name: &str) -> Series<T> {
        self.data[self.names.iter().position(|f| f == name).unwrap()].clone()
    }
    /// Convert all the values in the `Block` into an `Array2<T>`
    pub fn to_ndarray(&self) -> Array2<T> {
        // Prevent reallocation by preallocating the vector
        let mut temp_vec = Vec::with_capacity(self.data.len() * self.data[0].len());
        self.data
            .iter()
            .for_each(|f| temp_vec.extend_from_slice(&f.to_vec()));
        let array2 = Array2::from_shape_vec((self.data[0].len(), self.data.len()), temp_vec);
        array2.unwrap()
    }
    #[allow(non_snake_case)]
    pub fn T(&self) -> Array2<T> {
        let mut holder = Vec::with_capacity(self.data.len() * self.data[0].len());
        for elm in &self.data {
            holder.extend_from_slice(elm.to_vec().as_slice())
        }
        let arr2 = Array2::from_shape_vec((self.data[0].len(), self.data.len()), holder).unwrap();
        arr2.reversed_axes()
    }
    // Transform a block to another block
    pub fn transform<F, P>(&self, func: F, axis: bool) -> Block<P>
    where
        T: Send + Sync,
        F: Clone + Fn(Array1<T>) -> Array1<P> + Sync + Send,
        P: Send + Sync + Clone + Default + 'static,
    {
        if axis {
            let mut results_vec = Vec::with_capacity(self.data[0].len());
            unsafe {
                for i in 0..self.data[0].len() {
                    // Hold values of an axis temporarily.
                    let mut axis = vec![];
                    for j in 0..self.data.len() {
                        // I am pretty sure it exists so i can get it un -safely
                        axis.push(self.data.get_unchecked(j).get(i).unwrap().clone());
                    }
                    let arr_axis = Array1::from(axis);
                    results_vec.push(Series::from(func.clone()(arr_axis)));
                }
            }
            Block::from(results_vec)
        } else {
            Block::from(
                self.data
                    .clone()
                    .into_par_iter()
                    .map(|f| Series::from(func.clone()(f.to_ndarray())))
                    .collect::<Vec<Series<P>>>(),
            )
        }
    }
}
