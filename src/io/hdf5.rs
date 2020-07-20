//! Top level manipulations for HDF5 files in dami
//!
//! This module exports functions used in handling of hdf5  files
use crate::core::dataframe::DataFrame;
use crate::core::series::Series;
use hdf5::{File, H5Type};

use ndarray::{Array1, Array2};

/// Read a HDF5 dataSet to a dami [`Series`]
/// # Arguments
/// * `Generic` T: Which derives the [`Clone`] and  HDF5Type trait, for the latter
/// see the example at [hdf5](https://github.com/aldanor/hdf5-rust/blob/master/examples/simple.rs)
///
/// * `file`:`str` The string pointing to a HDF5 file in the local filesystem
///
/// * `dataset`:`str`: The dataset name to load
/// # Returns
/// [`Series`] with the underlying array as the dataset
/// # Panics
/// * If the file cannot be opened
///
/// * If the dataset is not a one dimensional array
///
/// * If the array cannot be converted into type `T`
pub fn read_dataset_to_series<T: Clone + H5Type + Default>(file: &str, dataset: &str) -> Series<T> {
    let file = File::open(file).unwrap();
    let dataset = file
        .dataset(dataset)
        .expect("Dataset could not be loaded \n");
    let array: Array1<T> = dataset.read_1d().unwrap();
    return Series::from(array);
}
/// Read hdf5 to a DataFrame
pub fn read_hdf5<T: Clone + H5Type + Default>(file: &str, dataset: &str) -> DataFrame {
    let file = File::open(file).unwrap();
    let dataset = file.dataset(dataset).expect("Dataset could not be loaded");
    let arr: Array2<T> = dataset
        .read_2d()
        .expect("Could not read DataSet to 2-D array");
    DataFrame::from(arr)
}
