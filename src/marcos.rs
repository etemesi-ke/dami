#[allow(unused_imports)]
use crate::core::series::Series;
/// Convenient function for convenient for cresting series using python list like syntax
/// without limitations of 32 values using arrays
///
#[macro_export]
macro_rules! series {
    () => {Series::default()};
    ($ ($x: expr),*) => {{
        let mut vector = Vec::new();
        // push items
        $(vector.push($x);)*
        Series::from(vector)
        }
    };
}
