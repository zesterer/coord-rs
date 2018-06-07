#![no_std]

extern crate num;

pub mod vec1;
pub mod vec2;
pub mod vec3;
pub mod vec4;
pub mod math;
#[macro_use]
pub mod macros;

// Reexports
pub use vec1::Vec1;
pub use vec2::Vec2;
pub use vec3::Vec3;
pub use vec4::Vec4;
pub use math::{VecNum, VecInt, VecUnsigned, VecSigned, VecFloat};
pub use num::{Num, Integer, Unsigned, Signed, Float};

pub trait VecItem: Copy + Clone + PartialEq {}

pub trait Vec {
    type Item: VecItem;
}

impl VecItem for u8 {}
impl VecItem for u16 {}
impl VecItem for u32 {}
impl VecItem for u64 {}
impl VecItem for u128 {}

impl VecItem for i8 {}
impl VecItem for i16 {}
impl VecItem for i32 {}
impl VecItem for i64 {}
impl VecItem for i128 {}

impl VecItem for f32 {}
impl VecItem for f64 {}

#[cfg(not(feature = "large_defaults"))]
pub mod defaults {
    use super::*;

    pub type Vec1u = Vec1<u32>;
    pub type Vec2u = Vec2<u32>;
    pub type Vec3u = Vec3<u32>;
    pub type Vec4u = Vec4<u32>;

    pub type Vec1i = Vec1<i32>;
    pub type Vec2i = Vec2<i32>;
    pub type Vec3i = Vec3<i32>;
    pub type Vec4i = Vec4<i32>;

    pub type Vec1f = Vec1<f32>;
    pub type Vec2f = Vec2<f32>;
    pub type Vec3f = Vec3<f32>;
    pub type Vec4f = Vec4<f32>;
}

#[cfg(feature = "large_defaults")]
pub mod defaults {
    use super::*;

    pub type Vec1u = Vec1<u64>;
    pub type Vec2u = Vec2<u64>;
    pub type Vec3u = Vec3<u64>;
    pub type Vec4u = Vec4<u64>;

    pub type Vec1i = Vec1<i64>;
    pub type Vec2i = Vec2<i64>;
    pub type Vec3i = Vec3<i64>;
    pub type Vec4i = Vec4<i64>;

    pub type Vec1f = Vec1<f64>;
    pub type Vec2f = Vec2<f64>;
    pub type Vec3f = Vec3<f64>;
    pub type Vec4f = Vec4<f64>;
}

pub mod prelude {
	pub use super::{Num, Integer, Unsigned, Signed, Float};

    pub use super::Vec1;
    pub use super::Vec2;
    pub use super::Vec3;
    pub use super::Vec4;

    pub use super::Vec;
    pub use super::VecNum;
    pub use super::VecInt;
    pub use super::VecUnsigned;
    pub use super::VecSigned;
    pub use super::VecFloat;

    pub use super::defaults::*;
}

#[cfg(test)]
mod tests {
    use super::prelude::*;

    #[test]
    fn basic_construction() {
        let _ = Vec1::new(1u32);
        let _ = Vec2::new(1u32, 2u32);
        let _ = Vec3::new(1u32, 2u32, 3u32);
        let _ = Vec4::new(1u32, 2u32, 3u32, 4u32);

        let _ = Vec2::from((1u32, 2u32));
        let _ = Vec3::from((1u32, 2u32, 3u32));
        let _ = Vec4::from((1u32, 2u32, 3u32, 4u32));
    }

    #[test]
    fn basic_operators() {
        let _ = Vec1u::new(0)          + Vec1u::new(3);
        let _ = Vec2u::new(0, 1)       + Vec2u::new(3, 2);
        let _ = Vec3u::new(0, 1, 2)    + Vec3u::new(3, 2, 1);
        let _ = Vec4u::new(0, 1, 2, 3) + Vec4u::new(3, 2, 1, 0);

        let _ = Vec1u::new(3)          - Vec1u::new(3);
        let _ = Vec2u::new(3, 2)       - Vec2u::new(3, 2);
        let _ = Vec3u::new(3, 2, 1)    - Vec3u::new(3, 2, 1);
        let _ = Vec4u::new(3, 2, 1, 0) - Vec4u::new(3, 2, 1, 0);

        let _ = Vec1u::new(1)          * Vec1u::new(4);
        let _ = Vec2u::new(1, 2)       * Vec2u::new(4, 3);
        let _ = Vec3u::new(1, 2, 3)    * Vec3u::new(4, 3, 2);
        let _ = Vec4u::new(1, 2, 3, 4) * Vec4u::new(4, 3, 2, 1);

        let _ = Vec1u::new(4)          / Vec1u::new(5);
        let _ = Vec2u::new(4, 3)       / Vec2u::new(5, 4);
        let _ = Vec3u::new(4, 3, 2)    / Vec3u::new(5, 4, 3);
        let _ = Vec4u::new(4, 3, 2, 1) / Vec4u::new(5, 4, 3, 2);
    }

    fn pass_vec1u(_: Vec1u) {}
    fn pass_vec2u(_: Vec2u) {}
    fn pass_vec3u(_: Vec3u) {}
    fn pass_vec4u(_: Vec4u) {}

    #[test]
    fn test_pass() {
        pass_vec1u(vec1!(1u32));
        pass_vec2u(vec2!(1u32, 2u32));
        pass_vec3u(vec3!(1u32, 2u32, 3u32));
        pass_vec4u(vec4!(1u32, 2u32, 3u32, 4u32));
    }

    fn length_of<V: VecFloat>(vec: V) -> V::Item where V::Item: Float {
        vec.length()
    }

    #[test]
    fn test_passing() {
        let v2 = vec2!(1, 2) + vec2!(2, 1);
        //println!("{:?}", v2);

        let v3 = vec3!(1.5, 2.0, 6.5) * vec3!(3.2, 7.7, 8.2);
        let l = v3.length();
        //println!("{:?}", v3.length());
    }
}
