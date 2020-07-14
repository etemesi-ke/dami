#[macro_use]
extern crate dami;
#[macro_use]
extern crate ndarray;
use dami::core::dataframe::DataFrame;
use dami::core::series::traits::floats::SeriesFloat;
use dami::core::series::traits::ints::SeriesInt;
use dami::core::series::traits::strings::Str;
use dami::core::series::Series;
use dami::enums::DataTypes;
use dami::io::parser::read_csv;
use ndarray::arr1;
use std::collections::HashMap;

#[test]
fn read_to_series() {
    let dir = concat!(env!("CARGO_MANIFEST_DIR"), "/examples/data/a.csv");
    let mut s2 = Series::from(vec![1., 2., 0.4, 5.]);
    s2 += 35.0;
    println!("{:?}", s2);
    let mut s3 = Series::from(vec![1., 2., 3., 43.]);
    let mut s4 = Series::from(["1", "2", "#", "$"]);
    let mut df = DataFrame::new();
    df.add_series(s2.clone(), false).unwrap();
    df.add_series(s3, false).unwrap();
    df.add_series(s4, false).unwrap();
    df.apply::<f64, _>(|f| f * 4 as f64);
    let mut frame = DataFrame::from(vec![vec![1, 2], vec![0, 3], vec![2, 0], vec![1, 1]]);
    println!("{:?}", frame.cov(1.0));
}
fn main() {}
