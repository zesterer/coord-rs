# <img src="https://imgur.com/qcUMer7.png" alt="Coord" width="32px"/> Coord

[![Build Status][ci-badge]][ci] [![Crates.io][cr-badge]][cr] ![Downloads][dl-badge] [![Docs][doc-badge]][doc]

[ci-badge]: https://travis-ci.org/zesterer/coord-rs.svg?branch=master
[ci]: https://travis-ci.org/zesterer/coord-rs
[cr-badge]: https://img.shields.io/crates/v/coord.svg
[cr]: https://crates.io/crates/coord
[dl-badge]: https://img.shields.io/crates/d/coord.svg
[doc-badge]: https://docs.rs/coord/badge.svg
[doc]: https://docs.rs/coord

## Description

Coord is a simple, ergonomic vector mathematics crate for Rust designed for use in game development, physics engines and other programs that deal with general-purpose multi-variable mathematics.

**Coord is now `no_std` compatible!**

## Example

```rust
#[macro_use]
extern crate coord;
use coord::prelude::*;

fn main() {
	// Coord supports 4 multi-variable vector types: Vec1, Vec2, Vec3 and Vec4
	let mut v = vec3!(1.0, 2.5, 3.0);

	// Coord supports common mathematical operations for both primitive and vector types
	v += vec3![1.0; 3] * 5.0;
	let _ = v * vec3!([10.0, 10.0, 10.0]);

	// Coord implements many common mathematic functions
	let _ = v.length();
	let _ = v.normalize();

	// Coord supports debug and display printing of vectors
	println!("Debug => {:?}", v);
	println!("Display => {}", v);

	// Coord allows arbitrary vector component types
	let _ = vec2!(true, false); // Create a boolean vector
}
```

**For more examples, visit [https://docs.rs/coord](https://docs.rs/coord)**

## Features

- [x] Generic `Vec1`, `Vec2`, `Vec3` and `Vec4` types
- [x] Utility macros to make vector manipulation simpler
- [x] `VecXu`, `VecXi` and `VecXf` default type definitions
- [x] Basic mathematic operations (`Add`, `Sub`, `Mul`, `Div`)
- [x] Mathematic functions (i.e: `.length()`, `.normalize()`, etc.)
- [x] Serialization support with the `serialize` feature
- [x] 64 bit default type support with the `large_defaults` feature

## Coming Soon

- [ ] Bitwise operations
- [ ] More mathematic functions
- [ ] Modulo operator for integer vector types

## Using Coord

To use Coord in your Rust project, add the following line beneath the `[dependencies]` section in your `Cargo.toml` file.

```
coord = "0.7.0"
```

If you want to enable serialization or 64-bit defaults, you should specify the dependency like the following line instead.

```
[dependencies.coord]
version = "0.7.0"
features = ["serialize", "large_defaults"]
```

## FAQ

### Why does Coord exist?

Coord came about as a result of a general dissatisfaction with existing vector mathematics libraries during development of the [Veloren Project](https://github.com/veloren/game). Existing solutions were either too complicated, awkward to use, or required too many dependencies.

### Does Coord aim to eventually implement feature X?

Coord does not aim to be a fully-blown N-dimensional mathematics library. It aims to implement a small yet elegant set of features that make it suitable for applications where simple multi-variable mathematics is required.

If you think feature X falls within that scope, and is not yet on the project todo list, you can open an issue on GitHub and I'll consider implementing it.

### Why does Coord choose 32 bit values for its default types?

Coord defines a set of 'default' types within the `defaults` module. They are included within the `prelude` module too. These default types, such as `Vec2f`, `Vec3u`, `Vec4b`, etc. exist to make writing code with Coord faster. It is still possible to program without them like `Vec2<f32>`, `Vec3<u32>`, `Vec4<bool>`, etc. Coord chooses 32-bit types by default because it is my belief that these are more useful when developing for games and physics engines, which require fast calculations. Additionally, 32 bit floating point types means that Coord is compatible with OpenGL.

### Why does Coord not have a general-purpose `vec!` macro?

Such a macro would conflict with the `vec!` macro within Rust's standard library.

### Why does Coord not rely on the standard library?

It doesn't need to. Why limit the use cases of the library by requiring `std`?

### Bug! Bug! I've found a bug!

Open an issue on Github and I'll fix it as soon as possible.

[You can open an issue here](https://github.com/zesterer/coord-rs/issues/new)

## License

Coord is open source software, licensed under the MIT license (LICENSE-MIT or [http://opensource.org/licenses/MIT](http://opensource.org/licenses/MIT))
