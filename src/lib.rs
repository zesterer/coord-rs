//! `coord` is a simple, ergonomic vector mathematics crate designed for use in game development, physics engines and other programs that deal with general-purpose multi-variable mathematics
//!
//! # Features
//!
//! - Support for basic vector operations
//! - Support for basic primitive operations
//! - Basic mathematical operations upon vectors
//! - Vector serialization
//!
//! # Examples
//!
//! ```
//! #[macro_use] extern crate coord;
//! use coord::prelude::*;
//!
//! fn main() {
//!     // Coord supports 4 multi-variable vector types: Vec1, Vec2, Vec3 and Vec4
//!     let mut v0 = vec3!(1, 2, 3);
//!     v0 += vec3![1; 3] * 5;
//!     let _ = v0 * vec3!([10, 10, 10]);
//!
//!     // Coord allows arbitrary vector component types
//!     let _ = vec2!(true, false); // Create a boolean vector
//! }
//! ```

#![no_std]

extern crate num;
#[macro_use]
extern crate serde_derive;
extern crate serde;

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
use num::{Num, Integer, Unsigned, Signed, Float};

use serde::{Serialize, Deserialize};

pub trait VecItem<'a>: Copy + Clone + PartialEq + Serialize + Deserialize<'a> {}

pub trait Vector<'a> {
    type Item: VecItem<'a>;
}

impl<'a> VecItem<'a> for bool {}

impl<'a> VecItem<'a> for u8 {}
impl<'a> VecItem<'a> for u16 {}
impl<'a> VecItem<'a> for u32 {}
impl<'a> VecItem<'a> for u64 {}
impl<'a> VecItem<'a> for u128 {}

impl<'a> VecItem<'a> for i8 {}
impl<'a> VecItem<'a> for i16 {}
impl<'a> VecItem<'a> for i32 {}
impl<'a> VecItem<'a> for i64 {}
impl<'a> VecItem<'a> for i128 {}

impl<'a> VecItem<'a> for f32 {}
impl<'a> VecItem<'a> for f64 {}

#[cfg(not(feature = "large_defaults"))]
pub mod defaults {
    //! This module contains several type definitions that make working with `coord` simpler and faster
    //!
    //! *The default size for numerical types is 32 bits*

    use super::*;

    pub type Vec1b<'a> = Vec1<'a, bool>;

    pub type Vec1u<'a> = Vec1<'a, u32>;
    pub type Vec2u<'a> = Vec2<'a, u32>;
    pub type Vec3u<'a> = Vec3<'a, u32>;
    pub type Vec4u<'a> = Vec4<'a, u32>;

    pub type Vec1i<'a> = Vec1<'a, i32>;
    pub type Vec2i<'a> = Vec2<'a, i32>;
    pub type Vec3i<'a> = Vec3<'a, i32>;
    pub type Vec4i<'a> = Vec4<'a, i32>;

    pub type Vec1f<'a> = Vec1<'a, f32>;
    pub type Vec2f<'a> = Vec2<'a, f32>;
    pub type Vec3f<'a> = Vec3<'a, f32>;
    pub type Vec4f<'a> = Vec4<'a, f32>;
}

#[cfg(feature = "large_defaults")]
pub mod defaults {
    //! This module contains several type definitions that make working with `coord` simpler and faster
    //!
    //! *The default size for numerical types is 64 bits*

    use super::*;

    pub type Vec1b<'a> = Vec1<'a, bool>;

    pub type Vec1u<'a> = Vec1<'a, u64>;
    pub type Vec2u<'a> = Vec2<'a, u64>;
    pub type Vec3u<'a> = Vec3<'a, u64>;
    pub type Vec4u<'a> = Vec4<'a, u64>;

    pub type Vec1i<'a> = Vec1<'a, i64>;
    pub type Vec2i<'a> = Vec2<'a, i64>;
    pub type Vec3i<'a> = Vec3<'a, i64>;
    pub type Vec4i<'a> = Vec4<'a, i64>;

    pub type Vec1f<'a> = Vec1<'a, f64>;
    pub type Vec2f<'a> = Vec2<'a, f64>;
    pub type Vec3f<'a> = Vec3<'a, f64>;
    pub type Vec4f<'a> = Vec4<'a, f64>;
}

pub mod prelude {
    //! This module contains a variety of commonly used types and functions.
    //!
    //! # Examples
    //!
    //! ```
    //! #[macro_use] extern crate coord;
    //! use coord::prelude::*;
    //! ```

	pub use super::Vec1;
    pub use super::Vec2;
    pub use super::Vec3;
    pub use super::Vec4;

