#![cfg(feature = "stats")]
use crate::core::block_manager::manager::Block;
use crate::core::block_manager::BlockManager;
use crate::core::dataframe::DataFrame;
use crate::core::series::Series;
use crate::enums::DataTypes;
use ndarray::Array2;
use ndarray_stats::CorrelationExt;
use plotly::Plot;
use std::env::temp_dir;
use std::fs;
use std::time::{SystemTime, UNIX_EPOCH};

macro_rules! generate_methods {
    ($func:ident) => {
        impl BlockManager {
            pub fn $func(&self) -> Series<f64> {
                let mut series = Series::default();
                let mut names = Vec::new();
                for i in self.blocks.iter() {
                    match i.0 {
                        DataTypes::F64 => {
                            let block = i.1.downcast_ref::<Block<f64>>().unwrap();
                            names.extend_from_slice(block.names.as_slice());
                            series.append(block.$func(), true, false);
                        }
                        DataTypes::F32 => {
                            let block = i.1.downcast_ref::<Block<f32>>().unwrap();
                            names.extend_from_slice(block.names.as_slice());
                            series.append(block.$func().as_type(), true, false);
                        }
                        DataTypes::I32 => {
                            let block = i.1.downcast_ref::<Block<i32>>().unwrap();
                            names.extend_from_slice(block.names.as_slice());
                            series.append(block.clone().as_type::<f64>().$func(), true, false);
                        }

                        _ => {}
                    }
                }
                series
            }
        }
    };
}
generate_methods!(max);
generate_methods!(min);
generate_methods!(mean);
generate_methods!(kurtosis);
generate_methods!(skewness);
generate_methods!(stdev);
generate_methods!(variance);

macro_rules! generate_moments {
    ($func:ident) => {
        impl BlockManager {
            pub fn $func(&self, order: u16) -> Series<f64> {
                let mut series = Series::default();
                let mut names = Vec::new();
                for i in self.blocks.iter() {
                    match i.0 {
                        DataTypes::F64 => {
                            let block = i.1.downcast_ref::<Block<f64>>().unwrap();
                            names.extend_from_slice(block.names.as_slice());
                            series.append(block.$func(order), true, false);
                        }
                        DataTypes::F32 => {
                            let block = i.1.downcast_ref::<Block<f32>>().unwrap();
                            names.extend_from_slice(block.names.as_slice());
                            series.append(block.$func(order).as_type(), true, false);
                        }
                        DataTypes::I32 => {
                            let block = i.1.downcast_ref::<Block<i32>>().unwrap();
                            names.extend_from_slice(block.names.as_slice());
                            series.append(block.clone().as_type::<f64>().$func(order), true, false);
                        }

                        _ => {}
                    }
                }
                series
            }
        }
    };
}
generate_moments!(central_moment);

