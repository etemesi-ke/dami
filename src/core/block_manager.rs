//! YOU have REACHED the bowels of the codes.
//! Where ugly stuff happens...
mod manager;

mod stats;

mod ops;
use crate::core::block_manager::manager::Block;
use crate::core::series::Series;
use crate::enums::DataFrameErrors::KeyError;
use crate::enums::{DataFrameErrors, DataTypes};
use crate::prelude::DataFrame;
use ndarray::{Array1, Array2};
use prettytable::evcxr::EvcxrDisplay;
use prettytable::format::consts::FORMAT_CLEAN;
use prettytable::{Cell, Row, Table};
use serde::export::Formatter;
use std::any::Any;
use std::collections::hash_map::RandomState;
use std::collections::HashMap;
use std::fmt;

#[derive(Default)]
pub struct BlockManager {
    // The block allocator
    blocks: HashMap<DataTypes, Box<dyn Any>>,
    // Contains reference to the name of a Series inside a DataType
    values: HashMap<String, DataTypes>,
    // Names of the series to preserve order
    names: Vec<String>,
    // Length of every series in the DataFrame
    len: usize,
    // The DataFrame index
    index: Vec<String>,
}
impl fmt::Debug for BlockManager {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let table = self.real_formatter(true);
        let mut tbl = table.to_string();
        if self.len >= 50 {
            tbl += &format!("\n[{} rows x {} columns]", self.len, self.names.len());
        }
        write!(f, "{}", tbl)
    }
}
impl fmt::Display for BlockManager {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let table = self.real_formatter(false);
        let mut tbl = table.to_string();
        if self.len >= 50 {
            tbl += &format!("\n{}[rows x {} columns]", self.len, self.names.len());
        }
        write!(f, "{}", tbl)
    }
}
impl BlockManager {
    /// Add a new series to the block manager
    pub fn add_series<T>(
        &mut self,
        other: Series<T>,
        preserve_names: bool,
    ) -> Result<(), DataFrameErrors>
    where
        T: Default + 'static + Clone,
    {
        let mut other = other;
        if !self.blocks.is_empty() && other.len() != self.len {
            return Err(DataFrameErrors::DifferentLength(other.len(), self.len));
        }
        if self.names.contains(&other.get_name()) || !preserve_names {
            let name = format!("{}", self.values.len());
            other.set_name(&name);
            // Check now there isn't naming conflicts if still, raise an error
            if self.names.contains(&name) {
                return Err(DataFrameErrors::ColumnNameErrors(name));
            }
        }

        if self.blocks.is_empty() {
            self.len = other.len();
            self.index.extend_from_slice(other.get_index().as_slice());
        }
        self.names.push(other.get_name());
        self.values.insert(other.get_name(), other.get_dtype());
        self.get_appropriate_block(&other.get_dtype(), Box::new(other));
        Ok(())
    }
    /// Apply a function on an Array and return a scalar
    pub fn apply<T, F>(&self, func: F, axis: bool) -> Option<Series<T>>
    where
        T: Clone + Default + 'static,
        F: Clone + Fn(Array1<T>) -> T,
    {
        let keys: Vec<&DataTypes> = self.values.values().collect();
        for i in &keys {
            if let Some(block) = self.blocks.get(i).unwrap().downcast_ref::<Block<T>>() {
                return Some(block.apply(func, axis));
            }
        }
        None
    }
    /// Apply a function to a block without using parallel iterators
    /// On smaller datasets, this function should be preferred over [par_apply](#method.par_apply)
    pub fn apply_map<T, F>(&self, func: F) -> DataFrame
    where
        T: Clone + Default + 'static,
        F: Clone + Fn(T) -> T,
    {
        let mut block_mgr = BlockManager::default();
        let keys: Vec<&DataTypes> = self.values.values().collect();
        for i in &keys {
            if let Some(block) = self.blocks.get(i).unwrap().downcast_ref::<Block<T>>() {
                block_mgr.extend_from_block(block.apply_map(func.clone()));
            }
        }
        DataFrame::from(block_mgr)
    }
    pub fn dtypes(&self) -> HashMap<String, DataTypes, RandomState> {
        self.values.clone()
    }
    /// Apply a function using parallel iterators
    /// This method should be faster than [apply](#method.apply) on large DataSets.
    pub fn par_apply_map<
        T: Clone + Default + 'static + Send + Sync,
        F: Send + Sync + Clone + Fn(T) -> T,
    >(
        &self,
        func: F,
    ) -> DataFrame {
        let mut block_mgr = BlockManager::default();
        let keys: Vec<&DataTypes> = self.values.values().collect();
        for i in &keys {
            if let Some(block) = self.blocks.get(i).unwrap().downcast_ref::<Block<T>>() {
                block_mgr.extend_from_block(block.par_apply(func.clone()));
            }
        }
        DataFrame::from(block_mgr)
    }
    pub fn extend_from_block<T>(&mut self, block: Block<T>)
    where
        T: Clone + Default + 'static,
    {
        for i in block.data {
            self.add_series(i, true).unwrap();
        }
    }
    /// Apply in place using parallel iterators. And the underlying series also uses parallel iterators
    pub fn apply_map_inplace<T, F>(&mut self, func: F)
    where
        T: Clone + Default + 'static + Send + Sync,
        F: Clone + Sync + Send + Fn(T) -> T,
    {
        let keys = self.values.values().collect::<Vec<_>>();
        for i in &keys {
            if let Some(block) = self.blocks.get_mut(i).unwrap().downcast_mut::<Block<T>>() {
                block.apply_inplace(func.clone());
            }
        }
    }

