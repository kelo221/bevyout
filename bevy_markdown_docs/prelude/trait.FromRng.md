[bevy](../index.html)::[prelude](index.html)

# Trait FromRng 

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/sampling/standard.rs.html#45)

```rust
pub trait FromRng: Sizedwhere
    StandardUniform: Distribution<Self>,{
    // Provided method
    fn from_rng<R>(rng: &mut R) -> Self
       where R: RngExt + ?Sized { ... }
}
```

Ergonomics trait for a type with a [`StandardUniform`](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/distr/struct.StandardUniform.html "struct rand::distr::StandardUniform") distribution, allowing values to be generated uniformly from an [`RngExt`](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/rng/trait.RngExt.html "trait rand::rng::RngExt") by a method in its own namespace.

Example

```rust
let mut rng = StdRng::seed_from_u64(451);
let random_dir = Dir3::from_rng(&mut rng);
```

## Provided Methods

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/sampling/standard.rs.html#51)

#### fn [from\_rng](#method.from_rng)<R>(rng: [&mut R](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> Self

where R: [RngExt](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/rng/trait.RngExt.html "trait rand::rng::RngExt") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Construct a value of this type uniformly at random using `rng` as the source of randomness.

## Dyn Compatibility

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/sampling/standard.rs.html#65)

### impl [FromRng](trait.FromRng.html "trait bevy::prelude::FromRng") for [Dir2](struct.Dir2.html "struct bevy::prelude::Dir2")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/sampling/standard.rs.html#76)

### impl [FromRng](trait.FromRng.html "trait bevy::prelude::FromRng") for [Dir3](struct.Dir3.html "struct bevy::prelude::Dir3")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/sampling/standard.rs.html#87)

### impl [FromRng](trait.FromRng.html "trait bevy::prelude::FromRng") for [Dir3A](struct.Dir3A.html "struct bevy::prelude::Dir3A")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/sampling/standard.rs.html#99)

### impl [FromRng](trait.FromRng.html "trait bevy::prelude::FromRng") for [Quat](struct.Quat.html "struct bevy::prelude::Quat")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/sampling/standard.rs.html#97)

### impl [FromRng](trait.FromRng.html "trait bevy::prelude::FromRng") for [Rot2](struct.Rot2.html "struct bevy::prelude::Rot2")