impl BlockManager {
    pub fn corr(&self) -> DataFrame {
        let mut frames = Vec::new();
        let mut amt = 0;

        // To maintain order. we don't iterate over the block
        for i in &self.names {
            let dtype = self.values.get(i).unwrap();
            match dtype {
                DataTypes::F64 => {
                    let block = self
                        .blocks
                        .get(dtype)
                        .unwrap()
                        .downcast_ref::<Block<f64>>()
                        .unwrap();
                    frames.extend_from_slice(block.get_series_at_name(i).to_vec().as_slice());
                    amt += 1
                }
                DataTypes::F32 => {
                    let block = self
                        .blocks
                        .get(dtype)
                        .unwrap()
                        .downcast_ref::<Block<f32>>()
                        .unwrap();
                    frames.extend_from_slice(
                        block.get_series_at_name(i).as_type().to_vec().as_slice(),
                    );
                    amt += 1
                }
                DataTypes::I32 => {
                    let block = self
                        .blocks
                        .get(dtype)
                        .unwrap()
                        .downcast_ref::<Block<i32>>()
                        .unwrap();
                    frames.extend_from_slice(
                        block.get_series_at_name(i).as_type().to_vec().as_slice(),
                    );
                    amt += 1
                }

                _ => continue,
            }
        }
        let arr = Array2::from_shape_vec((amt, self.len), frames).unwrap();
        DataFrame::from(arr.pearson_correlation().unwrap())
    }
    pub fn cov(&self, min_periods: f64) -> DataFrame {
        let mut frames = Vec::new();
        let mut amt = 0;

        // To maintain order. we don't iterate over the block
        for i in &self.names {
            let dtype = self.values.get(i).unwrap();
            match dtype {
                DataTypes::F64 => {
                    let block = self
                        .blocks
                        .get(dtype)
                        .unwrap()
                        .downcast_ref::<Block<f64>>()
                        .unwrap();
                    frames.extend_from_slice(block.get_series_at_name(i).to_vec().as_slice());
                    amt += 1
                }
                DataTypes::F32 => {
                    let block = self
                        .blocks
                        .get(dtype)
                        .unwrap()
                        .downcast_ref::<Block<f32>>()
                        .unwrap();
                    frames.extend_from_slice(
                        block.get_series_at_name(i).as_type().to_vec().as_slice(),
                    );
                    amt += 1
                }
                DataTypes::I32 => {
                    let block = self
                        .blocks
                        .get(dtype)
                        .unwrap()
                        .downcast_ref::<Block<i32>>()
                        .unwrap();
                    frames.extend_from_slice(
                        block.get_series_at_name(i).as_type().to_vec().as_slice(),
                    );
                    amt += 1
                }

                _ => continue,
            }
        }
        let arr = Array2::from_shape_vec((amt, self.len), frames).unwrap();
        DataFrame::from(arr.cov(min_periods).unwrap())
    }
}

