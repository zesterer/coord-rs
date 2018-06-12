//! Functionality pertaining to `Vec3`

use core::ops::{Add, Sub, Mul, Div, AddAssign, SubAssign, MulAssign, DivAssign};
use core::marker::PhantomData;
use core::fmt;

#[allow(unused_imports)]
use num::{Num, Integer, Unsigned, Signed, Float};

#[allow(unused_imports)]
use super::{Vector, VecItem, VecNum, VecInt, VecUnsigned, VecSigned, VecFloat};

#[cfg(feature = "serialize")]
#[derive(Copy, Clone, Default, Serialize)]
pub struct Vec3<'a, T: 'a + VecItem<'a>> {
    pub x: T,
    pub y: T,
    pub z: T,
    phantom: PhantomData<&'a T>,
}

#[cfg(not(feature = "serialize"))]
#[derive(Copy, Clone, Default)]
pub struct Vec3<'a, T: 'a + VecItem<'a>> {
    pub x: T,
    pub y: T,
    pub z: T,
    phantom: PhantomData<&'a T>,
}

impl<'a, T: VecItem<'a>> Vec3<'a, T> {
    pub fn new(x: T, y: T, z: T) -> Self { Self { x, y, z, ..Default::default() } }
}

impl<'a, T: VecItem<'a>> Vector<'a> for Vec3<'a, T> {
    type Item = T;
}

// Debug and Display traits

impl<'a, T: VecItem<'a> + fmt::Debug> fmt::Debug for Vec3<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(x: {:?}, y: {:?}, z: {:?})", self.x, self.y, self.z)
    }
}

impl<'a, T: VecItem<'a> + fmt::Display> fmt::Display for Vec3<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

// From traits

impl<'a, T: VecItem<'a>> From<[T; 3]> for Vec3<'a, T> {
    fn from(arr: [T; 3]) -> Self { Self { x: arr[0], y: arr[1], z: arr[2], ..Default::default() } }
}

impl<'a, T: VecItem<'a>> From<(T, T, T)> for Vec3<'a, T> {
    fn from(tup: (T, T, T)) -> Self { Self { x: tup.0, y: tup.1, z: tup.2, ..Default::default() } }
}

// Op traits

impl<'a, T> Add for Vec3<'a, T> where T: VecItem<'a> + Add, T::Output: VecItem<'a> + Add {
    type Output = Vec3<'a, T::Output>;
    fn add(self, other: Self) -> Vec3<'a, T::Output> {
        Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            ..Default::default()
        }
    }
}

impl<'a, T> Sub for Vec3<'a, T> where T: VecItem<'a> + Sub, T::Output: VecItem<'a> + Sub {
    type Output = Vec3<'a, T::Output>;
    fn sub(self, other: Self) -> Vec3<'a, T::Output> {
        Vec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
            ..Default::default()
        }
    }
}

impl<'a, T> Mul for Vec3<'a, T> where T: VecItem<'a> + Mul, T::Output: VecItem<'a> + Mul {
    type Output = Vec3<'a, T::Output>;
    fn mul(self, other: Self) -> Vec3<'a, T::Output> {
        Vec3 {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
            ..Default::default()
        }
    }
}

impl<'a, T> Div for Vec3<'a, T> where T: VecItem<'a> + Div, T::Output: VecItem<'a> + Div {
    type Output = Vec3<'a, T::Output>;
    fn div(self, other: Self) -> Vec3<'a, T::Output> {
        Vec3 {
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z,
            ..Default::default()
        }
    }
}

// Op primitive traits

impl<'a, T> Add<T> for Vec3<'a, T> where T: VecItem<'a> + Add, T::Output: VecItem<'a> + Add {
    type Output = Vec3<'a, T::Output>;
    fn add(self, other: T) -> Vec3<'a, T::Output> {
        Vec3 {
            x: self.x + other,
            y: self.y + other,
            z: self.z + other,
            ..Default::default()
        }
    }
}

impl<'a, T> Sub<T> for Vec3<'a, T> where T: VecItem<'a> + Sub, T::Output: VecItem<'a> + Sub {
    type Output = Vec3<'a, T::Output>;
    fn sub(self, other: T) -> Vec3<'a, T::Output> {
        Vec3 {
            x: self.x - other,
            y: self.y - other,
            z: self.z - other,
            ..Default::default()
        }
    }
}

impl<'a, T> Mul<T> for Vec3<'a, T> where T: VecItem<'a> + Mul, T::Output: VecItem<'a> + Mul {
    type Output = Vec3<'a, T::Output>;
    fn mul(self, other: T) -> Vec3<'a, T::Output> {
        Vec3 {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
            ..Default::default()
        }
    }
}

impl<'a, T> Div<T> for Vec3<'a, T> where T: VecItem<'a> + Div, T::Output: VecItem<'a> + Div {
    type Output = Vec3<'a, T::Output>;
    fn div(self, other: T) -> Vec3<'a, T::Output> {
        Vec3 {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
            ..Default::default()
        }
    }
}

// Assign operators

impl<'a, T> AddAssign for Vec3<'a, T> where T: VecItem<'a> + Add<Output=T> {
    fn add_assign(&mut self, other: Self) {
        *self = Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            ..Default::default()
        }
    }
}

impl<'a, T> SubAssign for Vec3<'a, T> where T: VecItem<'a> + Sub<Output=T> {
    fn sub_assign(&mut self, other: Self) {
        *self = Vec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
            ..Default::default()
        }
    }
}

impl<'a, T> MulAssign for Vec3<'a, T> where T: VecItem<'a> + Mul<Output=T> {
    fn mul_assign(&mut self, other: Self) {
        *self = Vec3 {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
            ..Default::default()
        }
    }
}

impl<'a, T> DivAssign for Vec3<'a, T> where T: VecItem<'a> + Div<Output=T> {
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

impl<'a, T> AddAssign<T> for Vec3<'a, T> where T: VecItem<'a> + Add<Output=T> {
    fn add_assign(&mut self, other: T) {
        *self = Vec3 {
            x: self.x + other,
            y: self.y + other,
            z: self.z + other,
            ..Default::default()
        }
    }
}

impl<'a, T> SubAssign<T> for Vec3<'a, T> where T: VecItem<'a> + Sub<Output=T> {
    fn sub_assign(&mut self, other: T) {
        *self = Vec3 {
            x: self.x - other,
            y: self.y - other,
            z: self.z - other,
            ..Default::default()
        }
    }
}

impl<'a, T> MulAssign<T> for Vec3<'a, T> where T: VecItem<'a> + Mul<Output=T> {
    fn mul_assign(&mut self, other: T) {
        *self = Vec3 {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
            ..Default::default()
        }
    }
}

impl<'a, T> DivAssign<T> for Vec3<'a, T> where T: VecItem<'a> + Div<Output=T> {
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

impl<'a, T> VecNum<'a> for Vec3<'a, T> where T: VecItem<'a> + Num {
    fn sum(&self) -> Self::Item {
        self.x + self.y + self.z
    }

    fn product(&self) -> Self::Item {
        self.x * self.y * self.z
    }
}

// VecSigned traits

impl<'a, T> VecSigned<'a> for Vec3<'a, T> where T: VecItem<'a> + Signed {
    fn snake_length(&self) -> Self::Item {
        self.x.abs() + self.y.abs() + self.z.abs()
    }
}

// VecFloat traits

impl<'a, T> VecFloat<'a> for Vec3<'a, T> where T: VecItem<'a> + Float {
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
