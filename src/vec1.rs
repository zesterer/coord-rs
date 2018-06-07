use core::ops::{Add, Sub, Mul, Div, AddAssign, SubAssign, MulAssign, DivAssign};
use num::{Num, Integer, Unsigned, Signed, Float};
use super::{Vec, VecItem, VecNum, VecInt, VecUnsigned, VecSigned, VecFloat};

#[derive(Copy, Clone, Debug)]
pub struct Vec1<T: VecItem> {
    pub x: T,
}

impl<T: VecItem> Vec1<T> {
    pub fn new(x: T) -> Self { Self { x } }
}

impl<T: VecItem> Vec for Vec1<T> {
    type Item = T;
}

// From traits

impl<T: VecItem> From<T> for Vec1<T> {
    fn from(item: T) -> Self { Self { x: item } }
}

impl<T: VecItem> From<[T; 1]> for Vec1<T> {
    fn from(arr: [T; 1]) -> Self { Self { x: arr[0] } }
}

// Op traits

impl<T> Add for Vec1<T> where T: VecItem + Add, T::Output: VecItem + Add {
    type Output = Vec1<T::Output>;
    fn add(self, other: Self) -> Vec1<T::Output> {
        Vec1 {
            x: self.x + other.x,
        }
    }
}

impl<T> Sub for Vec1<T> where T: VecItem + Sub, T::Output: VecItem + Sub {
    type Output = Vec1<T::Output>;
    fn sub(self, other: Self) -> Vec1<T::Output> {
        Vec1 {
            x: self.x - other.x,
        }
    }
}

impl<T> Mul for Vec1<T> where T: VecItem + Mul, T::Output: VecItem + Mul {
    type Output = Vec1<T::Output>;
    fn mul(self, other: Self) -> Vec1<T::Output> {
        Vec1 {
            x: self.x * other.x,
        }
    }
}

impl<T> Div for Vec1<T> where T: VecItem + Div, T::Output: VecItem + Div {
    type Output = Vec1<T::Output>;
    fn div(self, other: Self) -> Vec1<T::Output> {
        Vec1 {
            x: self.x / other.x,
        }
    }
}

// Assign operators

impl<T> AddAssign for Vec1<T> where T: VecItem + Add<Output=T> {
    fn add_assign(&mut self, other: Self) {
        *self = Vec1 {
            x: self.x + other.x,
        }
    }
}

impl<T> SubAssign for Vec1<T> where T: VecItem + Sub<Output=T> {
    fn sub_assign(&mut self, other: Self) {
        *self = Vec1 {
            x: self.x - other.x,
        }
    }
}

impl<T> MulAssign for Vec1<T> where T: VecItem + Mul<Output=T> {
    fn mul_assign(&mut self, other: Self) {
        *self = Vec1 {
            x: self.x * other.x,
        }
    }
}

impl<T> DivAssign for Vec1<T> where T: VecItem + Div<Output=T> {
    fn div_assign(&mut self, other: Self) {
        *self = Vec1 {
            x: self.x / other.x,
        }
    }
}

// VecNum traits

impl<T> VecNum for Vec1<T> where T: VecItem + Num {
    fn sum(&self) -> Self::Item {
        self.x
    }

    fn product(&self) -> Self::Item {
        self.x
    }
}

// VecSigned traits

impl<T> VecSigned for Vec1<T> where T: VecItem + Signed {
    fn snake_length(&self) -> Self::Item {
        self.x.abs()
    }
}

// VecFloat traits

impl<T> VecFloat for Vec1<T> where T: VecItem + Float {
    fn length(&self) -> Self::Item {
        self.x
    }
}