    pub use super::Vector;
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
    fn basic_operations() {
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

    #[test]
    fn primitive_operations() {
        let _ = Vec1u::new(0)          + 1;
        let _ = Vec2u::new(0, 1)       + 2;
        let _ = Vec3u::new(0, 1, 2)    + 3;
        let _ = Vec4u::new(0, 1, 2, 3) + 4;

        let _ = Vec1u::new(6)          - 1;
        let _ = Vec2u::new(6, 7)       - 2;
        let _ = Vec3u::new(6, 7, 8)    - 3;
        let _ = Vec4u::new(6, 7, 8, 9) - 4;

        let _ = Vec1u::new(0)          * 1;
        let _ = Vec2u::new(0, 1)       * 2;
        let _ = Vec3u::new(0, 1, 2)    * 3;
        let _ = Vec4u::new(0, 1, 2, 3) * 4;

        let _ = Vec1u::new(0)          / 1;
        let _ = Vec2u::new(0, 1)       / 2;
        let _ = Vec3u::new(0, 1, 2)    / 3;
        let _ = Vec4u::new(0, 1, 2, 3) / 4;
    }

    #[test]
    fn basic_assignment() {
        let mut v1 = Vec1u::new(0);
        let mut v2 = Vec2u::new(0, 1);
        let mut v3 = Vec3u::new(0, 1, 2);
        let mut v4 = Vec4u::new(0, 1, 2, 3);

        v1 += Vec1u::new(0);
        v1 -= Vec1u::new(0);
        v1 *= Vec1u::new(1);
        v1 /= Vec1u::new(1);

        v2 += Vec2u::new(0, 1);
        v2 -= Vec2u::new(0, 1);
        v2 *= Vec2u::new(1, 2);
        v2 /= Vec2u::new(1, 2);

        v3 += Vec3u::new(0, 1, 2);
        v3 -= Vec3u::new(0, 1, 2);
        v3 *= Vec3u::new(1, 2, 3);
        v3 /= Vec3u::new(1, 2, 3);

        v4 += Vec4u::new(0, 1, 2, 3);
        v4 -= Vec4u::new(0, 1, 2, 3);
        v4 *= Vec4u::new(1, 2, 3, 4);
        v4 /= Vec4u::new(1, 2, 3, 4);
    }

    #[test]
    fn primitive_assignment() {
        let mut v1 = Vec1u::new(0);
        let mut v2 = Vec2u::new(0, 1);
        let mut v3 = Vec3u::new(0, 1, 2);
        let mut v4 = Vec4u::new(0, 1, 2, 3);

        v1 += 0;
        v1 -= 0;
        v1 *= 1;
        v1 /= 1;

        v2 += 0;
        v2 -= 0;
        v2 *= 1;
        v2 /= 1;

        v3 += 0;
        v3 -= 0;
        v3 *= 1;
        v3 /= 1;

        v4 += 0;
        v4 -= 0;
        v4 *= 1;
        v4 /= 1;
    }

    fn pass_vec1u(_: Vec1u) {}
    fn pass_vec2u(_: Vec2u) {}
    fn pass_vec3u(_: Vec3u) {}
    fn pass_vec4u(_: Vec4u) {}

    #[test]
    fn pass_to_func() {
        pass_vec1u(vec1!(1u32));
        pass_vec2u(vec2!(1u32, 2u32));
        pass_vec3u(vec3!(1u32, 2u32, 3u32));
        pass_vec4u(vec4!(1u32, 2u32, 3u32, 4u32));
    }

    fn length_of<'a, V: VecFloat<'a>>(vec: V) -> V::Item where V::Item: Float {
        vec.length()
    }

    #[test]
    fn pass_generic() {
        let v3 = vec3!(1.5, 2.0, 6.5);
        let _ = v3.length();
        let _ = length_of(v3);
    }

    #[test]
    fn macros() {
        let _v1_0 = vec1!(7);
        let _v1_1 = vec1!([7]);
        let _v1_2 = vec1![7; 1];

        let _v2_0 = vec2!(7, 5);
        let _v2_1 = vec2!([7, 5]);
        let _v2_2 = vec2![7; 2];

        let _v3_0 = vec3!(7, 5, 3);
        let _v3_1 = vec3!([7, 5, 3]);
        let _v3_2 = vec3![7; 3];

        let _v4_0 = vec4!(7, 5, 3 ,1);
        let _v4_1 = vec4!([7, 5, 3, 1]);
        let _v4_2 = vec4![7; 4];
    }

    #[test]
    fn serialize() {
        // Nothing yet
    }

    #[test]
    fn deserialize() {
        // Nothing yet
    }
}
