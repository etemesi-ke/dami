use crate::core::common::most_frequent;
use crate::core::series::traits::strings::Str;
use crate::core::series::traits::strings::Strings;
use crate::core::series::Series;
use std::collections::HashSet;
use std::iter::FromIterator;

impl Strings for Series<String> {
    fn between(&self, left: &str, right: &str, inclusive: bool) -> Series<bool> {
        // Use self as the name of the new series
        let name = self.name.clone();
        let mut new_series = Series::from(self.array.mapv(|f| {
            if inclusive {
                left <= f.as_str() && f.as_str() <= right
            } else {
                left < f.as_str() && f.as_str() < right
            }
        }));
        new_series.set_name(name.as_str());
        new_series
    }

    fn clip<'a>(&self, lower: &'a str, upper: &'a str) -> Series<String> {
        let name = self.name.clone();
        let mut series = Series::from(self.array.mapv(|f| {
            if f.as_str() < lower {
                lower.to_string()
            } else if f.as_str() > upper {
                upper.to_string()
            } else {
                f
            }
        }));
        series.name = name;
        series
    }
    fn describe(&self) -> Series<String> {
        let index = vec!["count", "unique", "top", "freq"];
        let mut described_data = vec![];
        let len = self.len();
        described_data.push(format!("{}", len));
        let unique: HashSet<&String> = HashSet::from_iter(self.array.iter());
        described_data.push(format!("{:?}", unique));
        if len == unique.len() {
            let mut arr = self.array.to_vec();
            arr.sort();
            described_data.push(arr.get(0).unwrap().to_string());
            described_data.push("1".to_string());
        } else {
            let (freq, elm) = most_frequent(&self.array.to_vec());
            described_data.push(elm);
            described_data.push(format!("{}", freq));
        }
        let mut series = Series::from(described_data);
        series.reindex(index, false);
        series
    }

    fn lower(&self) -> Series<String> {
        let mut series = Series::from(self.array.mapv(|f| f.to_lowercase()));
        series.set_name(&self.get_name());
        series
    }
}
// Lifetimes :<|
impl Str<'static> for Series<&'static str> {
    fn between(&self, left: &str, right: &str, inclusive: bool) -> Series<bool> {
        let name = self.name.clone();
        let mut new_series = Series::from(self.array.mapv(|f| {
            if inclusive {
                left <= f && f <= right
            } else {
                left < f && f < right
            }
        }));
        new_series.set_name(name.as_str());
        new_series
    }

    fn clip(&self, lower: &'static str, upper: &'static str) -> Series<&'static str> {
        let name = self.name.clone();
        let mut series = Series::from(self.array.mapv(|f| {
            if f < lower {
                lower
            } else if f > upper {
                upper
            } else {
                f
            }
        }));
        series.name = name;
        series
    }

    fn describe(&self) -> Series<String> {
        let index = vec!["count", "unique", "top", "freq"];
        let mut described_data = vec![];
        let len = self.len();
        described_data.push(format!("{}", len));
        let unique: HashSet<&&'static str> = HashSet::from_iter(self.array.iter());
        described_data.push(format!("{:?}", unique.len()));
        if len == unique.len() {
            let mut arr = self.array.to_vec();
            arr.sort();
            described_data.push((*arr.get(0).unwrap()).to_string());
            described_data.push("1".to_string());
        } else {
            let (freq, elm) = most_frequent(&self.array.to_vec());
            described_data.push(elm.to_string());
            described_data.push(format!("{}", freq));
        }
        let mut series = Series::from(described_data);
        series.reindex(index, false);
        series
    }

    fn lower(&self) -> Series<String> {
        let mut series = Series::from(self.array.mapv(str::to_lowercase));
        series.set_name(&self.get_name());
        series
    }
}
