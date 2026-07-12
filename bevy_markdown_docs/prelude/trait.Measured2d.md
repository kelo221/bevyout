[bevy](../index.html)::[prelude](index.html)

# Trait Measured2d 

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/primitives/mod.rs.html#39)

```rust
pub trait Measured2d {
    // Required methods
    fn perimeter(&self) -> f32;
    fn area(&self) -> f32;
}
```

A trait for getting measurements of 2D shapes

## Required Methods

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/primitives/mod.rs.html#41)

#### fn [perimeter](#tymethod.perimeter)(&self) -> [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)

Get the perimeter of the shape

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/primitives/mod.rs.html#44)

#### fn [area](#tymethod.area)(&self) -> [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)

Get the area of the shape

## Dyn Compatibility

This trait **is** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/primitives/dim2.rs.html#1032)

### impl [Measured2d](trait.Measured2d.html "trait bevy::prelude::Measured2d") for [Annulus](struct.Annulus.html "struct bevy::prelude::Annulus")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/primitives/dim2.rs.html#2227)

### impl [Measured2d](trait.Measured2d.html "trait bevy::prelude::Measured2d") for [Capsule2d](struct.Capsule2d.html "struct bevy::prelude::Capsule2d")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/primitives/dim2.rs.html#83)

### impl [Measured2d](trait.Measured2d.html "trait bevy::prelude::Measured2d") for [Circle](struct.Circle.html "struct bevy::prelude::Circle")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/primitives/dim2.rs.html#307)

### impl [Measured2d](trait.Measured2d.html "trait bevy::prelude::Measured2d") for [CircularSector](struct.CircularSector.html "struct bevy::prelude::CircularSector")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/primitives/dim2.rs.html#459)

### impl [Measured2d](trait.Measured2d.html "trait bevy::prelude::Measured2d") for [CircularSegment](struct.CircularSegment.html "struct bevy::prelude::CircularSegment")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/primitives/dim2.rs.html#886)

### impl [Measured2d](trait.Measured2d.html "trait bevy::prelude::Measured2d") for [Ellipse](struct.Ellipse.html "struct bevy::prelude::Ellipse")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/primitives/dim3.rs.html#162)

### impl [Measured2d](trait.Measured2d.html "trait bevy::prelude::Measured2d") for [Plane3d](struct.Plane3d.html "struct bevy::prelude::Plane3d")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/primitives/dim2.rs.html#1874)

### impl [Measured2d](trait.Measured2d.html "trait bevy::prelude::Measured2d") for [Rectangle](struct.Rectangle.html "struct bevy::prelude::Rectangle")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/primitives/dim2.rs.html#2160)

### impl [Measured2d](trait.Measured2d.html "trait bevy::prelude::Measured2d") for [RegularPolygon](struct.RegularPolygon.html "struct bevy::prelude::RegularPolygon")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/primitives/dim2.rs.html#1172)

### impl [Measured2d](trait.Measured2d.html "trait bevy::prelude::Measured2d") for [Rhombus](struct.Rhombus.html "struct bevy::prelude::Rhombus")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/primitives/dim2.rs.html#1775)

### impl [Measured2d](trait.Measured2d.html "trait bevy::prelude::Measured2d") for [Triangle2d](struct.Triangle2d.html "struct bevy::prelude::Triangle2d")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/primitives/dim3.rs.html#1410)

### impl [Measured2d](trait.Measured2d.html "trait bevy::prelude::Measured2d") for [Triangle3d](struct.Triangle3d.html "struct bevy::prelude::Triangle3d")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/primitives/dim2.rs.html#2288)

### impl<P> [Measured2d](trait.Measured2d.html "trait bevy::prelude::Measured2d") for [Ring](struct.Ring.html "struct bevy::prelude::Ring")<P>

where P: [Primitive2d](trait.Primitive2d.html "trait bevy::prelude::Primitive2d") + [Measured2d](trait.Measured2d.html "trait bevy::prelude::Measured2d"),