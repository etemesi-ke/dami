//! This module contains traits for rust [`String`] and [`str`] functions
use crate::core::series::Series;
/// This trait exports [`String`] functions for series.
/// # Note:
///  Most functions are not exported eg `not_null` or bool as they manipulate integers
pub trait Strings {
    /// Return a boolean series equivalent to left <= series <= right
    ///
    /// # Arguments
    /// * `left`: Left boundary
    /// > * type:&str
    /// * `right`: Right boundary
    /// > * type:&str
    /// * inclusive: If set to true series equivalent becomes left <= series <= right
    /// if false  left < series < right
    ///  # Returns
    /// [`Series`] representing whether each element is between left and right
    /// # Example
    /// ```
    /// use dami::core::series::Series;
    /// use dami::core::series::traits::strings::Strings;
    /// fn main(){
    /// let series = Series::from(vec!["a".to_string(),"b".to_string(),"c".to_string(),"g".to_string()]);
    ///     let b= series.between("a","f",true);
    ///     assert_eq!(b,Series::from(vec![true,true,true,false]))
    /// }
    /// ```
    fn between(&self, left: &str, right: &str, inclusive: bool) -> Series<bool>;
    /// Trim values at input thresholds
    ///
    /// Assigns values outside the boundary to boundary values
    ///
    /// # Arguments
    /// * `lower`: Minimum threshold value. All values below this  threshold will be set to it
    /// > * `type`: &str
    /// * `upper` : Maximum threshold value. All values above this threshold will be set to it
    /// > *`type`: &str
    ///
    /// # Returns
    /// [`Series`] With the same type as calling object with values outside the clip boundaries replaced
    fn clip<'a>(&self, lower: &'a str, upper: &'a str) -> Series<String>;
    /// Describe a series
    ///
    /// The result’s index will include count, unique, top, and freq.
    ///
    /// Count is the number of values
    ///
    /// The top is the most common value.
    ///
    /// The freq is the most common value’s frequency
    ///
    /// # Example
    /// ```
    /// use dami::core::series::Series;
    /// use dami::core::series::traits::strings::Strings;
    /// fn main(){
    ///     let series = Series::from(["hello".to_string(),"world".to_string(),"hello".to_string()]);
    ///     println!("{:?}",series.describe());
    /// }
    /// ```
    /// Prints
    /// ```text
    /// count   3
    /// unique  2
    /// top     "hello"
    /// freq    2
    /// ```
    fn describe(&self) -> Series<String>;
    /// Lower-cases all strings in the series.
    fn lower(&self) -> Series<String>;
}
/// This trait exports [`str`] functions for series.
/// # Note:
///  Most functions are not exported eg not_null or bool as they manipulate integers
///
/// [`str`]: https://doc.rust-lang.org/nightly/std/primitive.str.html
pub trait Str<'a> {
    /// Return a boolean series equivalent to left <= series <= right
    ///
    /// # Arguments
    /// * `left`: Left boundary
    /// > * type:&str
    /// * `right`: Right boundary
    /// > * type:&str
    /// * `inclusive`: If set to true series equivalent becomes left <= series <= right
    /// if false  left < series < right
    ///  # Returns
    ///  [`Series`] representing whether each element is between left and right
    /// # Example
    /// ```
    /// use dami::core::series::Series;
    /// use dami::core::series::traits::strings::Str;
    /// fn main(){
    ///     let series = Series::from(vec!["a","b","c","g"]);
    ///     let b= series.between("a","f",true);
    ///     assert_eq!(b,Series::from(vec![true,true,true,false]))
    /// }
    /// ```
    fn between(&self, left: &str, right: &str, inclusive: bool) -> Series<bool>;
    /// Trim values at input thresholds
    ///
    /// Assigns values outside the boundary to boundary values
    ///
    /// # Arguments
    /// * `lower`: Minimum threshold value. All values below this  threshold will be set to it
    /// > * `type`: &str
    /// * `upper` : Maximum threshold value. All values above this threshold will be set to it
    /// > * `type`: &str
    ///
    /// # Returns
    /// [`Series`] With the same type as calling object with values outside the clip boundaries replaced
    ///
    /// # Example
    /// ```
    /// use crate::dami::core::series::Series;
    /// use dami::core::series::traits::strings::Str;
    /// fn main(){
    ///     let series =  Series::from(vec!["1","2","3","4","5"]);
    ///     let series_b = series.clip("0","3");
    ///     assert_eq!(series_b,Series::from(vec!["1","2","3","3","3"]))
    /// }
    /// ```
    fn clip(&self, lower: &'static str, upper: &'static str) -> Series<&'static str>;
    /// Describe a series
    ///
    /// The result’s index will include count, unique, top, and freq.
    ///
    /// Count is the number of values
    ///
    /// The top is the most common value.
    ///
    /// The freq is the most common value’s frequency
    ///
    /// # Example
    /// ```
    /// use dami::core::series::Series;
    /// use dami::core::series::traits::strings::Str;
    /// fn main(){
    ///     let series = Series::from(["hello","world","hello"]);
    ///     println!("{:?}",series.describe());
    /// }
    /// ```
    /// Prints
    /// ```text
    /// count   3
    /// unique  2
    /// top     "hello"
    /// freq    2
    /// ```
    fn describe(&self) -> Series<String>;
    /// Lower-cases all the elements of the series
    fn lower(&self) -> Series<String>;
}
