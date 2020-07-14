use crate::enums::DataTypes;
use crate::prelude::DataFrame;
use plotly::common::Mode;
use plotly::{BoxPlot, Plot};
use std::env::temp_dir;
use std::fs::File;
use std::io::{BufReader, Read};
use std::time::{SystemTime, UNIX_EPOCH};

impl DataFrame {
    /// Requires feature
    /// * `stats`
    ///
    /// Plot the series in the DataFrame
    ///
    /// The types that will be plotted include f64,f32,i128,i64 and i32 type Series
    ///
    /// # Arguments
    /// * `kind`: The type of plot to draw.
    /// > Supported plots are
    /// > > * "hist"-> histogram
    ///
    /// > > * "h_hist" -> horizontal histogram
    ///
    /// > > * "bar"->bar graph,
    ///
    /// > > * "marks"|"points" -> scatter graph
    ///
    /// > > * "line" -> line graph
    ///
    /// > > * "box" - > box graph,
    ///
    /// If the string passed to `kind` argument doesn't match the above values. A line plot is drown
    ///
    /// # Note
    /// This is backed by [plotly.js] using the [plotly] crate, so the resulting graph is opened in
    /// your default browser.
    ///
    /// [plotly.js]: https://plot.ly/javascript/
    /// [plotly]: https://docs.rs/plotly
    pub fn plot(&self, kind: &str) {
        let mut plot = Plot::new();
        match kind {
            "bar" => self.plot_bar(&mut plot),
            "line" => self.plot_line(&mut plot),
            "marks" | "points" => self.plot_marks(&mut plot),
            "h_hist" => self.plot_horizontal_hist(&mut plot),
            "hist" => self.plot_hist(&mut plot),
            "box" => self.plot_box(&mut plot),
            _ => {
                eprintln!("Method {} not known defaulting to line graph", kind);
                self.plot_line(&mut plot)
            }
        }
        plot.show()
    }
    /// Plot a bar plot
    fn plot_bar(&self, plot: &mut Plot) {
        for i in &self.get_order() {
            let dtype = self.get_dtype_at_key(i).unwrap();
            match dtype {
                DataTypes::F64 => {
                    let series = self.get_series::<f64>(i).unwrap();
                    plot.add_trace(series.plot_bar(i.as_str()));
                }
                DataTypes::F32 => {
                    let series = self.get_series::<f32>(i).unwrap();
                    plot.add_trace(series.plot_bar(i.as_str()));
                }
                DataTypes::I64 => {
                    let series = self.get_series::<i64>(i).unwrap();
                    plot.add_trace(series.plot_bar(i.as_str()));
                }
                DataTypes::I128 => {
                    let series = self.get_series::<i128>(i).unwrap();
                    plot.add_trace(series.plot_bar(i.as_str()));
                }
                DataTypes::I32 => {
                    let series = self.get_series::<i32>(i).unwrap();
                    plot.add_trace(series.plot_bar(i.as_str()));
                }

                _ => continue,
            }
        }
    }
    /// Plot a line plot
    fn plot_line(&self, plot: &mut Plot) {
        for i in &self.get_order() {
            let dtype = self.get_dtype_at_key(i).unwrap();
            match dtype {
                DataTypes::F64 => {
                    let series = self.get_series::<f64>(i).unwrap();
                    plot.add_trace(series.plot_line(Mode::Lines, i.as_str()));
                }
                DataTypes::F32 => {
                    let series = self.get_series::<f32>(i).unwrap();
                    plot.add_trace(series.plot_line(Mode::Lines, i.as_str()));
                }
                DataTypes::I64 => {
                    let series = self.get_series::<i64>(i).unwrap();
                    plot.add_trace(series.plot_line(Mode::Lines, i.as_str()));
                }
                DataTypes::I128 => {
                    let series = self.get_series::<i128>(i).unwrap();
                    plot.add_trace(series.plot_line(Mode::Lines, i.as_str()));
                }
                DataTypes::I32 => {
                    let series = self.get_series::<i32>(i).unwrap();
                    plot.add_trace(series.plot_line(Mode::Lines, i.as_str()));
                }

                _ => continue,
            }
        }
    }
    /// Plot a scatter plot
    fn plot_marks(&self, plot: &mut Plot) {
        for i in &self.get_order() {
            let dtype = self.get_dtype_at_key(i).unwrap();
            match dtype {
                DataTypes::F64 => {
                    let series = self.get_series::<f64>(i).unwrap();
                    plot.add_trace(series.plot_line(Mode::Markers, i.as_str()));
                }
                DataTypes::F32 => {
                    let series = self.get_series::<f32>(i).unwrap();
                    plot.add_trace(series.plot_line(Mode::Markers, i.as_str()));
                }
                DataTypes::I64 => {
                    let series = self.get_series::<i64>(i).unwrap();
                    plot.add_trace(series.plot_line(Mode::Markers, i.as_str()));
                }
                DataTypes::I128 => {
                    let series = self.get_series::<i128>(i).unwrap();
                    plot.add_trace(series.plot_line(Mode::Markers, i.as_str()));
                }
                DataTypes::I32 => {
                    let series = self.get_series::<i32>(i).unwrap();
                    plot.add_trace(series.plot_line(Mode::Markers, i.as_str()));
                }

                _ => continue,
            }
        }
    }
    fn plot_hist(&self, plot: &mut Plot) {
        for i in &self.get_order() {
            let dtype = self.get_dtype_at_key(i).unwrap();
            match dtype {
                DataTypes::F64 => {
                    let series = self.get_series::<f64>(i).unwrap();
                    plot.add_trace(series.plot_histogram(i.as_str()));
                }
                DataTypes::F32 => {
                    let series = self.get_series::<f32>(i).unwrap();
                    plot.add_trace(series.plot_histogram(i.as_str()));
                }
                DataTypes::I64 => {
                    let series = self.get_series::<i64>(i).unwrap();
                    plot.add_trace(series.plot_histogram(i.as_str()));
                }
                DataTypes::I128 => {
                    let series = self.get_series::<i128>(i).unwrap();
                    plot.add_trace(series.plot_histogram(i.as_str()));
                }
                DataTypes::I32 => {
                    let series = self.get_series::<i32>(i).unwrap();
                    plot.add_trace(series.plot_histogram(i.as_str()));
                }

                _ => continue,
            }
        }
    }
    fn plot_horizontal_hist(&self, plot: &mut Plot) {
        for i in &self.get_order() {
            let dtype = self.get_dtype_at_key(i).unwrap();
            match dtype {
                DataTypes::F64 => {
                    let series = self.get_series::<f64>(i).unwrap();
                    plot.add_trace(series.plot_histogram(i.as_str()));
                }
                DataTypes::F32 => {
                    let series = self.get_series::<f32>(i).unwrap();
                    plot.add_trace(series.plot_histogram(i.as_str()));
                }
                DataTypes::I64 => {
                    let series = self.get_series::<i64>(i).unwrap();
                    plot.add_trace(series.plot_histogram(i.as_str()));
                }
                DataTypes::I128 => {
                    let series = self.get_series::<i128>(i).unwrap();
                    plot.add_trace(series.plot_histogram(i.as_str()));
                }
                DataTypes::I32 => {
                    let series = self.get_series::<i32>(i).unwrap();
                    plot.add_trace(series.plot_histogram(i.as_str()));
                }

                _ => continue,
            }
        }
    }
    fn plot_box(&self, plot: &mut Plot) {
        for i in &self.get_order() {
            let dtype = self.get_dtype_at_key(i).unwrap();
            match dtype {
                DataTypes::F64 => {
                    let series = self.get_series::<f64>(i).unwrap();
                    let box_plot = BoxPlot::new(series.to_vec()).name(&series.get_name());
                    plot.add_trace(box_plot);
                }
                DataTypes::F32 => {
                    let series = self.get_series::<f32>(i).unwrap().as_type::<f64>();
                    let box_plot = BoxPlot::new(series.to_vec()).name(&series.get_name());
                    plot.add_trace(box_plot);
                }
                DataTypes::I32 => {
                    let series = self.get_series::<i32>(i).unwrap().as_type::<f64>();
                    let box_plot = BoxPlot::new(series.to_vec()).name(&series.get_name());
                    plot.add_trace(box_plot);
                }
                _ => continue,
            }
        }
    }
    /// Plot a graph into a jupyter notebook using rust repl environment which can be downloaded and installed
    /// from  [here](https://github.com/google/evcxr)
    /// # READ THIS!
    /// You should have already `set up a conda environment` this [website](https://shahinrostami.com/posts/programming/rust-notebooks/setup-anaconda-jupyter-and-rust/)
    /// has some nice instructions on how to do this.
    /// > This embeds a whole HTML file ( about 10kb) to the notebook. And this grows linearly for every graph rendered.
    ///
    /// To make it work run
    /// ```bash
    /// $ jupyter labextension install jupyterlab-plotly@4.8.2
    /// ```
    /// to install plotly extension in your `conda environment`
    /// # Tip
    /// This is a large library and pulls in a lot of dependencies. It takes long to compile
    /// so it is recommended that you use your jupyter environment configured to use sccache.
    /// Instructions for setting that up can be found [here](https://github.com/google/evcxr/tree/master/evcxr_jupyter)
    pub fn plot_evcxr(&self, kind: &str) {
        let mut plot = Plot::new();
        let mut temp_dir = temp_dir();
        let time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        temp_dir.push(format!("dami{}.html", time));
        match kind {
            "bar" => self.plot_bar(&mut plot),
            "line" => self.plot_line(&mut plot),
            "scatter" => self.plot_marks(&mut plot),
            "h_hist" => self.plot_horizontal_hist(&mut plot),
            "hist" => self.plot_hist(&mut plot),
            "box" => self.plot_box(&mut plot),
            _ => {
                eprintln!("Method {} not known defaulting to line graph", kind);
                self.plot_line(&mut plot)
            }
        }
        plot.to_html(temp_dir.clone());
        let mut fs = BufReader::new(File::open(temp_dir).unwrap());
        let mut contents = String::new();
        fs.read_to_string(&mut contents).unwrap();
        println!(
            "EVCXR_BEGIN_CONTENT text/html\n{}\nEVCXR_END_CONTENT",
            format!(
                "<div>{}</div>",
                &contents.replace("plotly-html-element", &format!("dami_{}", time))
            )
        )
    }
}
