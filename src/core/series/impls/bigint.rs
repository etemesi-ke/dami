use crate::core::series::traits::bigint::BigIntSeries;
use crate::core::series::Series;
use std::cmp::{max, min};

use crate::core::series::errors::SeriesErrors;

macro_rules! int_impl (($type:ident) => (
    impl BigIntSeries<$type> for Series<$type> {
    fn all(&self) -> bool {
        let array = self.array.to_vec();
        array.iter().all(|&x| x != 0)
    }
    fn any(&self) -> bool {
        self.array.iter().any(|&x| x != 0)
    }
    fn bool(&self)->bool{
        debug_assert_eq!(self.len(),1);
        *self.array.get(0).unwrap() != 0
    }
    fn between(&self, left: $type, right: $type, inclusive: bool) -> Series<bool> {
        let name = self.name.clone();
        let mut new_series = Series::from(self.array.mapv(|f| {
            if inclusive {
                left <= f && f <= right
            } else {
                left < f && f < right
        }}));
        new_series.set_name(name.as_str());
        new_series
    }

    fn clip(&self, lower: $type, upper: $type) -> Series<$type> {
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
    fn cum_sum(&self)-> Series<$type>{
        let mut prev_sum = 0;
        let mut vector = vec![];
        self.array.iter().enumerate().for_each(|(len,f)|
        {   if len==0{
              prev_sum=f.to_owned();
              vector.push(prev_sum);
            }
            else{
            prev_sum+=f.to_owned();
            vector.push(prev_sum);
            }
        }
        );
        return Series::from(vector);
    }
    fn cum_max(&self)->Series<$type>{
        let mut prev = 0;
        let mut cum_max=vec![];
        for (len,f) in self.array.into_iter().enumerate(){
            if len == 0{
                prev = *f;
            }
            prev = max(prev,*f);
            cum_max.push(prev);
        }
        Series::from(cum_max)
    }
    fn cum_min(&self)->Series<$type>{
        let mut prev = 0;
        let mut cum_min=vec![];
        //TODO: Add support for NaN options without actually dropping it
        for (len,f) in self.array.into_iter().enumerate(){
            if len == 0{
                prev = *f;
            }
            prev = min(prev,*f);
            cum_min.push(prev);
        }
        Series::from(cum_min)
    }
    fn cum_prod(&self)->Series<$type>{
        let mut prev = 0;
        // Hold the result
        let mut cum_prod =vec![];
        for (len,f) in self.array.into_iter().enumerate(){
            if len == 0{
                prev = *f;
            }
            prev *= f;
            cum_prod.push(prev);
        }
        Series::from(cum_prod)
    }
    fn dot(&self,other:&Series<$type>)->Result<$type,SeriesErrors>{
        let me_arr = &self.array;
        let other_arr = &other.array;
        if self.len() == other.len(){
            // Use ndarray's dot backend
            Ok(me_arr.dot(other_arr))
        }
        // if lengths misalign raise an error
        else{
            Err(SeriesErrors::MatrixUnaligned(self.len(),other.len()))

        }
    }
 }

));
int_impl!(i64);
int_impl!(i128);
