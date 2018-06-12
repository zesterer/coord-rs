/// # Examples
///
/// ```
/// #[macro_use] extern crate coord;
/// use coord::prelude::*;
///
/// let my_vec = vec1!(7);         // Standard instantiation
/// let my_vec = vec1!((12));      // Tuple instantiation
/// let my_vec = vec1!([3.2]);     // Array slice instantiation
/// let my_vec = vec1!([true; 1]); // Array copy instantiation
/// ```
#[macro_export]
#[allow(unused_macros)]
macro_rules! vec1 {
    ($arr:expr) => (
        Vec1::from($arr)
    );

    [$arr:expr; 1] => (
        Vec1::from([$arr; 1])
    );

    ($x:expr) => (
        Vec1::new($x)
    );
}

/// # Examples
///
/// ```
/// #[macro_use] extern crate coord;
/// use coord::prelude::*;
///
/// let my_vec = vec2!(7, 9);       // Standard instantiation
/// let my_vec = vec2!((12, -4));   // Tuple instantiation
/// let my_vec = vec2!([3.2, 8.9]); // Array slice instantiation
/// let my_vec = vec2!([true; 2]);  // Array copy instantiation
/// ```
#[macro_export]
#[allow(unused_macros)]
macro_rules! vec2 {
    ($arr:expr) => (
        Vec2::from($arr)
    );

    [$arr:expr; 2] => (
        Vec2::from([$arr; 2])
    );

    ($x:expr, $y:expr) => (
        Vec2::new($x, $y)
    );
}

/// # Examples
///
/// ```
/// #[macro_use] extern crate coord;
/// use coord::prelude::*;
///
/// let my_vec = vec3!(7, 9, 2);         // Standard instantiation
/// let my_vec = vec3!((12, -4, 0));     // Tuple instantiation
/// let my_vec = vec3!([3.2, 8.9, 7.3]); // Array slice instantiation
/// let my_vec = vec3!([true; 3]);       // Array copy instantiation
/// ```
#[macro_export]
#[allow(unused_macros)]
macro_rules! vec3 {
    ($arr:expr) => (
        Vec3::from($arr)
    );

    [$arr:expr; 3] => (
        Vec3::from([$arr; 3])
    );

    ($x:expr, $y:expr, $z:expr) => (
        Vec3::new($x, $y, $z)
    );
}

/// # Examples
///
/// ```
/// #[macro_use] extern crate coord;
/// use coord::prelude::*;
///
/// let my_vec = vec4!(7, 9, 2, -5);           // Standard instantiation
/// let my_vec = vec4!((12, -4, 0, 9));        // Tuple instantiation
/// let my_vec = vec4!([3.2, 8.9, 7.3, -2.0]); // Array slice instantiation
/// let my_vec = vec4!([true; 4]);             // Array copy instantiation
/// ```
#[macro_export]
#[allow(unused_macros)]
macro_rules! vec4 {
    ($arr:expr) => (
        Vec4::from($arr)
    );

    [$arr:expr; 4] => (
        Vec4::from([$arr; 4])
    );

    ($x:expr, $y:expr, $z:expr, $w:expr) => (
        Vec4::new($x, $y, $z, $w)
    );
}
