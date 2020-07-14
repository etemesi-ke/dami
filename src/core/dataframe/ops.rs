use crate::core::dataframe::DataFrame;
use crate::enums::DataTypes;
use std::ops::Add;
use std::ops::Div;
use std::ops::Mul;
use std::ops::Sub;
macro_rules! impl_ops {
    ($trait:ident,$val:ident) => {
        impl $trait for DataFrame {
            type Output = DataFrame;

            fn $val(self, rhs: Self) -> Self::Output {
                let mut df = DataFrame::new();
                for i in &self.get_order() {
                    let dtype = self.get_dtype_at_key(i).unwrap();
                    match dtype {
                        DataTypes::F64 => {
                            let me = self.get_series::<f64>(i).unwrap();
                            if let Some(series) = rhs.get_series::<f64>(i) {
                                df.add_series(me.$val(series), true).unwrap();
                                continue;
                            };
                            df.add_series(me.clone(), true).unwrap();
                        }
                        DataTypes::F32 => {
                            let me = self.get_series::<f32>(i).unwrap();
                            if let Some(series) = rhs.get_series::<f32>(i) {
                                df.add_series(me.$val(series), true).unwrap();
                                continue;
                            };
                            df.add_series(me.clone(), true).unwrap();
                        }
                        DataTypes::I64 => {
                            let me = self.get_series::<i64>(i).unwrap();
                            if let Some(series) = rhs.get_series::<i64>(i) {
                                df.add_series(me.$val(series), true).unwrap();
                                continue;
                            };
                            df.add_series(me.clone(), true).unwrap();
                        }
                        DataTypes::I32 => {
                            let me = self.get_series::<i32>(i).unwrap();
                            if let Some(series) = rhs.get_series::<i32>(i) {
                                df.add_series(me.$val(series), true).unwrap();
                                continue;
                            };
                            df.add_series(me.clone(), true).unwrap();
                        }
                        DataTypes::I128 => {
                            let me = self.get_series::<i128>(i).unwrap();
                            if let Some(series) = rhs.get_series::<i128>(i) {
                                df.add_series(me.$val(series), true).unwrap();
                                continue;
                            };
                            df.add_series(me.clone(), true).unwrap();
                        }
                        _ => continue,
                    }
                }
                df
            }
        }
    };
}

impl_ops!(Sub, sub);
impl_ops!(Add, add);
impl_ops!(Div, div);
impl_ops!(Mul, mul);
