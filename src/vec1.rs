use core::ops::{Add, Sub, Mul, Div};

pub use super::VecItem;

#[derive(Copy, Clone, Debug)]
pub struct Vec1<T: VecItem> {
    pub x: T,
}

impl<T: VecItem> Vec1<T> {
    pub fn new(x: T) -> Self { Self { x } }
}

impl<T: VecItem> From<T> for Vec1<T> {
    fn from(item: T) -> Self { Self { x: item } }
}

impl<T: VecItem> From<[T; 1]> for Vec1<T> {
    fn from(arr: [T; 1]) -> Self { Self { x: arr[0] } }
}

impl<T> Add for Vec1<T> where T: VecItem + Add, T::Output: VecItem + Add + Copy {
    type Output = Vec1<T::Output>;
    fn add(self, other: Self) -> Vec1<T::Output> {
        Vec1 {
            x: self.x + other.x,
        }
    }
}

impl<T> Sub for Vec1<T> where T: VecItem + Sub, T::Output: VecItem + Sub + Copy {
    type Output = Vec1<T::Output>;
    fn sub(self, other: Self) -> Vec1<T::Output> {
        Vec1 {
            x: self.x - other.x,
        }
    }
}

impl<T> Mul for Vec1<T> where T: VecItem + Mul, T::Output: VecItem + Mul + Copy {
    type Output = Vec1<T::Output>;
    fn mul(self, other: Self) -> Vec1<T::Output> {
        Vec1 {
            x: self.x * other.x,
        }
    }
}

impl<T> Div for Vec1<T> where T: VecItem + Div, T::Output: VecItem + Div + Copy {
    type Output = Vec1<T::Output>;
    fn div(self, other: Self) -> Vec1<T::Output> {
        Vec1 {
            x: self.x / other.x,
        }
    }
}
