//! Functionality pertaining to `Vec4`

use core::ops::{Add, Sub, Mul, Div, AddAssign, SubAssign, MulAssign, DivAssign};
use core::marker::PhantomData;
use core::fmt;
use num::{Num, Integer, Unsigned, Signed, Float};
use super::{Vector, VecItem, VecNum, VecInt, VecUnsigned, VecSigned, VecFloat};

#[derive(Copy, Clone)]
pub struct Vec4<'a, T: 'a + VecItem<'a>> {
    pub x: T,
    pub y: T,
    pub z: T,
    pub w: T,
    phantom: PhantomData<&'a T>,
}

impl<'a, T: VecItem<'a>> Vec4<'a, T> {
    pub fn new(x: T, y: T, z: T, w: T) -> Self { Self { x, y, z, w, phantom: PhantomData } }
}

impl<'a, T: VecItem<'a>> Vector<'a> for Vec4<'a, T> {
    type Item = T;
}

// Debug and Display traits

impl<'a, T: VecItem<'a> + fmt::Debug> fmt::Debug for Vec4<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(x: {:?}, y: {:?}, z: {:?}, w: {:?})", self.x, self.y, self.z, self.w)
    }
}

impl<'a, T: VecItem<'a> + fmt::Display> fmt::Display for Vec4<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {}, {}, {})", self.x, self.y, self.z, self.w)
    }
}

// From traits

impl<'a, T: VecItem<'a>> From<[T; 4]> for Vec4<'a, T> {
    fn from(arr: [T; 4]) -> Self { Self { x: arr[0], y: arr[1], z: arr[2], w: arr[3], phantom: PhantomData } }
}

impl<'a, T: VecItem<'a>> From<(T, T, T, T)> for Vec4<'a, T> {
    fn from(tup: (T, T, T, T)) -> Self { Self { x: tup.0, y: tup.1, z: tup.2, w: tup.3, phantom: PhantomData } }
}

// Op traits

impl<'a, T> Add for Vec4<'a, T> where T: VecItem<'a> + Add, T::Output: VecItem<'a> + Add {
    type Output = Vec4<'a, T::Output>;
    fn add(self, other: Self) -> Vec4<'a, T::Output> {
        Vec4 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w + other.w,
            phantom: PhantomData,
        }
    }
}

impl<'a, T> Sub for Vec4<'a, T> where T: VecItem<'a> + Sub, T::Output: VecItem<'a> + Sub {
    type Output = Vec4<'a, T::Output>;
    fn sub(self, other: Self) -> Vec4<'a, T::Output> {
        Vec4 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
            w: self.w - other.w,
            phantom: PhantomData,
        }
    }
}

impl<'a, T> Mul for Vec4<'a, T> where T: VecItem<'a> + Mul, T::Output: VecItem<'a> + Mul {
    type Output = Vec4<'a, T::Output>;
    fn mul(self, other: Self) -> Vec4<'a, T::Output> {
        Vec4 {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
            w: self.w * other.w,
            phantom: PhantomData,
        }
    }
}

impl<'a, T> Div for Vec4<'a, T> where T: VecItem<'a> + Div, T::Output: VecItem<'a> + Div {
    type Output = Vec4<'a, T::Output>;
    fn div(self, other: Self) -> Vec4<'a, T::Output> {
        Vec4 {
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z,
            w: self.w / other.w,
            phantom: PhantomData,
        }
    }
}

// Op primitive traits

impl<'a, T> Add<T> for Vec4<'a, T> where T: VecItem<'a> + Add, T::Output: VecItem<'a> + Add {
    type Output = Vec4<'a, T::Output>;
    fn add(self, other: T) -> Vec4<'a, T::Output> {
        Vec4 {
            x: self.x + other,
            y: self.y + other,
            z: self.z + other,
            w: self.w + other,
            phantom: PhantomData,
        }
    }
}

impl<'a, T> Sub<T> for Vec4<'a, T> where T: VecItem<'a> + Sub, T::Output: VecItem<'a> + Sub {
    type Output = Vec4<'a, T::Output>;
    fn sub(self, other: T) -> Vec4<'a, T::Output> {
        Vec4 {
            x: self.x - other,
            y: self.y - other,
            z: self.z - other,
            w: self.w - other,
            phantom: PhantomData,
        }
    }
}

