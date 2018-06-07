use num::{Num, Integer, Unsigned, Signed, Float};
use super::{Vec, VecItem};

pub trait VecNum: Vec where Self::Item: VecItem + Num {
    fn sum(&self) -> Self::Item;
    fn product(&self) -> Self::Item;
}

pub trait VecInt: Vec where Self::Item: VecItem + Integer {}

pub trait VecUnsigned: Vec where Self::Item: VecItem + Unsigned {}

pub trait VecSigned: Vec where Self::Item: VecItem + Signed {
    fn snake_length(&self) -> Self::Item;
}

pub trait VecFloat: Vec where Self::Item: VecItem + Float {
    fn length(&self) -> Self::Item;
}
