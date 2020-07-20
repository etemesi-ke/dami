//! Operator ops for BlockManager
#![allow(unused_variables)]
use crate::core::block_manager::manager::Block;
use crate::core::block_manager::BlockManager;
use crate::enums::DataTypes;
use std::ops::{Add, Div, Mul, Sub};
macro_rules! generate_df_ops {
    ($trait:ident,$func:ident) => {
        #[allow(clippy::suspicious_arithmetic_impl)]
        impl $trait for BlockManager {
            type Output = BlockManager;

            fn $func(self, rhs: Self) -> Self::Output {
                let mut block = BlockManager::default();
                let mut f64_counter = 0;
                let mut f32_counter = 0;
                let mut i32_counter = 0;
                let mut i64_counter = 0;
                for i in &self.names {
                    let dtype = self.values.get(i).unwrap();
                    match dtype {
                        DataTypes::F64 => {
                            let me = self
                                .blocks
                                .get(dtype)
                                .unwrap()
                                .downcast_ref::<Block<f64>>()
                                .unwrap()
                                .get(f64_counter);
                            if let Some(other) = rhs.get(i) {
                                block.add_series(me.$func(other), true).unwrap();
                                f64_counter += 1
                            }
                        }
                        DataTypes::F32 => {
                            let me = self
                                .blocks
                                .get(dtype)
                                .unwrap()
                                .downcast_ref::<Block<f32>>()
                                .unwrap()
                                .get(f64_counter);
                            if let Some(other) = rhs.get(i) {
                                block.add_series(me.$func(other), true).unwrap();
                                f32_counter += 1
                            }
                        }

                        DataTypes::I64 => {
                            let me = self
                                .blocks
                                .get(dtype)
                                .unwrap()
                                .downcast_ref::<Block<i64>>()
                                .unwrap()
                                .get(f64_counter);
                            if let Some(other) = rhs.get(i) {
                                block.add_series(me.$func(other), true).unwrap();
                                i64_counter += 1
                            }
                        }

                        DataTypes::I32 => {
                            let me = self
                                .blocks
                                .get(dtype)
                                .unwrap()
                                .downcast_ref::<Block<i32>>()
                                .unwrap()
                                .get(f64_counter);
                            if let Some(other) = rhs.get(i) {
                                block.add_series(me.$func(other), true).unwrap();
                                i32_counter += 1
                            }
                        }
                        _ => {}
                    }
                }
                block
            }
        }
    };
}
generate_df_ops!(Mul, mul);
generate_df_ops!(Sub, sub);
generate_df_ops!(Div, div);
generate_df_ops!(Add, add);