impl<'a, T> Mul<T> for Vec4<'a, T> where T: VecItem<'a> + Mul, T::Output: VecItem<'a> + Mul {
    type Output = Vec4<'a, T::Output>;
    fn mul(self, other: T) -> Vec4<'a, T::Output> {
        Vec4 {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
            w: self.w * other,
            phantom: PhantomData,
        }
    }
}

impl<'a, T> Div<T> for Vec4<'a, T> where T: VecItem<'a> + Div, T::Output: VecItem<'a> + Div {
    type Output = Vec4<'a, T::Output>;
    fn div(self, other: T) -> Vec4<'a, T::Output> {
        Vec4 {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
            w: self.w / other,
            phantom: PhantomData,
        }
    }
}

// Assign operators

impl<'a, T> AddAssign for Vec4<'a, T> where T: VecItem<'a> + Add<Output=T> {
    fn add_assign(&mut self, other: Self) {
        *self = Vec4 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w + other.w,
            phantom: PhantomData,
        }
    }
}

impl<'a, T> SubAssign for Vec4<'a, T> where T: VecItem<'a> + Sub<Output=T> {
    fn sub_assign(&mut self, other: Self) {
        *self = Vec4 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
            w: self.w - other.w,
            phantom: PhantomData,
        }
    }
}

impl<'a, T> MulAssign for Vec4<'a, T> where T: VecItem<'a> + Mul<Output=T> {
    fn mul_assign(&mut self, other: Self) {
        *self = Vec4 {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
            w: self.w * other.w,
            phantom: PhantomData,
        }
    }
}

impl<'a, T> DivAssign for Vec4<'a, T> where T: VecItem<'a> + Div<Output=T> {
    fn div_assign(&mut self, other: Self) {
        *self = Vec4 {
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z,
            w: self.w / other.w,
            phantom: PhantomData,
        }
    }
}

// Assign primitive operators

impl<'a, T> AddAssign<T> for Vec4<'a, T> where T: VecItem<'a> + Add<Output=T> {
    fn add_assign(&mut self, other: T) {
        *self = Vec4 {
            x: self.x + other,
            y: self.y + other,
            z: self.z + other,
            w: self.w + other,
            phantom: PhantomData,
        }
    }
}

impl<'a, T> SubAssign<T> for Vec4<'a, T> where T: VecItem<'a> + Sub<Output=T> {
    fn sub_assign(&mut self, other: T) {
        *self = Vec4 {
            x: self.x - other,
            y: self.y - other,
            z: self.z - other,
            w: self.w - other,
            phantom: PhantomData,
        }
    }
}

impl<'a, T> MulAssign<T> for Vec4<'a, T> where T: VecItem<'a> + Mul<Output=T> {
    fn mul_assign(&mut self, other: T) {
        *self = Vec4 {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
            w: self.w * other,
            phantom: PhantomData,
        }
    }
}

impl<'a, T> DivAssign<T> for Vec4<'a, T> where T: VecItem<'a> + Div<Output=T> {
    fn div_assign(&mut self, other: T) {
        *self = Vec4 {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
            w: self.w / other,
            phantom: PhantomData,
        }
    }
}

// VecNum traits

impl<'a, T> VecNum<'a> for Vec4<'a, T> where T: VecItem<'a> + Num {
    fn sum(&self) -> Self::Item {
        self.x + self.y + self.z + self.w
    }

    fn product(&self) -> Self::Item {
        self.x * self.y * self.z * self.w
    }
}

// VecSigned traits

impl<'a, T> VecSigned<'a> for Vec4<'a, T> where T: VecItem<'a> + Signed {
    fn snake_length(&self) -> Self::Item {
        self.x.abs() + self.y.abs() + self.z.abs() + self.w.abs()
    }
}

// VecFloat traits

impl<'a, T> VecFloat<'a> for Vec4<'a, T> where T: VecItem<'a> + Float {
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
