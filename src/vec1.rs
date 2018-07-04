//! Functionality pertaining to `Vec1`

use core::ops::{Add, Sub, Mul, Div, AddAssign, SubAssign, MulAssign, DivAssign};
use core::fmt;

#[allow(unused_imports)]
use num::{Num, Integer, Unsigned, Signed, Float};

#[allow(unused_imports)]
use super::{Vector, VecItem, VecNum, VecInt, VecUnsigned, VecSigned, VecFloat};

#[cfg(feature = "serialize")]
#[derive(Copy, Clone, Default, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct Vec1<T: VecItem> {
    pub x: T,
}

#[cfg(not(feature = "serialize"))]
#[derive(Copy, Clone, Default, Hash, Eq, PartialEq)]
pub struct Vec1<T: VecItem> {
    pub x: T,
}

impl<T: VecItem> Vec1<T> {
    /// Creates a new Vec1 from a single component
    pub fn new(x: T) -> Self { Self { x, ..Default::default() } }

    /// Returns the elements of the vector as an array
    pub fn elements(&self) -> [T; 1] { [self.x] }

    /// Apply an operation to all elements of this vector, returning the result
    pub fn map<U: VecItem, F: Fn(T) -> U>(&self, f: F) -> Vec1<U> {
        Vec1 {
            x: f(self.x),
        }
    }

    pub fn convert_to<U: VecItem + From<T>>(&self) -> Vec1<U> {
        Vec1 {
            x: U::from(self.x),
        }
    }
}

impl<T: VecItem> Vector for Vec1<T> {
    type Item = T;
}

// Debug and Display traits

impl<T: VecItem + fmt::Debug> fmt::Debug for Vec1<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(x: {:?})", self.x)
    }
}

impl<T: VecItem + fmt::Display> fmt::Display for Vec1<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({})", self.x)
    }
}

// From traits

impl<T: VecItem> From<T> for Vec1<T> {
    fn from(item: T) -> Self { Self { x: item, ..Default::default() } }
}

impl<T: VecItem> From<[T; 1]> for Vec1<T> {
    fn from(arr: [T; 1]) -> Self { Self { x: arr[0], ..Default::default() } }
}

// Op traits

impl<T> Add for Vec1<T> where T: VecItem + Add, T::Output: VecItem + Add {
    type Output = Vec1<T::Output>;
    fn add(self, other: Self) -> Vec1<T::Output> {
        Vec1 {
            x: self.x + other.x,
            ..Default::default()
        }
    }
}

impl<T> Sub for Vec1<T> where T: VecItem + Sub, T::Output: VecItem + Sub {
    type Output = Vec1<T::Output>;
    fn sub(self, other: Self) -> Vec1<T::Output> {
        Vec1 {
            x: self.x - other.x,
            ..Default::default()
        }
    }
}

impl<T> Mul for Vec1<T> where T: VecItem + Mul, T::Output: VecItem + Mul {
    type Output = Vec1<T::Output>;
    fn mul(self, other: Self) -> Vec1<T::Output> {
        Vec1 {
            x: self.x * other.x,
            ..Default::default()
        }
    }
}

impl<T> Div for Vec1<T> where T: VecItem + Div, T::Output: VecItem + Div {
    type Output = Vec1<T::Output>;
    fn div(self, other: Self) -> Vec1<T::Output> {
        Vec1 {
            x: self.x / other.x,
            ..Default::default()
        }
    }
}

// Op primitive traits

impl<T> Add<T> for Vec1<T> where T: VecItem + Add, T::Output: VecItem + Add {
    type Output = Vec1<T::Output>;
    fn add(self, other: T) -> Vec1<T::Output> {
        Vec1 {
            x: self.x + other,
            ..Default::default()
        }
    }
}

impl<T> Sub<T> for Vec1<T> where T: VecItem + Sub, T::Output: VecItem + Sub {
    type Output = Vec1<T::Output>;
    fn sub(self, other: T) -> Vec1<T::Output> {
        Vec1 {
            x: self.x - other,
            ..Default::default()
        }
    }
}

impl<T> Mul<T> for Vec1<T> where T: VecItem + Mul, T::Output: VecItem + Mul {
    type Output = Vec1<T::Output>;
    fn mul(self, other: T) -> Vec1<T::Output> {
        Vec1 {
            x: self.x * other,
            ..Default::default()
        }
    }
}

impl<T> Div<T> for Vec1<T> where T: VecItem + Div, T::Output: VecItem + Div {
    type Output = Vec1<T::Output>;
    fn div(self, other: T) -> Vec1<T::Output> {
        Vec1 {
            x: self.x / other,
            ..Default::default()
        }
    }
}

// Assign operators

impl<T> AddAssign for Vec1<T> where T: VecItem + Add<Output=T> {
    fn add_assign(&mut self, other: Self) {
        *self = Vec1 {
            x: self.x + other.x,
            ..Default::default()
        }
    }
}

impl<T> SubAssign for Vec1<T> where T: VecItem + Sub<Output=T> {
    fn sub_assign(&mut self, other: Self) {
        *self = Vec1 {
            x: self.x - other.x,
            ..Default::default()
        }
    }
}

impl<T> MulAssign for Vec1<T> where T: VecItem + Mul<Output=T> {
    fn mul_assign(&mut self, other: Self) {
        *self = Vec1 {
            x: self.x * other.x,
            ..Default::default()
        }
    }
}

impl<T> DivAssign for Vec1<T> where T: VecItem + Div<Output=T> {
    fn div_assign(&mut self, other: Self) {
        *self = Vec1 {
            x: self.x / other.x,
            ..Default::default()
        }
    }
}

// Assign primitive operators

impl<T> AddAssign<T> for Vec1<T> where T: VecItem + Add<Output=T> {
    fn add_assign(&mut self, other: T) {
        *self = Vec1 {
            x: self.x + other,
            ..Default::default()
        }
    }
}

impl<T> SubAssign<T> for Vec1<T> where T: VecItem + Sub<Output=T> {
    fn sub_assign(&mut self, other: T) {
        *self = Vec1 {
            x: self.x - other,
            ..Default::default()
        }
    }
}

impl<T> MulAssign<T> for Vec1<T> where T: VecItem + Mul<Output=T> {
    fn mul_assign(&mut self, other: T) {
        *self = Vec1 {
            x: self.x * other,
            ..Default::default()
        }
    }
}

impl<T> DivAssign<T> for Vec1<T> where T: VecItem + Div<Output=T> {
    fn div_assign(&mut self, other: T) {
        *self = Vec1 {
            x: self.x / other,
            ..Default::default()
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

    fn normalize(&self) -> Self {
        Vec1::new(
            self.x,
        )
    }
}
