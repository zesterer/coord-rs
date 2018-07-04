//! Mathematic functions and traits

pub use num::{Num, Integer, Unsigned, Signed, Float};
use super::{Vector, VecItem};

/// A trait for vectors containing numerical types
pub trait VecNum: Vector where Self::Item: VecItem + Num {
    /// Calculates the sum of all components of the vector
    fn sum(&self) -> Self::Item;
    /// Calculates the produce of all components of the vector
    fn product(&self) -> Self::Item;
}

/// A trait for vectors containing integer types
pub trait VecInt: Vector where Self::Item: VecItem + Integer {
    /// Performs a Eucledian division (i.e: rounds towards negative infinity) operation upon the vector
    fn div_euc(&self, other: Self) -> Self;
}

/// A trait for vectors containing unsigned numerical types
pub trait VecUnsigned: Vector where Self::Item: VecItem + Unsigned {}

/// A trait for vectors containing signed numerical types
pub trait VecSigned: Vector where Self::Item: VecItem + Signed {
    /// Calculates the snake length (also known as 'manhattan distance') of the vector
    ///
    /// *For unsigned vectors, this is identical to the `.sum()` of the vector*
    fn snake_length(&self) -> Self::Item;
}

/// A trait for vectors containing floating point numerical types
pub trait VecFloat: Vector where Self::Item: VecItem + Float {
    /// Calculates the magnitude of the vector
    fn length(&self) -> Self::Item;
    /// Calculates the normalized form of the vector *(i.e: a vector with identical direction but a magnitude of 1)*
    fn norm(&self) -> Self;

    /// Rounds each element of the vector down to the nearest whole number
    fn floor(&self) -> Self;

    /// Rounds each element of the vector up to the nearest whole number
    fn ceil(&self) -> Self;

    /// Rounds each element of the vector to the nearest whole number
    fn round(&self) -> Self;
}
