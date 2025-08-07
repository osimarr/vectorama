# vectorama

Very simple linear algebra library for Rust.

This crate is designed for OpenGL and glTF standards: it uses column-major matrices, YXZ Euler rotation order, and is hardcoded to use the `f32` data type.

## Features

- Column-major matrices (OpenGL/glTF convention)
- YXZ Euler angle rotation order (glTF standard)
- `f32`-only types for performance and interoperability
- Vectors, matrices, quaternions, translations, and scales
- Interoperability with [`nalgebra`] (optional, via feature flags)

## Examples

### Creating and using vectors

```rust
use vectorama::Vec3;

let v = Vec3::new(1.0, 2.0, 3.0);
let w = Vec3::ones();
let sum = v + w;
println!("{:?}", sum); // Vec3 { x: 2.0, y: 3.0, z: 4.0 }
```

### Creating and using matrices

```rust
use vectorama::Mat3;

let a = Mat3::ones();
let b = Mat3::ones();
let c = a * b;
println!("{:?}", c);
```

### Using quaternions for rotation

```rust
use vectorama::{UnitQuaternion, Vec3};

let axis = Vec3::new(0.0, 1.0, 0.0);
let angle = std::f32::consts::FRAC_PI_2;
let q = UnitQuaternion::from_axis_angle(axis, angle);

let v = Vec3::new(1.0, 0.0, 0.0);
let rotated = q.rotate_vector(v);
println!("{:?}", rotated);
```

### Using translations

```rust
use vectorama::{Translation3, Vec3};

let t = Translation3::new(1.0, 2.0, 3.0);
let v = Vec3::new(0.0, 0.0, 0.0);
let translated = t * v;
println!("{:?}", translated); // Vec3 { x: 1.0, y: 2.0, z: 3.0 }
```

## Licensing
This project is dual-licensed under the MIT License and the Apache License 2.0. You may choose either license to govern your use of this project.

SPDX-License-Identifier: MIT OR Apache-2.0
