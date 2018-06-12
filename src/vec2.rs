use core::ops::{Add, Sub, Mul, Div, AddAssign, SubAssign, MulAssign, DivAssign};
use core::marker::PhantomData;
use num::{Num, Integer, Unsigned, Signed, Float};
use super::{Vector, VecItem, VecNum, VecInt, VecUnsigned, VecSigned, VecFloat};

#[derive(Copy, Clone, Debug)]
pub struct Vec2<'a, T: 'a + VecItem<'a>> {
    pub x: T,
    pub y: T,
    phantom: PhantomData<&'a T>,
}

impl<'a, T: VecItem<'a>> Vec2<'a, T> {
    pub fn new(x: T, y: T) -> Self { Self { x, y, phantom: PhantomData } }
}

impl<'a, T: VecItem<'a>> Vector<'a> for Vec2<'a, T> {
    type Item = T;
}

// From traits

impl<'a, T: VecItem<'a>> From<[T; 2]> for Vec2<'a, T> {
    fn from(arr: [T; 2]) -> Self { Self { x: arr[0], y: arr[1], phantom: PhantomData } }
}

impl<'a, T: VecItem<'a>> From<(T, T)> for Vec2<'a, T> {
    fn from(tup: (T, T)) -> Self { Self { x: tup.0, y: tup.1, phantom: PhantomData } }
}

// Op traits

impl<'a, T> Add for Vec2<'a, T> where T: VecItem<'a> + Add, T::Output: VecItem<'a> + Add {
    type Output = Vec2<'a, T::Output>;
    fn add(self, other: Self) -> Vec2<'a, T::Output> {
        Vec2 {
            x: self.x + other.x,
            y: self.y + other.y,
            phantom: PhantomData,
        }
    }
}

impl<'a, T> Sub for Vec2<'a, T> where T: VecItem<'a> + Sub, T::Output: VecItem<'a> + Sub {
    type Output = Vec2<'a, T::Output>;
    fn sub(self, other: Self) -> Vec2<'a, T::Output> {
        Vec2 {
            x: self.x - other.x,
            y: self.y - other.y,
            phantom: PhantomData,
        }
    }
}

impl<'a, T> Mul for Vec2<'a, T> where T: VecItem<'a> + Mul, T::Output: VecItem<'a> + Mul {
    type Output = Vec2<'a, T::Output>;
    fn mul(self, other: Self) -> Vec2<'a, T::Output> {
        Vec2 {
            x: self.x * other.x,
            y: self.y * other.y,
            phantom: PhantomData,
        }
    }
}

impl<'a, T> Div for Vec2<'a, T> where T: VecItem<'a> + Div, T::Output: VecItem<'a> + Div {
    type Output = Vec2<'a, T::Output>;
    fn div(self, other: Self) -> Vec2<'a, T::Output> {
        Vec2 {
            x: self.x / other.x,
            y: self.y / other.y,
            phantom: PhantomData,
        }
    }
}

// Op primitive traits

impl<'a, T> Add<T> for Vec2<'a, T> where T: VecItem<'a> + Add, T::Output: VecItem<'a> + Add {
    type Output = Vec2<'a, T::Output>;
    fn add(self, other: T) -> Vec2<'a, T::Output> {
        Vec2 {
            x: self.x + other,
            y: self.y + other,
            phantom: PhantomData,
        }
    }
}

impl<'a, T> Sub<T> for Vec2<'a, T> where T: VecItem<'a> + Sub, T::Output: VecItem<'a> + Sub {
    type Output = Vec2<'a, T::Output>;
    fn sub(self, other: T) -> Vec2<'a, T::Output> {
        Vec2 {
            x: self.x - other,
            y: self.y - other,
            phantom: PhantomData,
        }
    }
}

