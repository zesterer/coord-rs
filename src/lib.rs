#![no_std]

mod vec1;
mod vec2;
mod vec3;
mod vec4;

// Reexports
pub use vec1::Vec1;
pub use vec2::Vec2;
pub use vec3::Vec3;
pub use vec4::Vec4;

pub trait VecItem: Copy + Clone + PartialEq {}

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

pub mod prelude {
    pub use super::Vec1;
    pub use super::Vec2;
    pub use super::Vec3;
    pub use super::Vec4;

    pub use super::Vec1u;
    pub use super::Vec2u;
    pub use super::Vec3u;
    pub use super::Vec4u;

    pub use super::Vec1i;
    pub use super::Vec2i;
    pub use super::Vec3i;
    pub use super::Vec4i;

    pub use super::Vec1f;
    pub use super::Vec2f;
    pub use super::Vec3f;
    pub use super::Vec4f;
}

#[cfg(test)]
mod tests {
    use super::prelude::*;

    // TODO
    // #[test]
    // fn test() {
    //     // Nothing yet
    // }
}
