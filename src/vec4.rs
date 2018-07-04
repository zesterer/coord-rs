//! Functionality pertaining to `Vec4`

use core::ops::{Add, Sub, Mul, Div, AddAssign, SubAssign, MulAssign, DivAssign};
use core::fmt;

#[allow(unused_imports)]
use num::{Num, Integer, Unsigned, Signed, Float};

#[allow(unused_imports)]
use super::{Vector, VecItem, VecNum, VecInt, VecUnsigned, VecSigned, VecFloat};

#[cfg(feature = "serialize")]
#[derive(Copy, Clone, Default, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct Vec4<T: VecItem> {
    pub x: T,
    pub y: T,
    pub z: T,
    pub w: T,
}

#[cfg(not(feature = "serialize"))]
#[derive(Copy, Clone, Default, Hash, Eq, PartialEq)]
pub struct Vec4<T: VecItem> {
    pub x: T,
    pub y: T,
    pub z: T,
    pub w: T,
}

impl<T: VecItem> Vec4<T> {
    /// Creates a new Vec3 from multiple components
    pub fn new(x: T, y: T, z: T, w: T) -> Self { Self { x, y, z, w, ..Default::default() } }

    /// Returns the elements of the vector as an array
    pub fn elements(&self) -> [T; 4] { [self.x, self.y, self.z, self.w] }

    /// Apply an operation to all elements of this vector, returning the result
    pub fn map<U: VecItem, F: Fn(T) -> U>(&self, f: F) -> Vec4<U> {
        Vec4 {
            x: f(self.x),
            y: f(self.y),
            z: f(self.z),
            w: f(self.w),
        }
    }

    pub fn convert_to<U: VecItem + From<T>>(&self) -> Vec4<U> {
        Vec4 {
            x: U::from(self.x),
            y: U::from(self.y),
            z: U::from(self.z),
            w: U::from(self.w),
        }
    }
}

impl<T: VecItem> Vector for Vec4<T> {
    type Item = T;
}

// Debug and Display traits

impl<T: VecItem + fmt::Debug> fmt::Debug for Vec4<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(x: {:?}, y: {:?}, z: {:?}, w: {:?})", self.x, self.y, self.z, self.w)
    }
}

impl<T: VecItem + fmt::Display> fmt::Display for Vec4<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {}, {}, {})", self.x, self.y, self.z, self.w)
    }
}

// From traits

impl<T: VecItem> From<[T; 4]> for Vec4<T> {
    fn from(arr: [T; 4]) -> Self { Self { x: arr[0], y: arr[1], z: arr[2], w: arr[3], ..Default::default() } }
}

impl<T: VecItem> From<(T, T, T, T)> for Vec4<T> {
    fn from(tup: (T, T, T, T)) -> Self { Self { x: tup.0, y: tup.1, z: tup.2, w: tup.3, ..Default::default() } }
}

// Op traits

impl<T> Add for Vec4<T> where T: VecItem + Add, T::Output: VecItem + Add {
    type Output = Vec4<T::Output>;
    fn add(self, other: Self) -> Vec4<T::Output> {
        Vec4 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w + other.w,
            ..Default::default()
        }
    }
}

impl<T> Sub for Vec4<T> where T: VecItem + Sub, T::Output: VecItem + Sub {
    type Output = Vec4<T::Output>;
    fn sub(self, other: Self) -> Vec4<T::Output> {
        Vec4 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
            w: self.w - other.w,
            ..Default::default()
        }
    }
}

impl<T> Mul for Vec4<T> where T: VecItem + Mul, T::Output: VecItem + Mul {
    type Output = Vec4<T::Output>;
    fn mul(self, other: Self) -> Vec4<T::Output> {
        Vec4 {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
            w: self.w * other.w,
            ..Default::default()
        }
    }
}

impl<T> Div for Vec4<T> where T: VecItem + Div, T::Output: VecItem + Div {
    type Output = Vec4<T::Output>;
    fn div(self, other: Self) -> Vec4<T::Output> {
        Vec4 {
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z,
            w: self.w / other.w,
            ..Default::default()
        }
    }
}

// Op primitive traits

impl<T> Add<T> for Vec4<T> where T: VecItem + Add, T::Output: VecItem + Add {
    type Output = Vec4<T::Output>;
    fn add(self, other: T) -> Vec4<T::Output> {
        Vec4 {
            x: self.x + other,
            y: self.y + other,
            z: self.z + other,
            w: self.w + other,
            ..Default::default()
        }
    }
}