impl<'a, T> Mul<T> for Vec2<'a, T> where T: VecItem<'a> + Mul, T::Output: VecItem<'a> + Mul {
    type Output = Vec2<'a, T::Output>;
    fn mul(self, other: T) -> Vec2<'a, T::Output> {
        Vec2 {
            x: self.x * other,
            y: self.y * other,
            phantom: PhantomData,
        }
    }
}

impl<'a, T> Div<T> for Vec2<'a, T> where T: VecItem<'a> + Div, T::Output: VecItem<'a> + Div {
    type Output = Vec2<'a, T::Output>;
    fn div(self, other: T) -> Vec2<'a, T::Output> {
        Vec2 {
            x: self.x / other,
            y: self.y / other,
            phantom: PhantomData,
        }
    }
}

// Assign operators

impl<'a, T> AddAssign for Vec2<'a, T> where T: VecItem<'a> + Add<Output=T> {
    fn add_assign(&mut self, other: Self) {
        *self = Vec2 {
            x: self.x + other.x,
            y: self.y + other.y,
            phantom: PhantomData,
        }
    }
}

impl<'a, T> SubAssign for Vec2<'a, T> where T: VecItem<'a> + Sub<Output=T> {
    fn sub_assign(&mut self, other: Self) {
        *self = Vec2 {
            x: self.x - other.x,
            y: self.y - other.y,
            phantom: PhantomData,
        }
    }
}

impl<'a, T> MulAssign for Vec2<'a, T> where T: VecItem<'a> + Mul<Output=T> {
    fn mul_assign(&mut self, other: Self) {
        *self = Vec2 {
            x: self.x * other.x,
            y: self.y * other.y,
            phantom: PhantomData,
        }
    }
}

impl<'a, T> DivAssign for Vec2<'a, T> where T: VecItem<'a> + Div<Output=T> {
    fn div_assign(&mut self, other: Self) {
        *self = Vec2 {
            x: self.x / other.x,
            y: self.y / other.y,
            phantom: PhantomData,
        }
    }
}

// Assign primitive operators

impl<'a, T> AddAssign<T> for Vec2<'a, T> where T: VecItem<'a> + Add<Output=T> {
    fn add_assign(&mut self, other: T) {
        *self = Vec2 {
            x: self.x + other,
            y: self.y + other,
            phantom: PhantomData,
        }
    }
}

impl<'a, T> SubAssign<T> for Vec2<'a, T> where T: VecItem<'a> + Sub<Output=T> {
    fn sub_assign(&mut self, other: T) {
        *self = Vec2 {
            x: self.x - other,
            y: self.y - other,
            phantom: PhantomData,
        }
    }
}

impl<'a, T> MulAssign<T> for Vec2<'a, T> where T: VecItem<'a> + Mul<Output=T> {
    fn mul_assign(&mut self, other: T) {
        *self = Vec2 {
            x: self.x * other,
            y: self.y * other,
            phantom: PhantomData,
        }
    }
}

impl<'a, T> DivAssign<T> for Vec2<'a, T> where T: VecItem<'a> + Div<Output=T> {
    fn div_assign(&mut self, other: T) {
        *self = Vec2 {
            x: self.x / other,
            y: self.y / other,
            phantom: PhantomData,
        }
    }
}

// VecNum traits

impl<'a, T> VecNum<'a> for Vec2<'a, T> where T: VecItem<'a> + Num {
    fn sum(&self) -> Self::Item {
        self.x + self.y
    }

    fn product(&self) -> Self::Item {
        self.x * self.y
    }
}

// VecSigned traits

impl<'a, T> VecSigned<'a> for Vec2<'a, T> where T: VecItem<'a> + Signed {
    fn snake_length(&self) -> Self::Item {
        self.x.abs() + self.y.abs()
    }
}

// VecFloat traits

impl<'a, T> VecFloat<'a> for Vec2<'a, T> where T: VecItem<'a> + Float {
    fn length(&self) -> Self::Item {
        (self.x * self.x + self.y * self.y).sqrt()
    }
}