    pub fn assign<T, F>(
        &self,
        key: &str,
        name: &str,
        func: F,
    ) -> Result<BlockManager, DataFrameErrors>
    where
        T: Clone + Default + 'static,
        F: Fn(T) -> T,
    {
        match self.get::<T>(key) {
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
        match self.get::<T>(key) {
            Some(series) => {
                let mut new_series = series.apply(func);
                new_series.set_name(name);
                self.add_series(series, true).unwrap();
                Ok(())
            }
            None => Err(KeyError(format!("key {} does not exist", key))),
        }
    }
    pub fn at<T: Clone + Default + 'static>(&self, loc: (&str, &str)) -> Option<T> {
        if let Some(dtype) = self.values.get(loc.1) {
            if let Some(block) = self.blocks.get(dtype).unwrap().downcast_ref::<Block<T>>() {
                let pos = block.names.iter().position(|n| loc.0 == n).unwrap();
                return Some(block.get_str_value_at(pos, loc.1));
            }
        }
        None
    }
    pub fn combine<T, F>(self, other: &DataFrame, func: F) -> DataFrame
    where
        T: Default + 'static + Clone,
        F: Clone + Fn(T, T) -> T,
    {
        let mut df = DataFrame::new();
        for i in &self.names {
            let me = self.get::<T>(i).unwrap();
            if let Some(series) = other.get::<T>(i) {
                df.add_series(me.combine(&series, func.clone()), true)
                    .unwrap();
            }
        }
        df
    }
    pub fn get_appropriate_block(&mut self, dtype: &DataTypes, other: Box<dyn Any>) {
        match dtype {
            DataTypes::F64 => {
                if let Some(block) = self.blocks.get_mut(dtype) {
                    let series = other.downcast::<Series<f64>>().unwrap();
                    block.downcast_mut::<Block<f64>>().unwrap().push(*series);
                } else {
                    let series = other.downcast::<Series<f64>>().unwrap();
                    let mut block = Block::default();
                    block.push(*series);
                    self.blocks.insert(DataTypes::F64, Box::new(block));
                }
            }
            DataTypes::F32 => {
                if let Some(block) = self.blocks.get_mut(dtype) {
                    let series = other.downcast::<Series<f32>>().unwrap();
                    block.downcast_mut::<Block<f32>>().unwrap().push(*series);
                } else {
                    let series = other.downcast::<Series<f32>>().unwrap();
                    let mut block = Block::default();
                    block.push(*series);
                    self.blocks.insert(DataTypes::F32, Box::new(block));
                }
            }
            DataTypes::BOOL => {
                if let Some(block) = self.blocks.get_mut(dtype) {
                    let series = other.downcast::<Series<bool>>().unwrap();
                    block.downcast_mut::<Block<bool>>().unwrap().push(*series);
                } else {
                    let series = other.downcast::<Series<bool>>().unwrap();
                    let mut block = Block::default();
                    block.push(*series);
                    self.blocks.insert(DataTypes::BOOL, Box::new(block));
                }
            }
            DataTypes::I64 => {
                if let Some(block) = self.blocks.get_mut(dtype) {
                    let series = other.downcast::<Series<i64>>().unwrap();
                    block.downcast_mut::<Block<i64>>().unwrap().push(*series);
                } else {
                    let series = other.downcast::<Series<i64>>().unwrap();
                    let mut block = Block::default();
                    block.push(*series);
                    self.blocks.insert(DataTypes::I64, Box::new(block));
                }
            }
            DataTypes::I32 => {
                if let Some(block) = self.blocks.get_mut(dtype) {
                    let series = other.downcast::<Series<i32>>().unwrap();
                    block.downcast_mut::<Block<i32>>().unwrap().push(*series);
                } else {
                    let series = other.downcast::<Series<i32>>().unwrap();
                    let mut block = Block::default();
                    block.push(*series);
                    self.blocks.insert(DataTypes::I32, Box::new(block));
                }
            }
            DataTypes::STRING => {
                if let Some(block) = self.blocks.get_mut(dtype) {
                    let series = other.downcast::<Series<String>>().unwrap();
                    block.downcast_mut::<Block<String>>().unwrap().push(*series);
                } else {
                    let series = other.downcast::<Series<String>>().unwrap();
                    let mut block = Block::default();
                    block.push(*series);
                    self.blocks.insert(DataTypes::STRING, Box::new(block));
                }
            }
            DataTypes::STR => {
                if let Some(block) = self.blocks.get_mut(dtype) {
                    let series = other.downcast::<Series<&'static str>>().unwrap();
                    block
                        .downcast_mut::<Block<&'static str>>()
                        .unwrap()
                        .push(*series);
                } else {
                    let series = other.downcast::<Series<&'static str>>().unwrap();
                    let mut block = Block::default();
                    block.push(*series);
                    self.blocks.insert(DataTypes::STR, Box::new(block));
                }
            }
            DataTypes::OBJECT => {
                let names = self.names.pop().unwrap();
                self.values.remove(&names);
                eprintln!(
                    "Series with dtype {:?} was not added to the DataFrame",
                    dtype
                )
            }
        }
    }

