//! Mathematic functions and traits

pub use num::{Num, Integer, Unsigned, Signed, Float};
use super::{Vector, VecItem};

/// A trait for vectors containing numerical types
pub trait VecNum<'a>: Vector<'a> where Self::Item: VecItem<'a> + Num {
    /// Calculates the sum of all components of the vector
    fn sum(&self) -> Self::Item;
    /// Calculates the produce of all components of the vector
    fn product(&self) -> Self::Item;
}

/// A trait for vectors containing integer types
pub trait VecInt<'a>: Vector<'a> where Self::Item: VecItem<'a> + Integer {}

/// A trait for vectors containing unsigned numerical types
pub trait VecUnsigned<'a>: Vector<'a> where Self::Item: VecItem<'a> + Unsigned {}

/// A trait for vectors containing signed numerical types
pub trait VecSigned<'a>: Vector<'a> where Self::Item: VecItem<'a> + Signed {
    /// Calculates the snake length (also known as 'manhattan distance') of the vector
    ///
    /// *For unsigned vectors, this is identical to the `.sum()` of the vector*
    fn snake_length(&self) -> Self::Item;
}

/// A trait for vectors containing floating point numerical types
pub trait VecFloat<'a>: Vector<'a> where Self::Item: VecItem<'a> + Float {
    /// Calculates the magnitude of the vector
    fn length(&self) -> Self::Item;
    // Calculates the normalized form of the vector *(i.e: a vector with identical direction but a magnitude of 1)*
    fn normalize(&self) -> Self;
}
