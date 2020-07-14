# Series
A one dimensional array with axis labels
```text
__________________
|index  | elm    |
|-----------------
|index2 | elm2   |
------------------
|index3 | elm3   |
------------------
```
The above shows the structure of a Series

An index is basically a vector of strings, each index points to the element in the array
and can be used for Indexing the array

The elements are stored in a one dimensional [ndarray] which supports slicing, splitting and other
cool stuff

To create a new Series, use  *from* methods currently supported ones are from a [HashMap] with a len of 1
a [Vec] and  a slice generic slice `T`

## Creating a series
They're different methods for creating a series
> ### From a [Vec]
>  **Example**
> ```rust
> extern crate dami;
> use dami::core::series::Series;
> fn main(){
>     let series = Series::from(vec![0,1,2,3,4,5,6,7,8,9]);
>     // Voila , you have a series...
> }
> ```
> ### From  a [HashMap]
>They're two ways to create a series from a [HashMap] 
> #### Method 1
> > A HashMap<String,Vec\<T>> of length 1
> >
> > The Key will be set as the HashMap's name, and the vec will be set as the underlying array. 
> >
> > **Example**
> >```rust
> > extern crate dami;
> > use dami::core::series::Series;
> > use std::collections::HashMap;
> > use std::string::ToString;
> > fn main(){
> >    let mut map = HashMap::new();
> >    map.insert("Hello".to_string(),vec![1,2,3,4,5,6,8,9]);
> >    let series = Series::try_from(map).unwrap();
> >    // Voila , you have a series...
> > } 
> >```
>
> #### Method 2
>> A HashMap<&str,T>
>> The keys will become the index, and the values will become the array
>>
>> ** Example**
>>```rust
>> use dami::core::series::Series;
>> use std::collections::HashMap;
>> fn main(){
>>    let mut map = HashMap::new();
>>    map.insert("Hello",1);
>>    map.insert("world",2);
>>    let series = Series::from(map);
>>    // Voila , you have a series...
>> } 
>>```

> #### From an [ndarray] of dimension 1
> The array becomes the underlying series array
> 
> **Example**
> ```rust
> use dami::core::series::Series;
> use ndarray::Array1;
> fn main(){
> 	let new_array = Array1::from(vec![0,1,2,3,4,5,6,7]);
> 	let series = Series::from(new_array);
> }
> ```
> #### From an array
> An array in rust is created by square brackets eg `[0,1,3,4]` is an array.
> A series can be created from an array of up to **32** values. this makes `Series::from([0,1,2,4])` valid
> The array must consists of only one type of element
> ```rust
> use dami::core::series::Series;
> fn main(){
>	let series = Series::from([0,1,2,3,4,5,6]);
>}
> ```

## Indexing a Series
Indexing a series can be performed using either a &str or a  usize

**Example**
```rust
use dami::core::series::Series;
fn main(){
	let series = Series::from([1,2,3,4,5,6,7]);
   assert_eq!(series["0"],1);// True since index at zero is 1
   assert_eq!(index[1],2); // True
}
```
## Series Methods
### `abs(&self)`
> Implemented for: [ [f64],[f32],[i32],[i64],[i128]]
> 
> Not implemented for: [[str],[String]]

> Returns a Series with only absolute elements
> 
> ** Example **
> ```rust
> use dami::core::series::Series;
> fn main(){
>    let new_series = Series::from(vec![0,-4,2,21,-23]);
>    let updated_series = new_series.abs();
>    assert_ne!(new_series,updated_series); // Since one contains absolute values
 }
> ```

### `add_prefix(&mut self,prefix:&str)`
> Implemented for: All types
> 
> Add a prefix to the labels.
> This modifies the row label for the series
> 
> **Arguments**
> > prefix:[str]
> > > The string to add before each label

> This modifies the current series index

### `add_suffix(&mut self,suffix:&str)`
> Implemented for: All types
> 
> Add a suffix to the labels.
> This modifies the row label for the series
> 
> **Arguments**
> > suffix:[str]
> > > The string to add after each label

> This modifies the current series index

### `all(&self)`
> Implemented for : [[f64],[f32],[i64],[i32],[i128]]
> 
>Not implemented for [[str],[String]]



[f64]:https://doc.rust-lang.org/std/primitive.f64.html
[f32]:https://doc.rust-lang.org/std/primitive.f32.html
[i32]:https://doc.rust-lang.org/std/primitive.i32.html
[i64]:https://doc.rust-lang.org/std/primitive.i64.html
[i128]:https://doc.rust-lang.org/std/primitive.i128.html

[str]: https://doc.rust-lang.org/std/primitive.str.html
[String]:https://doc.rust-lang.org/std/string/struct.String.html

[ndarray]: https://docs.rs/ndarray/
[`Vec<T>`]: https://doc.rust-lang.org/std/collections/struct.HashMap.html
[Vec]:https://doc.rust-lang.org/std/collections/struct.HashMap.html
[HashMap]:https://doc.rust-lang.org/std/collections/struct.HashMap.html