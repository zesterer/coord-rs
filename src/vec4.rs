use core::ops::{Add, Sub, Mul, Div};

pub use super::VecItem;

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

impl<T: VecItem> From<[T; 4]> for Vec4<T> {
    fn from(arr: [T; 4]) -> Self { Self { x: arr[0], y: arr[1], z: arr[2], w: arr[3] } }
}

impl<T: VecItem> From<(T, T, T, T)> for Vec4<T> {
    fn from(tup: (T, T, T, T)) -> Self { Self { x: tup.0, y: tup.1, z: tup.2, w: tup.3 } }
}

impl<T> Add for Vec4<T> where T: VecItem + Add, T::Output: VecItem + Add + Copy {
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

impl<T> Sub for Vec4<T> where T: VecItem + Sub, T::Output: VecItem + Sub + Copy {
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

impl<T> Mul for Vec4<T> where T: VecItem + Mul, T::Output: VecItem + Mul + Copy {
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

impl<T> Div for Vec4<T> where T: VecItem + Div, T::Output: VecItem + Div + Copy {
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
