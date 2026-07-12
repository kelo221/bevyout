[bevy](../index.html)::[prelude](index.html)

# Trait ToRing 

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/primitives/dim2.rs.html#2303)

```rust
pub trait ToRing:
    Primitive2d
    + Inset
    + Sized {
    // Required method
    fn to_ring(self, thickness: f32) -> Ring<Self>;
}
```

Provides a convenience method for converting a primitive to a [`Ring`](struct.Ring.html "struct bevy::prelude::Ring"), with a given thickness.

The primitive must implement [`Inset`](trait.Inset.html "trait bevy::prelude::Inset").

## Required Methods

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/primitives/dim2.rs.html#2308)

#### fn [to\_ring](#tymethod.to_ring)(self, thickness: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [Ring](struct.Ring.html "struct bevy::prelude::Ring")<Self>

Construct a `Ring`

## Dyn Compatibility

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/primitives/dim2.rs.html#2311-2313)

### impl<P> [ToRing](trait.ToRing.html "trait bevy::prelude::ToRing") for P

where P: [Primitive2d](trait.Primitive2d.html "trait bevy::prelude::Primitive2d") + [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + [Inset](trait.Inset.html "trait bevy::prelude::Inset"),