# Coord

---

Coord is a simple, intuitive vector mathematics library for Rust.

---

## Example

```
#[macro_use]
extern crate coord;

use coord::prelude::*;

fn length_of<V: VecFloat>(vec: V) -> V::Item where V::Item: Float {
	vec.length()
}

fn main() {
	let v2 = vec2!(1, 2) + vec2!(2, 1);
	println!("{:?}", v2);

	let v3 = vec3!(1.5, 2.0, 6.5) * vec3!(3.2, 7.7, 8.2);
	println!("{:?}", v3.length());
}
```

## Features

- [x] Generic `Vec1`, `Vec2`, `Vec3` and `Vec4` types
- [x] `VecXu`, `VecXi` and `VecXf` type definitions
- [x] Implementation of basic mathematics operations (`Add`, `Sub`, `Mul`, `Div`)
- [ ] Implementation of mathematics functions (i.e: `.length()`, `.cross(v)`, etc.)
- [ ] Implementation of bitwise operations
