//! Functionality pertaining to `Vec3`

use core::ops::{Add, Sub, Mul, Div, AddAssign, SubAssign, MulAssign, DivAssign};
use core::fmt;

#[allow(unused_imports)]
use num::{Num, Integer, Unsigned, Signed, Float};

#[allow(unused_imports)]
use super::{Vector, VecItem, VecNum, VecInt, VecUnsigned, VecSigned, VecFloat};

#[cfg(feature = "serialize")]
#[derive(Copy, Clone, Default, Serialize)]
pub struct Vec3<T: VecItem> {
    pub x: T,
    pub y: T,
    pub z: T,
}

#[cfg(not(feature = "serialize"))]
#[derive(Copy, Clone, Default)]
pub struct Vec3<T: VecItem> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T: VecItem> Vec3<T> {
    pub fn new(x: T, y: T, z: T) -> Self { Self { x, y, z, ..Default::default() } }
}

impl<T: VecItem> Vector for Vec3<T> {
    type Item = T;
}

// Debug and Display traits

impl<T: VecItem + fmt::Debug> fmt::Debug for Vec3<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(x: {:?}, y: {:?}, z: {:?})", self.x, self.y, self.z)
    }
}

impl<T: VecItem + fmt::Display> fmt::Display for Vec3<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

// From traits

impl<T: VecItem> From<[T; 3]> for Vec3<T> {
    fn from(arr: [T; 3]) -> Self { Self { x: arr[0], y: arr[1], z: arr[2], ..Default::default() } }
}

impl<T: VecItem> From<(T, T, T)> for Vec3<T> {
    fn from(tup: (T, T, T)) -> Self { Self { x: tup.0, y: tup.1, z: tup.2, ..Default::default() } }
}

// Op traits

impl<T> Add for Vec3<T> where T: VecItem + Add, T::Output: VecItem + Add {
    type Output = Vec3<T::Output>;
    fn add(self, other: Self) -> Vec3<T::Output> {
        Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            ..Default::default()
        }
    }
}

impl<T> Sub for Vec3<T> where T: VecItem + Sub, T::Output: VecItem + Sub {
    type Output = Vec3<T::Output>;
    fn sub(self, other: Self) -> Vec3<T::Output> {
        Vec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
            ..Default::default()
        }
    }
}

impl<T> Mul for Vec3<T> where T: VecItem + Mul, T::Output: VecItem + Mul {
    type Output = Vec3<T::Output>;
    fn mul(self, other: Self) -> Vec3<T::Output> {
        Vec3 {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
            ..Default::default()
        }
    }
}

impl<T> Div for Vec3<T> where T: VecItem + Div, T::Output: VecItem + Div {
    type Output = Vec3<T::Output>;
    fn div(self, other: Self) -> Vec3<T::Output> {
        Vec3 {
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z,
            ..Default::default()
        }
    }
}

// Op primitive traits

impl<T> Add<T> for Vec3<T> where T: VecItem + Add, T::Output: VecItem + Add {
    type Output = Vec3<T::Output>;
    fn add(self, other: T) -> Vec3<T::Output> {
        Vec3 {
            x: self.x + other,
            y: self.y + other,
            z: self.z + other,
            ..Default::default()
        }
    }
}

impl<T> Sub<T> for Vec3<T> where T: VecItem + Sub, T::Output: VecItem + Sub {
    type Output = Vec3<T::Output>;
    fn sub(self, other: T) -> Vec3<T::Output> {
        Vec3 {
            x: self.x - other,
            y: self.y - other,
            z: self.z - other,
            ..Default::default()
        }
    }
}

impl<T> Mul<T> for Vec3<T> where T: VecItem + Mul, T::Output: VecItem + Mul {
    type Output = Vec3<T::Output>;
    fn mul(self, other: T) -> Vec3<T::Output> {
        Vec3 {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
            ..Default::default()
        }
    }
}

impl<T> Div<T> for Vec3<T> where T: VecItem + Div, T::Output: VecItem + Div {
    type Output = Vec3<T::Output>;
    fn div(self, other: T) -> Vec3<T::Output> {
        Vec3 {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
            ..Default::default()
        }
    }
}

// Assign operators

impl<T> AddAssign for Vec3<T> where T: VecItem + Add<Output=T> {
    fn add_assign(&mut self, other: Self) {
        *self = Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            ..Default::default()
        }
    }
}

impl<T> SubAssign for Vec3<T> where T: VecItem + Sub<Output=T> {
    fn sub_assign(&mut self, other: Self) {
        *self = Vec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
            ..Default::default()
        }
    }
}

impl<T> MulAssign for Vec3<T> where T: VecItem + Mul<Output=T> {
    fn mul_assign(&mut self, other: Self) {
        *self = Vec3 {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
            ..Default::default()
        }
    }
}

impl<T> DivAssign for Vec3<T> where T: VecItem + Div<Output=T> {
    fn div_assign(&mut self, other: Self) {
        *self = Vec3 {
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z,
            ..Default::default()
        }
    }
}

// Assign primitive operators

impl<T> AddAssign<T> for Vec3<T> where T: VecItem + Add<Output=T> {
    fn add_assign(&mut self, other: T) {
        *self = Vec3 {
            x: self.x + other,
            y: self.y + other,
            z: self.z + other,
            ..Default::default()
        }
    }
}

impl<T> SubAssign<T> for Vec3<T> where T: VecItem + Sub<Output=T> {
    fn sub_assign(&mut self, other: T) {
        *self = Vec3 {
            x: self.x - other,
            y: self.y - other,
            z: self.z - other,
            ..Default::default()
        }
    }
}

impl<T> MulAssign<T> for Vec3<T> where T: VecItem + Mul<Output=T> {
    fn mul_assign(&mut self, other: T) {
        *self = Vec3 {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
            ..Default::default()
        }
    }
}

impl<T> DivAssign<T> for Vec3<T> where T: VecItem + Div<Output=T> {
    fn div_assign(&mut self, other: T) {
        *self = Vec3 {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
            ..Default::default()
        }
    }
}

// VecNum traits

impl<T> VecNum for Vec3<T> where T: VecItem + Num {
    fn sum(&self) -> Self::Item {
        self.x + self.y + self.z
    }

    fn product(&self) -> Self::Item {
        self.x * self.y * self.z
    }
}

// VecSigned traits

impl<T> VecSigned for Vec3<T> where T: VecItem + Signed {
    fn snake_length(&self) -> Self::Item {
        self.x.abs() + self.y.abs() + self.z.abs()
    }
}

// VecFloat traits

impl<T> VecFloat for Vec3<T> where T: VecItem + Float {
    fn length(&self) -> Self::Item {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    fn normalize(&self) -> Self {
        let len = self.length();
        Vec3::new(
            self.x / len,
            self.y / len,
            self.z / len,
        )
    }
}
