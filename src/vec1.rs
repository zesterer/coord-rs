//! Functionality pertaining to `Vec1`

use core::ops::{Add, Sub, Mul, Div, AddAssign, SubAssign, MulAssign, DivAssign};
use core::marker::PhantomData;
use core::fmt;
use num::{Num, Integer, Unsigned, Signed, Float};
use super::{Vector, VecItem, VecNum, VecInt, VecUnsigned, VecSigned, VecFloat};

#[derive(Copy, Clone)]
pub struct Vec1<'a, T: 'a + VecItem<'a>> {
    pub x: T,
    phantom: PhantomData<&'a T>,
}

impl<'a, T: VecItem<'a>> Vec1<'a, T> {
    pub fn new(x: T) -> Self { Self { x, phantom: PhantomData } }
}

impl<'a, T: VecItem<'a>> Vector<'a> for Vec1<'a, T> {
    type Item = T;
}

// Debug and Display traits

impl<'a, T: VecItem<'a> + fmt::Debug> fmt::Debug for Vec1<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(x: {:?})", self.x)
    }
}

impl<'a, T: VecItem<'a> + fmt::Display> fmt::Display for Vec1<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({})", self.x)
    }
}

// From traits

impl<'a, T: VecItem<'a>> From<T> for Vec1<'a, T> {
    fn from(item: T) -> Self { Self { x: item, phantom: PhantomData } }
}

impl<'a, T: VecItem<'a>> From<[T; 1]> for Vec1<'a, T> {
    fn from(arr: [T; 1]) -> Self { Self { x: arr[0], phantom: PhantomData } }
}

// Op traits

impl<'a, T> Add for Vec1<'a, T> where T: VecItem<'a> + Add, T::Output: VecItem<'a> + Add {
    type Output = Vec1<'a, T::Output>;
    fn add(self, other: Self) -> Vec1<'a, T::Output> {
        Vec1 {
            x: self.x + other.x,
            phantom: PhantomData,
        }
    }
}

impl<'a, T> Sub for Vec1<'a, T> where T: VecItem<'a> + Sub, T::Output: VecItem<'a> + Sub {
    type Output = Vec1<'a, T::Output>;
    fn sub(self, other: Self) -> Vec1<'a, T::Output> {
        Vec1 {
            x: self.x - other.x,
            phantom: PhantomData,
        }
    }
}

impl<'a, T> Mul for Vec1<'a, T> where T: VecItem<'a> + Mul, T::Output: VecItem<'a> + Mul {
    type Output = Vec1<'a, T::Output>;
    fn mul(self, other: Self) -> Vec1<'a, T::Output> {
        Vec1 {
            x: self.x * other.x,
            phantom: PhantomData,
        }
    }
}

impl<'a, T> Div for Vec1<'a, T> where T: VecItem<'a> + Div, T::Output: VecItem<'a> + Div {
    type Output = Vec1<'a, T::Output>;
    fn div(self, other: Self) -> Vec1<'a, T::Output> {
        Vec1 {
            x: self.x / other.x,
            phantom: PhantomData,
        }
    }
}

// Op primitive traits

impl<'a, T> Add<T> for Vec1<'a, T> where T: VecItem<'a> + Add, T::Output: VecItem<'a> + Add {
    type Output = Vec1<'a, T::Output>;
    fn add(self, other: T) -> Vec1<'a, T::Output> {
        Vec1 {
            x: self.x + other,
            phantom: PhantomData,
        }
    }
}

impl<'a, T> Sub<T> for Vec1<'a, T> where T: VecItem<'a> + Sub, T::Output: VecItem<'a> + Sub {
    type Output = Vec1<'a, T::Output>;
    fn sub(self, other: T) -> Vec1<'a, T::Output> {
        Vec1 {
            x: self.x - other,
            phantom: PhantomData,
        }
    }
}

impl<'a, T> Mul<T> for Vec1<'a, T> where T: VecItem<'a> + Mul, T::Output: VecItem<'a> + Mul {
    type Output = Vec1<'a, T::Output>;
    fn mul(self, other: T) -> Vec1<'a, T::Output> {
        Vec1 {
            x: self.x * other,
            phantom: PhantomData,
        }
    }
}

impl<'a, T> Div<T> for Vec1<'a, T> where T: VecItem<'a> + Div, T::Output: VecItem<'a> + Div {
    type Output = Vec1<'a, T::Output>;
    fn div(self, other: T) -> Vec1<'a, T::Output> {
        Vec1 {
            x: self.x / other,
            phantom: PhantomData,
        }
    }
}

// Assign operators

impl<'a, T> AddAssign for Vec1<'a, T> where T: VecItem<'a> + Add<Output=T> {
    fn add_assign(&mut self, other: Self) {
        *self = Vec1 {
            x: self.x + other.x,
            phantom: PhantomData,
        }
    }
}

impl<'a, T> SubAssign for Vec1<'a, T> where T: VecItem<'a> + Sub<Output=T> {
    fn sub_assign(&mut self, other: Self) {
        *self = Vec1 {
            x: self.x - other.x,
            phantom: PhantomData,
        }
    }
}

impl<'a, T> MulAssign for Vec1<'a, T> where T: VecItem<'a> + Mul<Output=T> {
    fn mul_assign(&mut self, other: Self) {
        *self = Vec1 {
            x: self.x * other.x,
            phantom: PhantomData,
        }
    }
}

impl<'a, T> DivAssign for Vec1<'a, T> where T: VecItem<'a> + Div<Output=T> {
    fn div_assign(&mut self, other: Self) {
        *self = Vec1 {
            x: self.x / other.x,
            phantom: PhantomData,
        }
    }
}

// Assign primitive operators

impl<'a, T> AddAssign<T> for Vec1<'a, T> where T: VecItem<'a> + Add<Output=T> {
    fn add_assign(&mut self, other: T) {
        *self = Vec1 {
            x: self.x + other,
            phantom: PhantomData,
        }
    }
}

impl<'a, T> SubAssign<T> for Vec1<'a, T> where T: VecItem<'a> + Sub<Output=T> {
    fn sub_assign(&mut self, other: T) {
        *self = Vec1 {
            x: self.x - other,
            phantom: PhantomData,
        }
    }
}

impl<'a, T> MulAssign<T> for Vec1<'a, T> where T: VecItem<'a> + Mul<Output=T> {
    fn mul_assign(&mut self, other: T) {
        *self = Vec1 {
            x: self.x * other,
            phantom: PhantomData,
        }
    }
}

impl<'a, T> DivAssign<T> for Vec1<'a, T> where T: VecItem<'a> + Div<Output=T> {
    fn div_assign(&mut self, other: T) {
        *self = Vec1 {
            x: self.x / other,
            phantom: PhantomData,
        }
    }
}

// VecNum traits

impl<'a, T> VecNum<'a> for Vec1<'a, T> where T: VecItem<'a> + Num {
    fn sum(&self) -> Self::Item {
        self.x
    }

    fn product(&self) -> Self::Item {
        self.x
    }
}

// VecSigned traits

impl<'a, T> VecSigned<'a> for Vec1<'a, T> where T: VecItem<'a> + Signed {
    fn snake_length(&self) -> Self::Item {
        self.x.abs()
    }
}

// VecFloat traits

impl<'a, T> VecFloat<'a> for Vec1<'a, T> where T: VecItem<'a> + Float {
    fn length(&self) -> Self::Item {
        self.x
    }

    fn normalize(&self) -> Self {
        Vec1::new(
            self.x,
        )
    }
}
