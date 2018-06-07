use core::ops::{Add, Sub, Mul, Div, AddAssign, SubAssign, MulAssign, DivAssign};
use num::{Num, Integer, Unsigned, Signed, Float};
use super::{Vec, VecItem, VecNum, VecInt, VecUnsigned, VecSigned, VecFloat};

#[derive(Copy, Clone, Debug)]
pub struct Vec3<T: VecItem> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T: VecItem> Vec3<T> {
    pub fn new(x: T, y: T, z: T) -> Self { Self { x, y, z } }
}

impl<T: VecItem> Vec for Vec3<T> {
    type Item = T;
}

// From traits

impl<T: VecItem> From<[T; 3]> for Vec3<T> {
    fn from(arr: [T; 3]) -> Self { Self { x: arr[0], y: arr[1], z: arr[2] } }
}

impl<T: VecItem> From<(T, T, T)> for Vec3<T> {
    fn from(tup: (T, T, T)) -> Self { Self { x: tup.0, y: tup.1, z: tup.2 } }
}

// Op traits

impl<T> Add for Vec3<T> where T: VecItem + Add, T::Output: VecItem + Add {
    type Output = Vec3<T::Output>;
    fn add(self, other: Self) -> Vec3<T::Output> {
        Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
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
        }
    }
}

impl<T> SubAssign for Vec3<T> where T: VecItem + Sub<Output=T> {
    fn sub_assign(&mut self, other: Self) {
        *self = Vec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl<T> MulAssign for Vec3<T> where T: VecItem + Mul<Output=T> {
    fn mul_assign(&mut self, other: Self) {
        *self = Vec3 {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}

impl<T> DivAssign for Vec3<T> where T: VecItem + Div<Output=T> {
    fn div_assign(&mut self, other: Self) {
        *self = Vec3 {
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z,
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
}
