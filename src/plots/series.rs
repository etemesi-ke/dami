use crate::prelude::Series;
use num_traits::Num;
use serde::Serialize;

use plotly::common::Mode;
use plotly::{Bar, Histogram, Plot, Scatter};
use std::env::temp_dir;
use std::fs;
use std::time::{SystemTime, UNIX_EPOCH};

impl<T: Clone + Default + Num + Serialize + 'static> Series<T> {
    /// # Requires Feature
    ///  > * `stats`
    ///
    /// Plot data in the series
    ///
    /// This function provides higher order methods for plotting data in a series
    /// The series should contain numerical data (usize, floats,ints) ie anything implementing the Num trait
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
    /// > > * "scatter" -> scatter graph
    ///
    /// > > * "line" -> line graph
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
            "bar" => plot.add_trace(self.plot_bar(&self.get_name())),
            "line" => plot.add_trace(self.plot_line(Mode::Lines, &self.get_name())),
            "hist" => plot.add_trace(self.plot_histogram(&self.get_name())),
            "h_hist" => plot.add_trace(self.plot_horizontal_histogram(&self.get_name())),
            "scatter" => plot.add_trace(self.plot_line(Mode::Markers, &self.get_name())),
            _ => {
                eprintln!("Method {} not known,defaulting to line plot", kind);
                plot.add_trace(self.plot_line(Mode::Lines, self.get_name().as_str()));
            }
        };
        plot.show();
    }
    #[doc(hidden)]
    pub fn plot_bar(&self, name: &str) -> Box<Bar<String, T>> {
        Bar::new(self.get_index(), self.to_vec()).name(name)
    }
    #[doc(hidden)]
    pub fn plot_line(&self, mode: Mode, name: &str) -> Box<Scatter<String, T>> {
        Scatter::new(self.get_index(), self.to_vec())
            .name(name)
            .mode(mode)
    }
    #[doc(hidden)]
    pub fn plot_histogram(&self, name: &str) -> Box<Histogram<T>> {
        Histogram::new(self.to_vec()).name(name)
    }
    #[doc(hidden)]
    pub fn plot_horizontal_histogram(&self, name: &str) -> Box<Histogram<T>> {
        Histogram::new_horizontal(self.to_vec()).name(name)
    }
    /// Plot a graph into a jupyter notebook using rust repl environment which can be downloaded and installed
    /// from  [here](https://github.com/google/evcxr)
    /// # READ THIS!
    /// You should have already `set up a conda environment` this [website](https://shahinrostami.com/posts/programming/rust-notebooks/setup-anaconda-jupyter-and-rust/)
    /// has some nice instructions on how to do this.
    ///  Due to lack of an implementation by the default plotly library. This is a drop in replacement until
    /// (hopefully) we can get a better implementation by the maintainer (I didn't file an Issue if you wondering)
    ///
    /// > This embeds a whole HTML file ( about 3 mb) to the notebook. And this grows linearly for every graph rendered.
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

        let mut tempo_dir = temp_dir();
        let time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        tempo_dir.push(format!("dami{}.html", time));
        match kind {
            "bar" => plot.add_trace(self.plot_bar(self.get_name().as_str())),
            "line" => plot.add_trace(self.plot_line(Mode::Lines, self.get_name().as_str())),
            "hist" => plot.add_trace(self.plot_histogram(self.get_name().as_str())),
            "h_hist" => plot.add_trace(self.plot_horizontal_histogram(self.get_name().as_str())),
            "points" | "marks" => {
                plot.add_trace(self.plot_line(Mode::Markers, self.get_name().as_str()))
            }
            _ => {
                eprintln!("Method {} not known,defaulting to line plot", kind);
                plot.add_trace(self.plot_line(Mode::Lines, self.get_name().as_str()));
            }
        };
        plot.to_html(tempo_dir.clone());
        let plot_data = fs::read_to_string(tempo_dir).unwrap();

        println!(
            "EVCXR_BEGIN_CONTENT text/html\n{}\nEVCXR_END_CONTENT",
            format!(
                "<div>{}</div>",
                &plot_data.replace("plotly-html-element", &format!("dami_{}", time))
            )
        )
    }
}
