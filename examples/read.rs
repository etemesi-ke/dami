#![feature(test)]

extern crate dami;
extern crate test;

use dami::prelude::Strings;
use std::collections::HashMap;

#[test]
fn read_remote_csv() {
    /// Read a remote CSV into a dataframe
    use dami::io::parser::read_csv;
    read_csv(
        "https://raw.githubusercontent.com/petewarden/dstkdata/master/uk_district_names.csv",
        None,
    );
}
#[test]
fn test_remote_zipped_csv() {
    // Zipped CSV
    // Note this writes the CSV into a temporary directory
    use dami::io::parser::read_csv;
    let d =read_csv("https://www.stats.govt.nz/assets/Uploads/Retail-trade-survey/Retail-trade-survey-March-2020-quarter/Download-data/retail-trade-survey-march-2020-quarter.csv.zip",
             None);
    println!("{:?}", d);
}
#[test]
fn read_csv_with_options() {
    /// Read a local CSV
    use dami::io::parser::read_csv;
    let mut options = HashMap::new();
    options.insert("names", "index,1,fiver,3,life");
    let dir = concat!(env!("CARGO_MANIFEST_DIR"), "/examples/data/a.csv");
    let reader = read_csv(dir, None);
    println!("{:?}", reader);
}
#[test]
fn read_fwf() {
    /// Read a fixed width file
    use dami::io::parser::read_fwf;
    let dir = concat!(env!("CARGO_MANIFEST_DIR"), "/examples/data/bar.csv");

    read_fwf(dir, Some(vec![(0, 6), (10, 20), (23, 33), (36, 43)]), None);
}
#[test]
fn test_json() {
    use serde::Deserialize;
    #[derive(Debug, Deserialize)]
    struct JSON {
        category: String,
        headline: String,
        authors: String,
        link: String,
        short_description: String,
        date: String,
    }
    let dir = concat!(env!("CARGO_MANIFEST_DIR"), "/examples/data/test.json");

    use dami::io::json::JsonReader;
    let mut reader = JsonReader::new();
    reader.read(dir, true);
    let df = reader.to_dataframe();
    println!("{:?}", df.get_series::<String>("link").unwrap());
    println!(
        "{:?}",
        &df.get_series::<String>("category").unwrap().lower()[5]
    );
}
#[test]
fn read_compressed_zip_with_options() {
    use dami::io::parser::read_csv;
    let dir = concat!(env!("CARGO_MANIFEST_DIR"), "/examples/data/smallpop.zip");
    let mut options = HashMap::new();
    options.insert("names", "country,region,city,population");
    let p = read_csv(dir, None);
    println!("{:?}", p);
    println!("{:?}", p.dtypes());
}
#[cfg(feature = "alloc")]
fn current() -> usize {
    epoch::advance().unwrap();
    stats::allocated::read().unwrap()
}
fn main() {}
