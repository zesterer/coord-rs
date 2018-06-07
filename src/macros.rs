#[allow(unused_macros)]
macro_rules! vec {
    ($x:expr) => (
        Vec1::new($x)
    );

    ($arr:expr) => (
        Vec1::from($arr)
    );

    ($x:expr, $y:expr) => (
        Vec2::new($x, $y)
    );

    ($x:expr, $y:expr, $z:expr) => (
        Vec3::new($x, $y, $z)
    );

    ($x:expr, $y:expr, $z:expr, $w:expr) => (
        Vec4::new($x, $y, $z, $w)
    );
}
