use core::ops::{Add, Sub, Mul, Div, AddAssign, SubAssign, MulAssign, DivAssign};
use super::{Vec, VecItem, VecNum, VecInt, VecUnsigned, VecSigned, VecFloat, VecItemNum, VecItemInt, VecItemUnsigned, VecItemSigned, VecItemFloat};

#[derive(Copy, Clone, Debug)]
pub struct Vec4<T: VecItem> {
    pub x: T,
    pub y: T,
    pub z: T,
    pub w: T,
}

impl<T: VecItem> Vec4<T> {
    pub fn new(x: T, y: T, z: T, w: T) -> Self { Self { x, y, z, w } }
}

impl<T: VecItem> Vec for Vec4<T> {
    type Item = T;
}

// From traits

impl<T: VecItem> From<[T; 4]> for Vec4<T> {
    fn from(arr: [T; 4]) -> Self { Self { x: arr[0], y: arr[1], z: arr[2], w: arr[3] } }
}

impl<T: VecItem> From<(T, T, T, T)> for Vec4<T> {
    fn from(tup: (T, T, T, T)) -> Self { Self { x: tup.0, y: tup.1, z: tup.2, w: tup.3 } }
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
        }
    }
}

// VecNum traits

impl<T> VecNum for Vec4<T> where T: VecItemNum {
    fn sum(&self) -> Self::Item {
        self.x + self.y + self.z + self.w
    }

    fn product(&self) -> Self::Item {
        self.x * self.y * self.z * self.w
    }
}

// VecSigned traits

impl<T> VecSigned for Vec4<T> where T: VecItemSigned {
    fn snake_length(&self) -> Self::Item {
        self.x.abs() + self.y.abs() + self.z.abs() + self.w.abs()
    }
}

// VecFloat traits

impl<T> VecFloat for Vec4<T> where T: VecItemFloat {
    fn length(&self) -> Self::Item {
        (self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w).sqrt()
    }
}
