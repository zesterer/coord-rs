//! Functionality pertaining to `Vec2`

use core::ops::{Add, Sub, Mul, Div, AddAssign, SubAssign, MulAssign, DivAssign};
use core::fmt;

#[allow(unused_imports)]
use num::{Num, Integer, Unsigned, Signed, Float};

#[allow(unused_imports)]
use super::{Vector, VecItem, VecNum, VecInt, VecUnsigned, VecSigned, VecFloat};

#[cfg(feature = "serialize")]
#[derive(Copy, Clone, Default, Hash, Serialize, Deserialize)]
pub struct Vec2<T: VecItem> {
    pub x: T,
    pub y: T,
}

#[cfg(not(feature = "serialize"))]
#[derive(Copy, Clone, Default, Hash)]
pub struct Vec2<T: VecItem> {
    pub x: T,
    pub y: T,
}

impl<T: VecItem> Vec2<T> {
    /// Creates a new Vec2 from multiple components
    pub fn new(x: T, y: T) -> Self { Self { x, y, ..Default::default() } }

    /// Returns the elements of the vector as an array
    pub fn elements(&self) -> [T; 2] { [self.x, self.y] }
}

impl<T: VecItem> Vector for Vec2<T> {
    type Item = T;
}

// Debug and Display traits

impl<T: VecItem + fmt::Debug> fmt::Debug for Vec2<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(x: {:?}, y: {:?})", self.x, self.y)
    }
}

impl<T: VecItem + fmt::Display> fmt::Display for Vec2<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

// From traits

impl<T: VecItem> From<[T; 2]> for Vec2<T> {
    fn from(arr: [T; 2]) -> Self { Self { x: arr[0], y: arr[1], ..Default::default() } }
}

impl<T: VecItem> From<(T, T)> for Vec2<T> {
    fn from(tup: (T, T)) -> Self { Self { x: tup.0, y: tup.1, ..Default::default() } }
}

// Op traits

impl<T> Add for Vec2<T> where T: VecItem + Add, T::Output: VecItem + Add {
    type Output = Vec2<T::Output>;
    fn add(self, other: Self) -> Vec2<T::Output> {
        Vec2 {
            x: self.x + other.x,
            y: self.y + other.y,
            ..Default::default()
        }
    }
}

impl<T> Sub for Vec2<T> where T: VecItem + Sub, T::Output: VecItem + Sub {
    type Output = Vec2<T::Output>;
    fn sub(self, other: Self) -> Vec2<T::Output> {
        Vec2 {
            x: self.x - other.x,
            y: self.y - other.y,
            ..Default::default()
        }
    }
}

impl<T> Mul for Vec2<T> where T: VecItem + Mul, T::Output: VecItem + Mul {
    type Output = Vec2<T::Output>;
    fn mul(self, other: Self) -> Vec2<T::Output> {
        Vec2 {
            x: self.x * other.x,
            y: self.y * other.y,
            ..Default::default()
        }
    }
}

impl<T> Div for Vec2<T> where T: VecItem + Div, T::Output: VecItem + Div {
    type Output = Vec2<T::Output>;
    fn div(self, other: Self) -> Vec2<T::Output> {
        Vec2 {
            x: self.x / other.x,
            y: self.y / other.y,
            ..Default::default()
        }
    }
}

// Op primitive traits

impl<T> Add<T> for Vec2<T> where T: VecItem + Add, T::Output: VecItem + Add {
    type Output = Vec2<T::Output>;
    fn add(self, other: T) -> Vec2<T::Output> {
        Vec2 {
            x: self.x + other,
            y: self.y + other,
            ..Default::default()
        }
    }
}

impl<T> Sub<T> for Vec2<T> where T: VecItem + Sub, T::Output: VecItem + Sub {
    type Output = Vec2<T::Output>;
    fn sub(self, other: T) -> Vec2<T::Output> {
        Vec2 {
            x: self.x - other,
            y: self.y - other,
            ..Default::default()
        }
    }
}

impl<T> Mul<T> for Vec2<T> where T: VecItem + Mul, T::Output: VecItem + Mul {
    type Output = Vec2<T::Output>;
    fn mul(self, other: T) -> Vec2<T::Output> {
        Vec2 {
            x: self.x * other,
            y: self.y * other,
            ..Default::default()
        }
    }
}

impl<T> Div<T> for Vec2<T> where T: VecItem + Div, T::Output: VecItem + Div {
    type Output = Vec2<T::Output>;
    fn div(self, other: T) -> Vec2<T::Output> {
        Vec2 {
            x: self.x / other,
            y: self.y / other,
            ..Default::default()
        }
    }
}

// Assign operators

impl<T> AddAssign for Vec2<T> where T: VecItem + Add<Output=T> {
    fn add_assign(&mut self, other: Self) {
        *self = Vec2 {
            x: self.x + other.x,
            y: self.y + other.y,
            ..Default::default()
        }
    }
}

impl<T> SubAssign for Vec2<T> where T: VecItem + Sub<Output=T> {
    fn sub_assign(&mut self, other: Self) {
        *self = Vec2 {
            x: self.x - other.x,
            y: self.y - other.y,
            ..Default::default()
        }
    }
}

impl<T> MulAssign for Vec2<T> where T: VecItem + Mul<Output=T> {
    fn mul_assign(&mut self, other: Self) {
        *self = Vec2 {
            x: self.x * other.x,
            y: self.y * other.y,
            ..Default::default()
        }
    }
}

impl<T> DivAssign for Vec2<T> where T: VecItem + Div<Output=T> {
    fn div_assign(&mut self, other: Self) {
        *self = Vec2 {
            x: self.x / other.x,
            y: self.y / other.y,
            ..Default::default()
        }
    }
}

// Assign primitive operators

impl<T> AddAssign<T> for Vec2<T> where T: VecItem + Add<Output=T> {
    fn add_assign(&mut self, other: T) {
        *self = Vec2 {
            x: self.x + other,
            y: self.y + other,
            ..Default::default()
        }
    }
}

impl<T> SubAssign<T> for Vec2<T> where T: VecItem + Sub<Output=T> {
    fn sub_assign(&mut self, other: T) {
        *self = Vec2 {
            x: self.x - other,
            y: self.y - other,
            ..Default::default()
        }
    }
}

impl<T> MulAssign<T> for Vec2<T> where T: VecItem + Mul<Output=T> {
    fn mul_assign(&mut self, other: T) {
        *self = Vec2 {
            x: self.x * other,
            y: self.y * other,
            ..Default::default()
        }
    }
}

impl<T> DivAssign<T> for Vec2<T> where T: VecItem + Div<Output=T> {
    fn div_assign(&mut self, other: T) {
        *self = Vec2 {
            x: self.x / other,
            y: self.y / other,
            ..Default::default()
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

    fn normalize(&self) -> Self {
        let len = self.length();
        Vec2::new(
            self.x / len,
            self.y / len,
        )
    }
}
