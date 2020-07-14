//! Read/parse/write JSON files
use crate::core::dataframe::DataFrame;
use crate::core::series::Series;
use crate::io::dtypes::{
    json_is_bool, json_is_float, json_is_int, json_value_to_bool, json_value_to_float,
    json_value_to_int, json_value_to_string,
};
use crate::io::utils::read;
use serde_json::Value;
use std::cmp::min;
use std::collections::HashMap;
use std::fmt::Debug;
use std::path::Path;

/// The JSON reader
#[derive(Clone)]
pub struct JsonReader<'a> {
    data: Vec<Vec<Value>>,
    settings: HashMap<&'a str, &'a str>,
    headers: Vec<String>,
}
impl<'a> Default for JsonReader<'a> {
    fn default() -> Self {
        JsonReader {
            data: Vec::new(),
            settings: HashMap::new(),
            headers: Vec::new(),
        }
    }
}
impl<'a> JsonReader<'a> {
    /// Creates a new json reader
    pub fn new() -> Self {
        JsonReader::default()
    }
    /// Read a JSON file
    ///
    /// The path is a string pointing to a directory
    pub fn read<P: AsRef<Path> + Debug + Clone>(&mut self, path: P, lines: bool) {
        let line_terminator = self.settings.get("line_terminator").unwrap_or(&"\n");
        let data = read(path);
        if lines {
            let array = data
                .split(line_terminator)
                .collect::<Vec<&str>>()
                .into_iter()
                .map(std::string::ToString::to_string)
                .collect::<Vec<String>>();

            for line in array {
                let mut i_guess: Vec<Value> = Vec::new();
                let val: Result<Value, _> = serde_json::from_str(&line);
                match val {
                    Ok(value) => {
                        let object = value.as_object().unwrap().to_owned();
                        if self.headers.is_empty() {
                            let headers = object.keys();
                            for i in headers {
                                self.headers.push(i.to_owned());
                            }
                        }
                        let vals = object.values();
                        for each in vals {
                            i_guess.push(each.to_owned());
                        }
                        self.smart_push(i_guess);
                    }
                    Err(_) => continue,
                };
            }
        } else {
            let val: Result<Value, _> = serde_json::from_str(&data);
            if let Ok(value) = val {
                let object = value.as_object().unwrap().to_owned();
                if self.data.is_empty() {
                    let headers = object.keys();
                    for i in headers {
                        self.headers.push(i.to_string());
                    }
                }
                for i in object {
                    self.smart_push(vec![i.1]);
                }
            };
        };
    }
    fn smart_push(&mut self, data: Vec<Value>) {
        for (pos, record) in data.into_iter().enumerate() {
            let in_pos = self.data.get_mut(pos);
            if let Some(pos) = in_pos {
                pos.push(record);
            } else {
                self.data.push(vec![record]);
            }
        }
    }
    /// Convert the JSON Data into a DataFrame
    pub fn to_dataframe(&self) -> DataFrame {
        let size = min(10, self.data[0].len());
        let mut df = DataFrame::new();
        for (i, j) in self.data.iter().enumerate() {
            let header = self.headers.get(i).unwrap();
            if json_is_int(&j[0..size]) {
                let mut series = Series::from(json_value_to_int(j));
                series.set_name(header.as_str());
                df.add_series(series, true).unwrap();
            } else if json_is_float(&j[0..size]) {
                let mut series = Series::from(json_value_to_float(j));
                series.set_name(header.as_str());
                df.add_series(series, true).unwrap();
            } else if json_is_bool(&j[0..size]) {
                let mut series = Series::from(json_value_to_bool(j));
                series.set_name(header.as_str());
                df.add_series(series, true).unwrap();
            } else {
                let mut series = Series::from(json_value_to_string(j));
                series.set_name(header.as_str());
                df.add_series(series, true).unwrap();
            }
        }
        df
    }
}
