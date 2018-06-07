use core::ops::{Add, Sub, Mul, Div};

pub use super::VecItem;

#[derive(Copy, Clone, Debug)]
pub struct Vec2<T: VecItem> {
    pub x: T,
    pub y: T,
}

impl<T: VecItem> Vec2<T> {
    pub fn new(x: T, y: T) -> Self { Self { x, y } }
}

impl<T: VecItem> From<[T; 2]> for Vec2<T> {
    fn from(arr: [T; 2]) -> Self { Self { x: arr[0], y: arr[1] } }
}

impl<T: VecItem> From<(T, T)> for Vec2<T> {
    fn from(tup: (T, T)) -> Self { Self { x: tup.0, y: tup.1 } }
}

impl<T> Add for Vec2<T> where T: VecItem + Add, T::Output: VecItem + Add + Copy {
    type Output = Vec2<T::Output>;
    fn add(self, other: Self) -> Vec2<T::Output> {
        Vec2 {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl<T> Sub for Vec2<T> where T: VecItem + Sub, T::Output: VecItem + Sub + Copy {
    type Output = Vec2<T::Output>;
    fn sub(self, other: Self) -> Vec2<T::Output> {
        Vec2 {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl<T> Mul for Vec2<T> where T: VecItem + Mul, T::Output: VecItem + Mul + Copy {
    type Output = Vec2<T::Output>;
    fn mul(self, other: Self) -> Vec2<T::Output> {
        Vec2 {
            x: self.x * other.x,
            y: self.y * other.y,
        }
    }
}

impl<T> Div for Vec2<T> where T: VecItem + Div, T::Output: VecItem + Div + Copy {
    type Output = Vec2<T::Output>;
    fn div(self, other: Self) -> Vec2<T::Output> {
        Vec2 {
            x: self.x / other.x,
            y: self.y / other.y,
        }
    }
}
