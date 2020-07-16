# dami 
## Data Manipulations in Rust
## Supported features out of the box
* Reading csv files(both local and remote
* Reading JSON files (both local and remote)
* Statistical functions `enabled by default` allowing for plotting and higher order statistical functions
like `kurtosis()` and `skewness()`
* Reading compressed zipped,lzma and xz files
* Plotting for `i32`, `i64`, `i128`,`f32` and `f64` Series types and DataFrames
* Support for `evcxr rust jupyter` runtimes with methods that contain `_excvr` 
* Well documented code.

 # Pluggable features
* `feature="remote"`: Allows for reading remote (compressed and non-compressed) files 
* `feature="excel"`: Read excel/odf spreadsheet files
* `feature="hdf5"` : Read hdf files 
* `feature="clipboard"` : Read data from clipboard and pass it to the CSV Reader
## Limitations 
* DataFrames cannot be **indexed**
```python3
df['col1']=df['col2']+df['col3']
```

> Might be something possible in python. DataFrames
But in Rust. `col2` and `col3` types cannot be determined during compile time
Though it's possible with 
```rust
df.add_col("col3",df.get::<f64>("col1")+df.get::<f64>("col2"))
```
* Some functions require explicit types specified
The following is a function in python tor square root all `f64` types
```python
import pandas as pd, numpy as np
df =pd.DataFrame(np.random.rand((50,4))
df.apply(np.sqrt) # root all numbers
```
> The following function does the same in Dami

```rust
extern crate dami;
extern crate ndarray;
extern crate ndarray_rand;

use dami:: prelude::*;
use ndarray::Array2;
use ndarray_rand::{Uniform}
fn main(){
   let df = DataFrame::from(Array2::random((50,4), Uniform::from(10.,100.));
   df.apply::<f64,_>::(f64::sqrt); root all floats
}
```
> As an added benefit only series types  containing `f64` floats 
will be square rooted even if the DataFrame contains heterogeneous Data as opposed to `pandas.DataFrame.apply`
which will try applying the function to all types. Which will panic if it meets a string type
