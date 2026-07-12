[bevy](../../index.html)::[math](../index.html)::[prelude](index.html)

# Trait Measured3d 

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/primitives/mod.rs.html#48)

```rust
pub trait Measured3d {
    // Required methods
    fn area(&self) -> f32;
    fn volume(&self) -> f32;
}
```

A trait for getting measurements of 3D shapes

## Required Methods

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/primitives/mod.rs.html#50)

#### fn [area](#tymethod.area)(&self) -> [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)

Get the surface area of the shape

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/primitives/mod.rs.html#53)

#### fn [volume](#tymethod.volume)(&self) -> [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)

Get the volume of the shape

## Dyn Compatibility

This trait **is** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/primitives/dim3.rs.html#903)

### impl [Measured3d](../../prelude/trait.Measured3d.html "trait bevy::prelude::Measured3d") for [Capsule3d](../../prelude/struct.Capsule3d.html "struct bevy::prelude::Capsule3d")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/primitives/dim3.rs.html#989)

### impl [Measured3d](../../prelude/trait.Measured3d.html "trait bevy::prelude::Measured3d") for [Cone](../../prelude/struct.Cone.html "struct bevy::prelude::Cone")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/primitives/dim3.rs.html#1085)

### impl [Measured3d](../../prelude/trait.Measured3d.html "trait bevy::prelude::Measured3d") for [ConicalFrustum](../../prelude/struct.ConicalFrustum.html "struct bevy::prelude::ConicalFrustum")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/primitives/dim3.rs.html#756)

### impl [Measured3d](../../prelude/trait.Measured3d.html "trait bevy::prelude::Measured3d") for [Cuboid](../../prelude/struct.Cuboid.html "struct bevy::prelude::Cuboid")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/primitives/dim3.rs.html#836)

### impl [Measured3d](../../prelude/trait.Measured3d.html "trait bevy::prelude::Measured3d") for [Cylinder](../../prelude/struct.Cylinder.html "struct bevy::prelude::Cylinder")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/primitives/dim3.rs.html#77)

### impl [Measured3d](../../prelude/trait.Measured3d.html "trait bevy::prelude::Measured3d") for [Sphere](../../prelude/struct.Sphere.html "struct bevy::prelude::Sphere")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/primitives/dim3.rs.html#1512)

### impl [Measured3d](../../prelude/trait.Measured3d.html "trait bevy::prelude::Measured3d") for [Tetrahedron](../../prelude/struct.Tetrahedron.html "struct bevy::prelude::Tetrahedron")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/primitives/dim3.rs.html#1215)

### impl [Measured3d](../../prelude/trait.Measured3d.html "trait bevy::prelude::Measured3d") for [Torus](../../prelude/struct.Torus.html "struct bevy::prelude::Torus")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/primitives/dim3.rs.html#1565)

### impl<T> [Measured3d](../../prelude/trait.Measured3d.html "trait bevy::prelude::Measured3d") for [Extrusion](../../prelude/struct.Extrusion.html "struct bevy::prelude::Extrusion")<T>

where T: [Primitive2d](../../prelude/trait.Primitive2d.html "trait bevy::prelude::Primitive2d") + [Measured2d](../../prelude/trait.Measured2d.html "trait bevy::prelude::Measured2d"),