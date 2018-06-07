use core::ops::{Add, Sub, Mul, Div};

pub use super::VecItem;

#[derive(Copy, Clone, Debug)]
pub struct Vec3<T: VecItem> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T: VecItem> Vec3<T> {
    pub fn new(x: T, y: T, z: T) -> Self { Self { x, y, z } }
}

impl<T: VecItem> From<[T; 3]> for Vec3<T> {
    fn from(arr: [T; 3]) -> Self { Self { x: arr[0], y: arr[1], z: arr[2] } }
}

impl<T: VecItem> From<(T, T, T)> for Vec3<T> {
    fn from(tup: (T, T, T)) -> Self { Self { x: tup.0, y: tup.1, z: tup.2 } }
}

impl<T> Add for Vec3<T> where T: VecItem + Add, T::Output: VecItem + Add + Copy {
    type Output = Vec3<T::Output>;
    fn add(self, other: Self) -> Vec3<T::Output> {
        Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl<T> Sub for Vec3<T> where T: VecItem + Sub, T::Output: VecItem + Sub + Copy {
    type Output = Vec3<T::Output>;
    fn sub(self, other: Self) -> Vec3<T::Output> {
        Vec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl<T> Mul for Vec3<T> where T: VecItem + Mul, T::Output: VecItem + Mul + Copy {
    type Output = Vec3<T::Output>;
    fn mul(self, other: Self) -> Vec3<T::Output> {
        Vec3 {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}

impl<T> Div for Vec3<T> where T: VecItem + Div, T::Output: VecItem + Div + Copy {
    type Output = Vec3<T::Output>;
    fn div(self, other: Self) -> Vec3<T::Output> {
        Vec3 {
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z,
        }
    }
}
