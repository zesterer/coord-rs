#[macro_export]
#[allow(unused_macros)]
macro_rules! vec1 {
    ($x:expr) => (
        Vec1::new($x)
    );

    ($arr:expr) => (
        Vec1::from($arr)
    );
}


#[macro_export]
#[allow(unused_macros)]
macro_rules! vec2 {
    ($x:expr, $y:expr) => (
        Vec2::new($x, $y)
    );

    ($arr:expr) => (
        Vec2::from($arr)
    );
}

#[macro_export]
#[allow(unused_macros)]
macro_rules! vec3 {
    ($x:expr, $y:expr, $z:expr) => (
        Vec3::new($x, $y, $z)
    );

    ($arr:expr) => (
        Vec3::from($arr)
    );
}

#[macro_export]
#[allow(unused_macros)]
macro_rules! vec4 {
    ($x:expr, $y:expr, $z:expr, $w:expr) => (
        Vec4::new($x, $y, $z, $w)
    );

    ($arr:expr) => (
        Vec4::from($arr)
    );
}
