use crate::prelude::Series;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};
//----------------------------------------------------------------------------------------------------
impl<T: Default + Clone + 'static + Add<Output = T>> Add for Series<T> {
    type Output = Series<T>;

    fn add(self, rhs: Series<T>) -> Self::Output {
        let mut series = Series::from(
            self.array
                .iter()
                .zip(rhs.to_ndarray().iter())
                .map(|(f, g)| f.to_owned() + g.to_owned())
                .collect::<Vec<T>>(),
        );
        series.set_name(&self.get_name());
        series
    }
}
impl<T: Default + Clone + 'static + Add<Output = T>> Add for &Series<T> {
    type Output = Series<T>;

    fn add(self, rhs: &Series<T>) -> Self::Output {
        let mut series = Series::from(
            self.array
                .iter()
                .zip(rhs.to_ndarray().iter())
                .map(|(f, g)| f.to_owned() + g.to_owned())
                .collect::<Vec<T>>(),
        );
        series.set_name(&self.get_name());
        series
    }
}
impl<T: Default + Clone + 'static + Add<Output = T>> AddAssign<T> for Series<T> {
    fn add_assign(&mut self, rhs: T) {
        self.array.mapv_inplace(|f| f + rhs.clone());
    }
}
impl<T: Default + Clone + 'static + Add<Output = T>> Add<T> for Series<T> {
    type Output = Series<T>;

    fn add(self, rhs: T) -> Self::Output {
        let mut series = Series::from(self.array.mapv(|f| f + rhs.clone()));
        series.set_name(&self.get_name());
        series
    }
}
//----------------------------------------------------------------------------------------------------
impl<T: Default + Clone + 'static + Div<Output = T>> Div<T> for Series<T> {
    type Output = Series<T>;

    fn div(self, rhs: T) -> Self::Output {
        let mut series = Series::from(
            self.array
                .iter()
                .map(|f| f.to_owned() / rhs.clone())
                .collect::<Vec<T>>(),
        );
        series.set_name(&self.get_name());
        series
    }
}
impl<T: Default + Clone + 'static + Div<Output = T>> Div for &Series<T> {
    type Output = Series<T>;

    fn div(self, rhs: &Series<T>) -> Self::Output {
        let mut series = Series::from(
            self.array
                .iter()
                .zip(rhs.to_ndarray().iter())
                .map(|(f, g)| f.to_owned() / g.to_owned())
                .collect::<Vec<T>>(),
        );
        series.set_name(&self.get_name());
        series
    }
}
impl<T: Default + Clone + 'static + Div<Output = T>> DivAssign<T> for Series<T> {
    fn div_assign(&mut self, rhs: T) {
        self.array.mapv_inplace(|f| f / rhs.clone());
    }
}
impl<T: Default + Clone + 'static + Div<Output = T>> Div for Series<T> {
    type Output = Series<T>;
    fn div(self, rhs: Series<T>) -> Self::Output {
        let mut series = Series::from(
            self.array
                .iter()
                .zip(rhs.to_ndarray().iter())
                .map(|(f, g)| f.to_owned() / g.to_owned())
                .collect::<Vec<T>>(),
        );
        series.set_name(&self.get_name());
        series
    }
}
//-----------------------------------------------------------------------------------------------------------
impl<T: Default + Clone + 'static + Mul<Output = T>> Mul<T> for Series<T> {
    type Output = Series<T>;

    fn mul(self, rhs: T) -> Self::Output {
        let mut series = Series::from(
            self.array
                .iter()
                .map(|f| f.to_owned() * rhs.clone())
                .collect::<Vec<T>>(),
        );
        series.set_name(&self.get_name());
        series
    }
}

impl<T: Default + Clone + 'static + Mul<Output = T>> MulAssign<T> for Series<T> {
    fn mul_assign(&mut self, rhs: T) {
        self.array.mapv_inplace(|f| f * rhs.clone());
    }
}
impl<T: Default + Clone + 'static + Mul<Output = T>> Mul for Series<T> {
    type Output = Series<T>;
    fn mul(self, rhs: Series<T>) -> Self::Output {
        let mut series = Series::from(
            self.array
                .iter()
                .zip(rhs.to_ndarray().iter())
                .map(|(f, g)| f.to_owned() * g.to_owned())
                .collect::<Vec<T>>(),
        );
        series.set_name(&self.get_name());
        series
    }
}
impl<T: Default + Clone + 'static + Mul<Output = T>> Mul for &Series<T> {
    type Output = Series<T>;
    fn mul(self, rhs: &Series<T>) -> Self::Output {
        let mut series = Series::from(
            self.array
                .iter()
                .zip(rhs.to_ndarray().iter())
                .map(|(f, g)| f.to_owned() * g.to_owned())
                .collect::<Vec<T>>(),
        );
        series.set_name(&self.get_name());
        series
    }
}
//------------------------------------------------------------------------------------------------------------------------
impl<T: Default + Clone + 'static + Sub<Output = T>> Sub<T> for Series<T> {
    type Output = Series<T>;

    fn sub(self, rhs: T) -> Self::Output {
        let mut series = Series::from(
            self.array
                .iter()
                .map(|f| f.to_owned() - rhs.clone())
                .collect::<Vec<T>>(),
        );
        series.set_name(&self.get_name());
        series
    }
}

impl<T: Default + Clone + 'static + Sub<Output = T>> SubAssign<T> for Series<T> {
    fn sub_assign(&mut self, rhs: T) {
        self.array.mapv_inplace(|f| f - rhs.clone());
    }
}
impl<T: Default + Clone + 'static + Sub<Output = T>> Sub for Series<T> {
    type Output = Series<T>;
    fn sub(self, rhs: Series<T>) -> Self::Output {
        let mut series = Series::from(
            self.array
                .iter()
                .zip(rhs.to_ndarray().iter())
                .map(|(f, g)| f.to_owned() - g.to_owned())
                .collect::<Vec<T>>(),
        );
        series.set_name(&self.get_name());
        series
    }
}
impl<T: Default + Clone + 'static + Sub<Output = T>> Sub for &Series<T> {
    type Output = Series<T>;
    fn sub(self, rhs: &Series<T>) -> Self::Output {
        let mut series = Series::from(
            self.array
                .iter()
                .zip(rhs.to_ndarray().iter())
                .map(|(f, g)| f.to_owned() - g.to_owned())
                .collect::<Vec<T>>(),
        );
        series.set_name(&self.get_name());
        series
    }
}