impl<T> Sub<T> for Vec4<T> where T: VecItem + Sub, T::Output: VecItem + Sub {
    type Output = Vec4<T::Output>;
    fn sub(self, other: T) -> Vec4<T::Output> {
        Vec4 {
            x: self.x - other,
            y: self.y - other,
            z: self.z - other,
            w: self.w - other,
            ..Default::default()
        }
    }
}

impl<T> Mul<T> for Vec4<T> where T: VecItem + Mul, T::Output: VecItem + Mul {
    type Output = Vec4<T::Output>;
    fn mul(self, other: T) -> Vec4<T::Output> {
        Vec4 {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
            w: self.w * other,
            ..Default::default()
        }
    }
}

impl<T> Div<T> for Vec4<T> where T: VecItem + Div, T::Output: VecItem + Div {
    type Output = Vec4<T::Output>;
    fn div(self, other: T) -> Vec4<T::Output> {
        Vec4 {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
            w: self.w / other,
            ..Default::default()
        }
    }
}

// Assign operators

impl<T> AddAssign for Vec4<T> where T: VecItem + Add<Output=T> {
    fn add_assign(&mut self, other: Self) {
        *self = Vec4 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w + other.w,
            ..Default::default()
        }
    }
}

impl<T> SubAssign for Vec4<T> where T: VecItem + Sub<Output=T> {
    fn sub_assign(&mut self, other: Self) {
        *self = Vec4 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
            w: self.w - other.w,
            ..Default::default()
        }
    }
}

impl<T> MulAssign for Vec4<T> where T: VecItem + Mul<Output=T> {
    fn mul_assign(&mut self, other: Self) {
        *self = Vec4 {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
            w: self.w * other.w,
            ..Default::default()
        }
    }
}

impl<T> DivAssign for Vec4<T> where T: VecItem + Div<Output=T> {
    fn div_assign(&mut self, other: Self) {
        *self = Vec4 {
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z,
            w: self.w / other.w,
            ..Default::default()
        }
    }
}

// Assign primitive operators

impl<T> AddAssign<T> for Vec4<T> where T: VecItem + Add<Output=T> {
    fn add_assign(&mut self, other: T) {
        *self = Vec4 {
            x: self.x + other,
            y: self.y + other,
            z: self.z + other,
            w: self.w + other,
            ..Default::default()
        }
    }
}

impl<T> SubAssign<T> for Vec4<T> where T: VecItem + Sub<Output=T> {
    fn sub_assign(&mut self, other: T) {
        *self = Vec4 {
            x: self.x - other,
            y: self.y - other,
            z: self.z - other,
            w: self.w - other,
            ..Default::default()
        }
    }
}

impl<T> MulAssign<T> for Vec4<T> where T: VecItem + Mul<Output=T> {
    fn mul_assign(&mut self, other: T) {
        *self = Vec4 {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
            w: self.w * other,
            ..Default::default()
        }
    }
}

impl<T> DivAssign<T> for Vec4<T> where T: VecItem + Div<Output=T> {
    fn div_assign(&mut self, other: T) {
        *self = Vec4 {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
            w: self.w / other,
            ..Default::default()
        }
    }
}

// VecNum traits

impl<T> VecNum for Vec4<T> where T: VecItem + Num {
    fn sum(&self) -> Self::Item {
        self.x + self.y + self.z + self.w
    }

    fn product(&self) -> Self::Item {
        self.x * self.y * self.z * self.w
    }
}

// VecInt traits

impl<T> VecInt for Vec4<T> where T: VecItem + Integer {
    fn div_euc(&self, other: Self) -> Self {
        Self {
            x: self.x.div_floor(&other.x),
            y: self.y.div_floor(&other.y),
            z: self.z.div_floor(&other.z),
            w: self.w.div_floor(&other.w),
        }
    }
}

// VecSigned traits

impl<T> VecSigned for Vec4<T> where T: VecItem + Signed {
    fn snake_length(&self) -> Self::Item {
        self.x.abs() + self.y.abs() + self.z.abs() + self.w.abs()
    }
}

// VecFloat traits

impl<T> VecFloat for Vec4<T> where T: VecItem + Float {
    fn length(&self) -> Self::Item {
        (self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w).sqrt()
    }

    fn normalize(&self) -> Self {
        let len = self.length();
        Vec4::new(
            self.x / len,
            self.y / len,
            self.z / len,
            self.w / len,
        )
    }
}
