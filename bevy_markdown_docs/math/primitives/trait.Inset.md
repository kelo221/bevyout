[bevy](../../index.html)::[math](../index.html)::[primitives](index.html)

# Trait Inset 

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/primitives/inset.rs.html#15)

```rust
pub trait Inset: Primitive2d {
    // Required method
    fn inset(self, distance: f32) -> Self;
}
```

A primitive that can be resized uniformly.

See documentation on [`Inset::inset`](../../prelude/trait.Inset.html#tymethod.inset "method bevy::prelude::Inset::inset").

See also [`ToRing`](../../prelude/trait.ToRing.html "trait bevy::prelude::ToRing").

## Required Methods

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/primitives/inset.rs.html#23)

#### fn [inset](#tymethod.inset)(self, distance: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> Self

Create a new version of this primitive that is resized uniformly. That is, it resizes the shape inwards such that for the lines between vertices, it creates new parallel lines that are `distance` inwards from the original lines.

This is useful for creating smaller shapes or making outlines of `distance` thickness with [`Ring`](../../prelude/struct.Ring.html "struct bevy::prelude::Ring").

See also [`ToRing::to_ring`](../../prelude/trait.ToRing.html#tymethod.to_ring "method bevy::prelude::ToRing::to_ring")

## Dyn Compatibility

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/primitives/inset.rs.html#66)

### impl [Inset](../../prelude/trait.Inset.html "trait bevy::prelude::Inset") for [Capsule2d](../../prelude/struct.Capsule2d.html "struct bevy::prelude::Capsule2d")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/primitives/inset.rs.html#26)

### impl [Inset](../../prelude/trait.Inset.html "trait bevy::prelude::Inset") for [Circle](../../prelude/struct.Circle.html "struct bevy::prelude::Circle")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/primitives/inset.rs.html#80)

### impl [Inset](../../prelude/trait.Inset.html "trait bevy::prelude::Inset") for [CircularSegment](../../prelude/struct.CircularSegment.html "struct bevy::prelude::CircularSegment")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/primitives/inset.rs.html#73)

### impl [Inset](../../prelude/trait.Inset.html "trait bevy::prelude::Inset") for [Rectangle](../../prelude/struct.Rectangle.html "struct bevy::prelude::Rectangle")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/primitives/inset.rs.html#91)

### impl [Inset](../../prelude/trait.Inset.html "trait bevy::prelude::Inset") for [RegularPolygon](../../prelude/struct.RegularPolygon.html "struct bevy::prelude::RegularPolygon")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/primitives/inset.rs.html#55)

### impl [Inset](../../prelude/trait.Inset.html "trait bevy::prelude::Inset") for [Rhombus](../../prelude/struct.Rhombus.html "struct bevy::prelude::Rhombus")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/primitives/inset.rs.html#33)

### impl [Inset](../../prelude/trait.Inset.html "trait bevy::prelude::Inset") for [Triangle2d](../../prelude/struct.Triangle2d.html "struct bevy::prelude::Triangle2d")