use num::{Num, Integer, Unsigned, Signed, Float};
use super::{Vector, VecItem};

pub trait VecNum<'a>: Vector<'a> where Self::Item: VecItem<'a> + Num {
    /// Calculates the sum of all components of the vector
    fn sum(&self) -> Self::Item;
    /// Calculates the produce of all components of the vector
    fn product(&self) -> Self::Item;
}

pub trait VecInt<'a>: Vector<'a> where Self::Item: VecItem<'a> + Integer {}

pub trait VecUnsigned<'a>: Vector<'a> where Self::Item: VecItem<'a> + Unsigned {}

pub trait VecSigned<'a>: Vector<'a> where Self::Item: VecItem<'a> + Signed {
    /// Calculates the snake length (also known as 'manhattan distance') of the vector
    ///
    /// *For unsigned vectors, this is identical to the `.sum()` of the vector*
    fn snake_length(&self) -> Self::Item;
}

pub trait VecFloat<'a>: Vector<'a> where Self::Item: VecItem<'a> + Float {
    /// Calculates the magnitude of the vector
    fn length(&self) -> Self::Item;
}