    fn real_formatter(&self, debug: bool) -> Table {
        let mut table = Table::new();
        if self.len < 10 {
            self.format(0, self.len, true, &mut table);
        } else {
            self.format(0, 5, true, &mut table);
            table.add_row(Row::new(vec![Cell::new("...."); self.values.len() + 1]));
            let last_five = self.len - 5;
            self.format(last_five, self.len, false, &mut table);
        }
        if debug {
            table.add_empty_row();
            let mut row_dtypes = vec![Cell::new("types")];
            for i in &self.names {
                row_dtypes.push(Cell::new(&format!("{:?}", self.values.get(i).unwrap())));
            }
            table.add_row(Row::new(row_dtypes));
        }
        table
    }
    #[allow(clippy::similar_names, unused_assignments)]
    fn format(&self, start: usize, end: usize, add_titles: bool, table: &mut Table) {
        table.set_format(*FORMAT_CLEAN);
        if add_titles {
            let mut title = vec![Cell::new(" ")];
            for i in &self.names {
                title.push(Cell::new(i))
            }
            table.set_titles(Row::new(title));
        }
        for i in start..end {
            let mut row = vec![];
            let mut f64_counter = 0;
            let mut f32_counter = 0;
            let mut i64_counter = 0;
            let mut i32_counter = 0;
            let mut string_counter = 0;
            let mut str_counter = 0;
            let mut bool_counter = 0;

            for j in &self.names {
                // Counters
                // Because some things need order

                let value = self.values.get(j).unwrap();
                let dtype = self.blocks.get(value).unwrap();
                // Okay.
                // Time for magic
                // Dereference box any to a block of different types and get the value at X[i]
                match value {
                    DataTypes::F64 => {
                        let block = dtype.downcast_ref::<Block<f64>>().unwrap();
                        row.push(Cell::new(&format!(
                            "{:0<3.3}",
                            block.get_value_at(f64_counter, i)
                        )));
                        f64_counter += 1;
                    }
                    DataTypes::F32 => {
                        let block = dtype.downcast_ref::<Block<f32>>().unwrap();
                        row.push(Cell::new(&format!(
                            "{:0<3.3}",
                            block.get_value_at(f32_counter, i)
                        )));
                        f32_counter += 1;
                    }
                    DataTypes::I64 => {
                        let block = dtype.downcast_ref::<Block<i64>>().unwrap();
                        row.push(Cell::new(&format!(
                            "{}",
                            block.get_value_at(i64_counter, i)
                        )));
                        i64_counter += 1;
                    }
                    DataTypes::I32 => {
                        let block = dtype.downcast_ref::<Block<i32>>().unwrap();
                        row.push(Cell::new(&format!(
                            "{:?}",
                            block.get_value_at(i32_counter, i)
                        )));
                        i32_counter += 1;
                    }
                    DataTypes::STRING => {
                        let block = dtype.downcast_ref::<Block<String>>().unwrap();
                        let value_at = block.get_value_at(string_counter, i);
                        if value_at.len() < 30 {
                            row.push(Cell::new(value_at.as_str()));
                        } else {
                            row.push(Cell::new(&(value_at[0..30].to_string() + "...")));
                        }

                        string_counter += 1;
                    }
                    DataTypes::STR => {
                        let block = dtype.downcast_ref::<Block<&'static str>>().unwrap();
                        row.push(Cell::new(block.get_value_at(str_counter, i)));
                        str_counter += 1;
                    }
                    DataTypes::BOOL => {
                        let block = dtype.downcast_ref::<Block<bool>>().unwrap();
                        row.push(Cell::new(&format!(
                            "{}",
                            block.get_value_at(bool_counter, i)
                        )));
                        bool_counter += 1;
                    }
                    _ => continue,
                }
            }
            row.insert(0, Cell::new(&self.index[i]));
            table.add_row(Row::new(row));
        }
    }
    /// Get the series at the col X
    pub fn get<T>(&self, col: &str) -> Option<Series<T>>
    where
        T: Clone + Default + 'static,
    {
        for blocks in self.blocks.values() {
            if let Some(block) = blocks.downcast_ref::<Block<T>>() {
                return Some(block.get_series_at_name(col));
            };
        }
        None
    }
    fn reindex(&mut self, new_names: Vec<String>) {
        self.names = new_names;
    }
    pub fn head_evcxr(&self, n: usize) {
        let mut table = Table::new();
        self.format(0, n, true, &mut table);
        table.evcxr_display();
    }
    #[allow(clippy::needless_pass_by_value)]
    pub fn mask<T, F>(&mut self, value: T, cond: F)
    where
        T: Default + Clone + 'static,
        F: Clone + Fn(T) -> bool,
    {
        for blocks in &mut self.blocks {
            if let Some(block) = blocks.1.downcast_mut::<Block<T>>() {
                block.mask(value.clone(), cond.clone());
            };
        }
    }
    pub fn tail_evcxr(&self, n: usize) {
        let mut table = Table::new();
        let start = self.len - n;
        self.format(start, self.len, true, &mut table);
        table.evcxr_display();
    }
    pub fn to_ndarray<T>(&self) -> Option<Array2<T>>
    where
        T: Clone + Default + 'static,
    {
        for blocks in self.blocks.values() {
            if let Some(block) = blocks.downcast_ref::<Block<T>>() {
                return Some(block.to_ndarray());
            };
        }
        None
    }
    pub fn head(&self, n: usize) {
        let mut table = Table::new();
        self.format(0, n, true, &mut table);
        println!("{}", table.to_string());
    }
    pub fn tail(&self, n: usize) {
        let mut table = Table::new();
        let start = self.len - n;
        self.format(start, self.len, true, &mut table);
        println!("{}", table.to_string());
    }
    pub fn transform<T, P, F>(&self, func: F, axis: bool) -> Option<DataFrame>
    where
        T: Default + 'static + Clone + Send + Sync,
        P: Default + 'static + Clone + Send + Sync,
        F: Clone + Fn(Array1<T>) -> Array1<P> + Sync + Send,
    {
        for blocks in self.blocks.values() {
            if let Some(block) = blocks.downcast_ref::<Block<T>>() {
                let mut new_block = BlockManager::default();
                new_block.extend_from_block(block.transform(func, axis));
                return Some(DataFrame::from(new_block));
            };
        }
        None
    }
}

