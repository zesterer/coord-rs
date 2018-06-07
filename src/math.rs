use num::{Num, Integer, Unsigned, Signed, Float};
use super::{Vec, VecItem};

pub trait VecItemNum: VecItem + Num {}
pub trait VecItemInt: VecItemNum + Integer {}
pub trait VecItemUnsigned: VecItemInt + Integer + Unsigned {}
pub trait VecItemSigned: VecItemInt + Integer + Signed {}
pub trait VecItemFloat: VecItemNum + Float {}

pub trait VecNum: Vec where Self::Item: VecItemNum {
    fn sum(&self) -> Self::Item;
    fn product(&self) -> Self::Item;
}

pub trait VecInt: Vec where Self::Item: VecItemInt {}

pub trait VecUnsigned: Vec where Self::Item: VecItemUnsigned {}

pub trait VecSigned: Vec where Self::Item: VecItemSigned {
    fn snake_length(&self) -> Self::Item;
}

pub trait VecFloat: Vec where Self::Item: VecItemFloat {
    fn length(&self) -> Self::Item;
}
