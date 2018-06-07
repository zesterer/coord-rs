use core::ops::{Add, Sub, Mul, Div, AddAssign, SubAssign, MulAssign, DivAssign};
use num::{Num, Integer, Unsigned, Signed, Float};
use super::{Vec, VecItem, VecNum, VecInt, VecUnsigned, VecSigned, VecFloat};

#[derive(Copy, Clone, Debug)]
pub struct Vec2<T: VecItem> {
    pub x: T,
    pub y: T,
}

impl<T: VecItem> Vec2<T> {
    pub fn new(x: T, y: T) -> Self { Self { x, y } }
}

impl<T: VecItem> Vec for Vec2<T> {
    type Item = T;
}

// From traits

impl<T: VecItem> From<[T; 2]> for Vec2<T> {
    fn from(arr: [T; 2]) -> Self { Self { x: arr[0], y: arr[1] } }
}

impl<T: VecItem> From<(T, T)> for Vec2<T> {
    fn from(tup: (T, T)) -> Self { Self { x: tup.0, y: tup.1 } }
}

// Op traits

impl<T> Add for Vec2<T> where T: VecItem + Add, T::Output: VecItem + Add {
    type Output = Vec2<T::Output>;
    fn add(self, other: Self) -> Vec2<T::Output> {
        Vec2 {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl<T> Sub for Vec2<T> where T: VecItem + Sub, T::Output: VecItem + Sub {
    type Output = Vec2<T::Output>;
    fn sub(self, other: Self) -> Vec2<T::Output> {
        Vec2 {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl<T> Mul for Vec2<T> where T: VecItem + Mul, T::Output: VecItem + Mul {
    type Output = Vec2<T::Output>;
    fn mul(self, other: Self) -> Vec2<T::Output> {
        Vec2 {
            x: self.x * other.x,
            y: self.y * other.y,
        }
    }
}

impl<T> Div for Vec2<T> where T: VecItem + Div, T::Output: VecItem + Div {
    type Output = Vec2<T::Output>;
    fn div(self, other: Self) -> Vec2<T::Output> {
        Vec2 {
            x: self.x / other.x,
            y: self.y / other.y,
        }
    }
}

// Assign operators

impl<T> AddAssign for Vec2<T> where T: VecItem + Add<Output=T> {
    fn add_assign(&mut self, other: Self) {
        *self = Vec2 {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl<T> SubAssign for Vec2<T> where T: VecItem + Sub<Output=T> {
    fn sub_assign(&mut self, other: Self) {
        *self = Vec2 {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl<T> MulAssign for Vec2<T> where T: VecItem + Mul<Output=T> {
    fn mul_assign(&mut self, other: Self) {
        *self = Vec2 {
            x: self.x * other.x,
            y: self.y * other.y,
        }
    }
}

impl<T> DivAssign for Vec2<T> where T: VecItem + Div<Output=T> {
    fn div_assign(&mut self, other: Self) {
        *self = Vec2 {
            x: self.x / other.x,
            y: self.y / other.y,
        }
    }
}

// VecNum traits

impl<T> VecNum for Vec2<T> where T: VecItem + Num {
    fn sum(&self) -> Self::Item {
        self.x + self.y
    }

    fn product(&self) -> Self::Item {
        self.x * self.y
    }
}

// VecSigned traits

impl<T> VecSigned for Vec2<T> where T: VecItem + Signed {
    fn snake_length(&self) -> Self::Item {
        self.x.abs() + self.y.abs()
    }
}

// VecFloat traits

impl<T> VecFloat for Vec2<T> where T: VecItem + Float {
    fn length(&self) -> Self::Item {
        (self.x * self.x + self.y * self.y).sqrt()
    }
}