impl Clone for BlockManager {
    fn clone(&self) -> Self {
        let mut block_mgr = BlockManager::default();
        for i in &self.blocks {
            match i.0 {
                DataTypes::F64 => {
                    let block = i.1.downcast_ref::<Block<f64>>().unwrap();
                    block_mgr.extend_from_block(block.clone())
                }
                DataTypes::F32 => {
                    let block = i.1.downcast_ref::<Block<f32>>().unwrap();
                    block_mgr.extend_from_block(block.clone())
                }
                DataTypes::I64 => {
                    let block = i.1.downcast_ref::<Block<i64>>().unwrap();
                    block_mgr.extend_from_block(block.clone())
                }
                DataTypes::I32 => {
                    let block = i.1.downcast_ref::<Block<i32>>().unwrap();
                    block_mgr.extend_from_block(block.clone())
                }
                DataTypes::BOOL => {
                    let block = i.1.downcast_ref::<Block<bool>>().unwrap();
                    block_mgr.extend_from_block(block.clone())
                }
                DataTypes::STR => {
                    let block = i.1.downcast_ref::<Block<&'static str>>().unwrap();
                    block_mgr.extend_from_block(block.clone())
                }
                DataTypes::STRING => {
                    let block = i.1.downcast_ref::<Block<String>>().unwrap();
                    block_mgr.extend_from_block(block.clone())
                }
                _ => continue,
            }
        }
        block_mgr.reindex(self.names.clone());
        block_mgr
    }
}
