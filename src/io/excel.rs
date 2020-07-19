//! Read and deserialize spreadsheet files using [calamine]
//!
//! [calamine]:https://docs.rs/calamine/
#![cfg(feature="calamine")]
use crate::core::dataframe::DataFrame;
use crate::core::series::Series;
use crate::enums::DataTypes;

use calamine::{open_workbook_auto, CellErrorType, DataType, Reader};

/// Read,read and read
pub fn read_excel(
    path: &str,
    sheet: &str,
    headers: bool,
    dtypes: Option<Vec<DataTypes>>,
) -> DataFrame {
    // Let us select sheet names using regex, allowing us to match patterns and also names
    let sheet_names: Vec<String> = sheet.split(",").collect();
    // Open
    let mut workbook = open_workbook_auto(path.clone())
        .expect(format!("Could not open workbook at {}", path.clone()).as_str());
    // Get sheet names
    let mut all_sheets = workbook.sheet_names().to_vec();
    // Retain sheet names matching the regular expression
    all_sheets.retain(|f| sheet_names.contains(f));

    let mut data_frame = DataFrame::new();

    let master = dtypes;
    for i in all_sheets {
        // We know its there. So its safe to unwrap
        let loaded_sheet = workbook.worksheet_range(i.as_str()).unwrap().unwrap();
        if loaded_sheet.is_empty() {
            continue;
        };
        // Collect rows to a vec and remove any entry with errors
        //  This is expensive but avoids a lot of errors that may come later
        let mut rows = loaded_sheet
            .rows()
            .map(|f| f.to_vec())
            .collect::<Vec<Vec<DataType>>>();
        rows.retain(|f| {
            // Why are we matching all of this :<|
            // Can't ewe do this better :<(
            // TODO: Do this better
            if f.contains(&DataType::Empty)
                || f.contains(&DataType::Error(CellErrorType::Div0))
                || f.contains(&DataType::Error(CellErrorType::Null))
                || f.contains(&DataType::Error(CellErrorType::Num))
                || f.contains(&DataType::Error(CellErrorType::Value))
                || f.contains(&DataType::Error(CellErrorType::Name))
                || f.contains(&DataType::Error(CellErrorType::GettingData))
                || f.contains(&DataType::Error(CellErrorType::Ref))
            {
                false
            } else {
                true
            }
        });
        // If the first line is headers, take it and store it
        let mut my_names = vec![];
        // Only support string headers
        if headers {
            for i in 0..loaded_sheet.width() {
                // Hopefully one day will add support for other header types
                my_names.push(
                    loaded_sheet
                        .get((0, i))
                        .unwrap()
                        .to_owned()
                        .get_string()
                        .expect("Only headers which are string type are supported")
                        .to_string(),
                );
            }
        }
        // We have the specified data types for each row, so we use those
        if master.is_some() == true {
            let the_types = master.clone().unwrap();
            let dtypes_len = the_types.len();
            // Assert that the keywords and row widths are equal
            assert_eq!(loaded_sheet.width(),dtypes_len,
                       "DataType keyword and the spreadsheet file have incompatible lengths,datatype{},sheet{}",
                       dtypes_len,loaded_sheet.width());
            for (pos, types) in the_types.iter().enumerate() {
                //Calamine supports i64,f64,String,Bool
                match types {
                    DataTypes::I64 | DataTypes::I32 => {
                        let mut series: Vec<i64> = vec![];
                        for (ptr, i) in rows.iter().enumerate() {
                            // Sometimes Calamine treats int as f64 types. So lets cover that here
                            if ptr == 0 && headers {
                                continue;
                            };
                            series.push(i[pos].get_int()
                                .expect("Could not get integer type\n Sometimes Calamine (the underlying parser) treats integers as floating points, try using DataTypes::F64 instead"))
                        }
                        let mut new_series = Series::from(series);
                        // If the file has headers. Push that
                        if headers {
                            new_series.set_name(my_names[pos].as_str())
                        }
                        data_frame
                            .add_series(Series::from(new_series), true)
                            .unwrap();
                    }
                    DataTypes::F64 | DataTypes::F32 => {
                        // Repeat
                        let mut series: Vec<f64> = vec![];
                        for (ptr, i) in rows.iter().enumerate() {
                            if ptr == 0 && headers {
                                continue;
                            };
                            if i[pos].get_float().is_some() {
                                series.push(i[pos].get_float().unwrap())
                            }
                        }
                        let mut new_series = Series::from(series);
                        if new_series.is_empty() {
                            continue;
                        }
                        if headers {
                            new_series.set_name(my_names[pos].as_str())
                        }
                        data_frame
                            .add_series(Series::from(new_series), true)
                            .unwrap();
                    }
                    DataTypes::STR | DataTypes::STRING => {
                        // TODO: See if its applicable to use macros here
                        let mut series: Vec<String> = vec![];
                        for (ptr, i) in rows.iter().enumerate() {
                            if ptr == 0 && headers {
                                continue;
                            };
                            if i[pos].get_string().is_some() {
                                series.push(i[pos].get_string().unwrap().to_string())
                            }
                        }
                        let mut new_series = Series::from(series);
                        if new_series.is_empty() {
                            continue;
                        }
                        if headers {
                            new_series.set_name(my_names[pos].as_str())
                        }
                        data_frame
                            .add_series(Series::from(new_series), true)
                            .unwrap();
                    }
                    _ => continue,
                }
            }
        }
    }
    data_frame
}
