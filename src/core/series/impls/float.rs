#![allow(unused_imports)]
use crate::core::series::errors::SeriesErrors;
use crate::core::series::traits::floats::SeriesFloat;
use crate::core::series::Series;
use noisy_float::types::N64;

use noisy_float::types::n32;
use noisy_float::types::n64;

use std::convert::From;
use std::f64::NAN;

macro_rules! float_impl (($type:ty,$ntype:ident) => (
    impl SeriesFloat<$type> for Series<$type>{

        fn all(&self) -> bool {
            let array = self.array.to_vec();
            array.iter().all(|&x| x != 0.0)
        }

        fn any(&self) -> bool {
            self.array.iter().any(|&x| x != 0.0)
        }

        fn isnull(&self) -> Series<bool> {

           let mut series = Series::from(self.array.mapv(|f| !f.is_nan()));
           series.name = self.name.clone();
           series
        }

        fn notna(&self) -> Series<bool> {
            self.isnull()
        }
        fn between(&self, left:$type, right: $type, inclusive: bool) -> Series<bool> {
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
        fn bool(&self) -> bool {
            debug_assert_eq!(self.len(),1);
            *self.array.get(0).unwrap() != 0.0
        }

        fn clip(&self, lower:$type, upper: $type) -> Series<$type> {
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

        fn count(&self)->usize{
            let mut count:usize=0;
            self.array.iter().for_each(|f| if f.is_nan(){count+=1});
            count
        }
        fn cum_sum(&self)-> Series<$type>{
            let mut prev_sum = 0.0;
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
           let mut series = Series::from(vector);
           series.name = self.name.clone();
           series
        }
        fn cum_max(&self)->Series<$type>{
            let mut prev = $ntype(0.0);
            let mut cum_max:Vec<$type> =vec![];
            //TODO: Add support for NaN options without actually dropping it
            for (len,f) in self.array.into_iter().enumerate(){
                if len == 0{
                    prev = $ntype(*f);
                }
                // Skip nan values
                if f.is_nan(){continue}
                prev = prev.max($ntype((*f)));
                cum_max.push(prev.raw().into());
            }
           let mut series = Series::from(cum_max);
           series.name = self.name.clone();
           series
        }
        fn cum_min(&self)->Series<$type>{
            let mut prev = $ntype(0.0);
            let mut cum_min: Vec<$type> = vec![];
            //TODO: Add support for NaN options without actually dropping it
            for (len,f) in self.array.into_iter().enumerate(){
                if len == 0{
                    prev = $ntype(*f);
                }
                // Skip nan values
                if f.is_nan(){continue}
                prev = prev.min($ntype(*f));
                cum_min.push(prev.raw().into());
            }

           let mut series = Series::from(cum_min);
           series.name = self.name.clone();
           series
        }
        fn cum_prod(&self,skip_na:bool)->Series<$type>{
            let mut prev = 0.0;
            // Hold the result
            let mut cum_prod: Vec<$type>=vec![];
            for (len,f) in self.array.into_iter().enumerate(){
                if len == 0{
                    prev = *f;
                }
                // Skip nan values
                if skip_na && f.is_nan(){continue}
                prev *= f;
                cum_prod.push(prev);
            }
           let mut series = Series::from(cum_prod);
           series.name = self.name.clone();
           series
        }
        #[cfg(feature = "stats")]
        fn describe(&self)->Series<f64>{
            // The names according to how they will be stored
            let names = vec!["count","mean","stdev","pstdev","min","25%","50%","75%","max"];
            let mut described_data:Vec<f64> = Vec::with_capacity(8);
            // count
            described_data.push(self.len() as f64);
            // mean
            described_data.push(self.mean().unwrap().into());
            // standard deviation
            described_data.push(self.stdev().into());
            // Population standard deviation
            described_data.push(self.pstdev().into());
            // minimum
            described_data.push((*self.min().unwrap()).into());
            // Quantiles
            let mut convert:Vec<N64> = vec![];
            for i in self.array.iter(){
                if i.is_nan(){continue}
                else{ convert.push(n64((*i).into()));}
            }
            let mut quantiles = Series::from(convert);
            described_data.push(quantiles.quantile_axis_mut(n64(0.25)).unwrap().first().unwrap().to_owned().into());
            // We could do this better :| One day...
            described_data.push(quantiles.quantile_axis_mut(n64(0.5)).unwrap().first().unwrap().to_owned().into());
            // Don't cry its gonna be alright...
            described_data.push(quantiles.quantile_axis_mut(n64(0.75)).unwrap().first().unwrap().to_owned().into());
            // Maximum
            described_data.push((*self.max().unwrap()).into());
            // Series
            let mut  series = Series::from(described_data);
            series.name=self.name.clone();
            series.reindex(names,false);
            series
        }
        fn diff(&self,period:i32)->Series<$type>{
            // Okay rust does not allow negative index
            // a[-1] in python should be a[a.len()-1]
            let mut holder = vec![];
            // Used in negatives to tell us when to stop in order not to overflow
            let fixed = (self.len()) as i32 + period-1;
            for (len,i) in self.array.iter().enumerate(){

                if period.is_negative(){
                        if len as i32 > fixed{
                            holder.push(NAN as $type)
                        }
                        else{
                            // Bad arithmetic
                            // But it works: <>
                            // at pos 8,period -1 eg it becomes a[8]-a[7--1] which is a[8]-a[9]
                            holder.push(i-self.array[(len as i32 -period) as usize]);
                        }
                }
                else{
                    let new_period = len as i32 - period;
                    // 0-1 fetch elm 1
                    if new_period.is_negative(){
                        holder.push(NAN as $type);
                    }
                    else{
                        holder.push(i-self.array[new_period as usize])
                    }
                }
            }
            let mut series=Series::from(holder);
            series.name =self.name.clone();
            series
        }
        fn dot(&self,other:&Series<$type>)->Result<$type,SeriesErrors>{
            let me_arr = &self.array;
            let other_arr = &other.array;
            if self.len() == other.len(){
                // Use ndarray's backend
                Ok(me_arr.dot(other_arr))
            }
            // if lengths misalign raise an error
            else{
                Err(SeriesErrors::MatrixUnaligned(self.len(),other.len()))
            }
        }

        fn drop_na(&self)->Series<$type>{
           let mut arr = vec![];
           for i in self.array.iter(){
                if i.is_nan(){
                    continue
                }
                // dereference and push
                arr.push(*i);
               }
           let mut series = Series::from(arr);
           series.name = self.name.clone();
           series
        }
        fn fillna(&self,value:$type)->Series<$type>{
            Series::from(self.array.mapv(|f|{if f.is_nan(){value} else{f}}))
        }
        fn fillna_inplace(&mut self,value:$type){
            //Since array size doesn't change this is safe
            self.array = self.array.mapv(|f|{if f.is_nan(){value} else{f}});
        }
        fn first_valid_index(&self)->Option<String>{
            // TODO : Once I've implemented iter use here to prevent consuming the values
            for i in self.clone().into_iter().enumerate(){
                if !i.1.is_nan(){
                    return Some(self.index[i.0].clone())
                }
            }
            None
        }
        fn pct_change(&self,period:i32)->Series<$type>{
            // Okay rust does not allow negative index
            // a[-1] in python should be a[a.len()-1]
            let mut holder = vec![];
            // Used in negatives to tell us when to stop in order not to overflow
            let fixed = (self.len()) as i32 + period-1;
            for (len,i) in self.array.iter().enumerate(){
                if period.is_negative(){
                        if len as i32 > fixed{
                            holder.push(NAN as $type)
                        }
                        else{
                            // Bad arithmetic
                            // But it works: <>
                            // at pos 8,period -1 eg it becomes a[8]-a[7--1] which is a[8]-a[9]
                            holder.push(i/self.array[(len as i32 -period) as usize]);
                        }
                }
                else{
                    let new_period = len as i32 - period;
                    // 0-1 fetch elm 1
                    if new_period.is_negative(){
                        holder.push(NAN as $type);
                    }
                    else{
                        holder.push(i/self.array[new_period as usize])
                    }
                }
            }
            let mut series=Series::from(holder);
            series.name = self.name.clone();
            series

        }
        fn round(&self)->Series<$type>{
           let mut series = Series::from(self.array.mapv(|f| f.round()));
           series.name = self.name.clone();
           series

        }

    }
));

float_impl!(f32, n32);
float_impl!(f64, n64);
