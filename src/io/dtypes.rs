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
    val.iter()
        .map(|f| f.parse::<f64>().unwrap_or(NAN))
        .collect()
}
/// Parse a slice of strings to a vector of ints
pub fn str_to_int(val: &[String]) -> Vec<i32> {
    val.iter()
        .map(|f| f.parse::<i32>().unwrap_or_default())
        .collect()
}
/// Parse a slice of strings to a vector of booleans
pub fn str_to_bool(val: &[String]) -> Vec<bool> {
    val.iter()
        .map(|f| f.parse::<bool>().unwrap_or_default())
        .collect()
}
/// Parse a slice of strings to a vector of i64's
#[allow(dead_code)]
pub fn str_to_big_int(val: &[String]) -> Vec<i64> {
    val.iter()
        .map(|f| f.parse::<i64>().unwrap_or_default())
        .collect()
}

pub fn json_is_int(val: &[Value]) -> bool {
    val.iter().all(|v| v.is_i64())
}
pub fn json_is_float(val: &[Value]) -> bool {
    val.iter().all(|v| v.is_f64())
}
pub fn json_is_bool(val: &[Value]) -> bool {
    val.iter().all(|v| v.is_boolean())
}
pub fn json_value_to_int(val: &[Value]) -> Vec<i32> {
    val.iter().map(|v| v.as_i64().unwrap() as i32).collect()
}
pub fn json_value_to_string(val: &[Value]) -> Vec<String> {
    val.iter()
        .map(|f| {
            f.as_str()
                .unwrap_or_else(|| panic!("Invalid json value {:?}", f))
                .to_string()
        })
        .collect()
}

pub fn json_value_to_bool(val: &[Value]) -> Vec<bool> {
    val.iter()
        .map(|f| {
            f.as_bool()
                .unwrap_or_else(|| panic!("Invalid json value {:?}", f))
        })
        .collect()
}
pub fn json_value_to_float(val: &[Value]) -> Vec<f64> {
    val.iter()
        .map(|f| {
            f.as_f64()
                .unwrap_or_else(|| panic!("Invalid json value {:?}", f))
        })
        .collect()
}
