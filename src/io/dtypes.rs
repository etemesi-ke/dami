//! Contains helper functions for determining and converting between various DataTypes;
use serde_json::Value;
use std::f64::NAN;
/// Determines whether a string value can be parsed to an integer.
pub fn is_int(val: &[String]) -> bool {
    val.iter().all(|f| f.parse::<i32>().is_ok())
}
/// Determines whether a string value can be parsed to a floating type.
pub fn is_float(val: &[String]) -> bool {
    val.iter().all(|f| f.parse::<f64>().is_ok())
}
/// Determines whether a string value can be parsed to a boolean
pub fn is_bool(val: &[String]) -> bool {
    val.iter().all(|f| f.parse::<i32>().is_ok())
}
/// Parse a slice of strings to a vector of floats
pub fn str_to_float(val: &[String]) -> Vec<f64> {
    let mut vec: Vec<f64> = Vec::with_capacity(val.len());
    val.iter()
        .for_each(|f| vec.push(f.parse::<f64>().unwrap_or(NAN)));
    vec
}
/// Parse a slice of strings to a vector of ints
pub fn str_to_int(val: &[String]) -> Vec<i32> {
    let mut vec = Vec::with_capacity(val.len());
    val.iter()
        .for_each(|f| vec.push(f.parse::<i32>().unwrap_or_default()));
    vec
}
/// Parse a slice of strings to a vector of booleans
pub fn str_to_bool(val: &[String]) -> Vec<bool> {
    let mut vec = Vec::with_capacity(val.len());
    val.iter()
        .for_each(|f| vec.push(f.parse::<bool>().unwrap_or_default()));
    vec
}
/// Parse a slice of strings to a vector of i64's
#[allow(dead_code)]
pub fn str_to_big_int(val: &[String]) -> Vec<i64> {
    val.iter()
        .map(|f| f.parse::<i64>().unwrap_or_default())
        .collect()
}

pub fn json_is_int(val: &[Value]) -> bool {
    val.iter().all(serde_json::value::Value::is_i64)
}
pub fn json_is_float(val: &[Value]) -> bool {
    val.iter().all(serde_json::value::Value::is_f64)
}
pub fn json_is_bool(val: &[Value]) -> bool {
    val.iter().all(serde_json::value::Value::is_boolean)
}
#[allow(clippy::cast_possible_truncation)]
pub fn json_value_to_int(val: &[Value]) -> Vec<i64> {
    val.iter().map(|v| v.as_i64().unwrap()).collect()
}
pub fn json_value_to_string(val: &[Value]) -> Vec<String> {
    let mut vec = Vec::with_capacity(val.len());
    val.iter().for_each(|f| {
        vec.push(
            f.as_str()
                .unwrap_or_else(|| panic!("Invalid json value {:?}", f))
                .to_string(),
        )
    });
    vec
}

pub fn json_value_to_bool(val: &[Value]) -> Vec<bool> {
    let mut list = Vec::with_capacity(val.len());
    val.iter().for_each(|f| {
        list.push(
            f.as_bool()
                .unwrap_or_else(|| panic!("Invalid json value {:?}", f)),
        )
    });
    list
}
pub fn json_value_to_float(val: &[Value]) -> Vec<f64> {
    let mut list = Vec::with_capacity(val.len());
    val.iter()
        .for_each(|f| list.push(f.as_f64().unwrap_or(NAN)));
    list
}
