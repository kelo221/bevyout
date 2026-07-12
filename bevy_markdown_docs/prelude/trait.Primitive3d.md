[bevy](../index.html)::[prelude](index.html)

# Trait Primitive3d 

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/primitives/mod.rs.html#21)

```rust
pub trait Primitive3d { }
```

A marker trait for 3D primitives

## Dyn Compatibility

This trait **is** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/primitives/dim3.rs.html#870)

### impl [Primitive3d](trait.Primitive3d.html "trait bevy::prelude::Primitive3d") for [Capsule3d](struct.Capsule3d.html "struct bevy::prelude::Capsule3d")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/primitives/dim3.rs.html#941)

### impl [Primitive3d](trait.Primitive3d.html "trait bevy::prelude::Primitive3d") for [Cone](struct.Cone.html "struct bevy::prelude::Cone")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/primitives/dim3.rs.html#1026)

### impl [Primitive3d](trait.Primitive3d.html "trait bevy::prelude::Primitive3d") for [ConicalFrustum](struct.ConicalFrustum.html "struct bevy::prelude::ConicalFrustum")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/primitives/dim3.rs.html#696)

### impl [Primitive3d](trait.Primitive3d.html "trait bevy::prelude::Primitive3d") for [Cuboid](struct.Cuboid.html "struct bevy::prelude::Cuboid")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/primitives/dim3.rs.html#791)

### impl [Primitive3d](trait.Primitive3d.html "trait bevy::prelude::Primitive3d") for [Cylinder](struct.Cylinder.html "struct bevy::prelude::Cylinder")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/direction.rs.html#408)

### impl [Primitive3d](trait.Primitive3d.html "trait bevy::prelude::Primitive3d") for [Dir3](struct.Dir3.html "struct bevy::prelude::Dir3")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/direction.rs.html#812)

### impl [Primitive3d](trait.Primitive3d.html "trait bevy::prelude::Primitive3d") for [Dir3A](struct.Dir3A.html "struct bevy::prelude::Dir3A")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/primitives/dim3.rs.html#192)

### impl [Primitive3d](trait.Primitive3d.html "trait bevy::prelude::Primitive3d") for [InfinitePlane3d](struct.InfinitePlane3d.html "struct bevy::prelude::InfinitePlane3d")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/primitives/dim3.rs.html#369)

### impl [Primitive3d](trait.Primitive3d.html "trait bevy::prelude::Primitive3d") for [Line3d](struct.Line3d.html "struct bevy::prelude::Line3d")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/primitives/dim3.rs.html#110)

### impl [Primitive3d](trait.Primitive3d.html "trait bevy::prelude::Primitive3d") for [Plane3d](struct.Plane3d.html "struct bevy::prelude::Plane3d")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/primitives/dim3.rs.html#637)

### impl [Primitive3d](trait.Primitive3d.html "trait bevy::prelude::Primitive3d") for [Polyline3d](struct.Polyline3d.html "struct bevy::prelude::Polyline3d")

Available on **crate feature `alloc`** only.

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/primitives/dim3.rs.html#389)

### impl [Primitive3d](trait.Primitive3d.html "trait bevy::prelude::Primitive3d") for [Segment3d](struct.Segment3d.html "struct bevy::prelude::Segment3d")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/primitives/dim3.rs.html#35)

### impl [Primitive3d](trait.Primitive3d.html "trait bevy::prelude::Primitive3d") for [Sphere](struct.Sphere.html "struct bevy::prelude::Sphere")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/primitives/dim3.rs.html#1445)

### impl [Primitive3d](trait.Primitive3d.html "trait bevy::prelude::Primitive3d") for [Tetrahedron](struct.Tetrahedron.html "struct bevy::prelude::Tetrahedron")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/primitives/dim3.rs.html#1144)

### impl [Primitive3d](trait.Primitive3d.html "trait bevy::prelude::Primitive3d") for [Torus](struct.Torus.html "struct bevy::prelude::Torus")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/primitives/dim3.rs.html#1248)

### impl [Primitive3d](trait.Primitive3d.html "trait bevy::prelude::Primitive3d") for [Triangle3d](struct.Triangle3d.html "struct bevy::prelude::Triangle3d")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/primitives/dim3.rs.html#1553)

### impl<T> [Primitive3d](trait.Primitive3d.html "trait bevy::prelude::Primitive3d") for [Extrusion](struct.Extrusion.html "struct bevy::prelude::Extrusion")<T>

where T: [Primitive2d](trait.Primitive2d.html "trait bevy::prelude::Primitive2d"),