impl BlockManager {
    pub fn plot(&self, kind: &str) {
        let mut me = Plot::new();
        match kind {
            "bar" => self.plot_bar(&mut me),
            "line" => self.plot_lines(&mut me),
            "hist" => self.plot_hist(&mut me),
            "h_hist" => self.plot_h_hist(&mut me),
            "scatter" => self.plot_marks(&mut me),
            "box" => self.plot_box(&mut me),
            _ => {
                eprintln!("Method {} not known,defaulting to line plot", kind);
                self.plot_lines(&mut me);
            }
        };
        me.show();
    }
    pub fn plot_evcxr(&self, kind: &str) {
        let mut me = Plot::new();
        let mut tempo_dir = temp_dir();
        let time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        tempo_dir.push(format!("dami{}.html", time));
        match kind {
            "bar" => self.plot_bar(&mut me),
            "line" => self.plot_lines(&mut me),
            "hist" => self.plot_hist(&mut me),
            "h_hist" => self.plot_h_hist(&mut me),
            "scatter" => self.plot_marks(&mut me),
            "box" => self.plot_box(&mut me),
            _ => {
                eprintln!("Method {} not known,defaulting to line plot", kind);
                self.plot_lines(&mut me);
            }
        };
        me.to_html(tempo_dir.clone());
        let plot_data = fs::read_to_string(tempo_dir).unwrap();

        println!(
            "EVCXR_BEGIN_CONTENT text/html\n{}\nEVCXR_END_CONTENT",
            format!(
                "<div>{}</div>",
                &plot_data.replace("plotly-html-element", &format!("dami_{}", time))
            )
        )
    }
    pub fn plot_bar(&self, plot: &mut Plot) {
        for (dtype, block) in &self.blocks {
            match dtype {
                DataTypes::F64 => {
                    block
                        .downcast_ref::<Block<f64>>()
                        .unwrap()
                        .plot_bar(plot, self.index.clone());
                }
                DataTypes::F32 => {
                    block
                        .downcast_ref::<Block<f32>>()
                        .unwrap()
                        .plot_bar(plot, self.index.clone());
                }
                DataTypes::I64 => {
                    block
                        .downcast_ref::<Block<i64>>()
                        .unwrap()
                        .plot_bar(plot, self.index.clone());
                }
                DataTypes::I32 => {
                    block
                        .downcast_ref::<Block<i32>>()
                        .unwrap()
                        .plot_bar(plot, self.index.clone());
                }
                _ => continue,
            }
        }
    }
    pub fn plot_box(&self, plot: &mut Plot) {
        for (dtype, block) in &self.blocks {
            match dtype {
                DataTypes::F64 => {
                    block.downcast_ref::<Block<f64>>().unwrap().plot_box(plot);
                }
                DataTypes::F32 => {
                    block.downcast_ref::<Block<f32>>().unwrap().plot_box(plot);
                }
                DataTypes::I64 => {
                    block.downcast_ref::<Block<i64>>().unwrap().plot_box(plot);
                }
                DataTypes::I32 => {
                    block.downcast_ref::<Block<i32>>().unwrap().plot_box(plot);
                }
                _ => continue,
            }
        }
    }
    pub fn plot_lines(&self, plot: &mut Plot) {
        for (dtype, block) in &self.blocks {
            match dtype {
                DataTypes::F64 => {
                    block
                        .downcast_ref::<Block<f64>>()
                        .unwrap()
                        .plot_line(plot, self.index.clone());
                }
                DataTypes::F32 => {
                    block
                        .downcast_ref::<Block<f32>>()
                        .unwrap()
                        .plot_line(plot, self.index.clone());
                }
                DataTypes::I64 => {
                    block
                        .downcast_ref::<Block<i64>>()
                        .unwrap()
                        .plot_line(plot, self.index.clone());
                }
                DataTypes::I32 => {
                    block
                        .downcast_ref::<Block<i32>>()
                        .unwrap()
                        .plot_line(plot, self.index.clone());
                }
                _ => continue,
            }
        }
    }
    pub fn plot_hist(&self, plot: &mut Plot) {
        for (dtype, block) in &self.blocks {
            match dtype {
                DataTypes::F64 => {
                    block.downcast_ref::<Block<f64>>().unwrap().plot_hist(plot);
                }
                DataTypes::F32 => {
                    block.downcast_ref::<Block<f32>>().unwrap().plot_hist(plot);
                }
                DataTypes::I64 => {
                    block.downcast_ref::<Block<i64>>().unwrap().plot_hist(plot);
                }
                DataTypes::I32 => {
                    block.downcast_ref::<Block<i32>>().unwrap().plot_hist(plot);
                }
                _ => continue,
            }
        }
    }
    pub fn plot_h_hist(&self, plot: &mut Plot) {
        for (dtype, block) in &self.blocks {
            match dtype {
                DataTypes::F64 => {
                    block
                        .downcast_ref::<Block<f64>>()
                        .unwrap()
                        .plot_h_hist(plot);
                }
                DataTypes::F32 => {
                    block
                        .downcast_ref::<Block<f32>>()
                        .unwrap()
                        .plot_h_hist(plot);
                }
                DataTypes::I64 => {
                    block
                        .downcast_ref::<Block<i64>>()
                        .unwrap()
                        .plot_h_hist(plot);
                }
                DataTypes::I32 => {
                    block
                        .downcast_ref::<Block<i32>>()
                        .unwrap()
                        .plot_h_hist(plot);
                }
                _ => continue,
            }
        }
    }
    pub fn plot_marks(&self, plot: &mut Plot) {
        for (dtype, block) in &self.blocks {
            match dtype {
                DataTypes::F64 => {
                    block
                        .downcast_ref::<Block<f64>>()
                        .unwrap()
                        .plot_dots(plot, self.index.clone());
                }
                DataTypes::F32 => {
                    block
                        .downcast_ref::<Block<f32>>()
                        .unwrap()
                        .plot_dots(plot, self.index.clone());
                }
                DataTypes::I64 => {
                    block
                        .downcast_ref::<Block<i64>>()
                        .unwrap()
                        .plot_dots(plot, self.index.clone());
                }
                DataTypes::I32 => {
                    block
                        .downcast_ref::<Block<i32>>()
                        .unwrap()
                        .plot_dots(plot, self.index.clone());
                }
                _ => continue,
            }
        }
    }
}
