[bevy](../../../index.html)::[math](../../index.html)::[sampling](../index.html)

# Module standard 

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/sampling/mod.rs.html#8)

Available on **crate feature `rand`** only.

This module holds local implementations of the [`Distribution`](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/distr/distribution/trait.Distribution.html "trait rand::distr::distribution::Distribution") trait for [`StandardUniform`](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/distr/struct.StandardUniform.html "struct rand::distr::StandardUniform"), which allow certain Bevy math types (those whose values can be randomly generated without additional input other than an [`RngExt`](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/rng/trait.RngExt.html "trait rand::rng::RngExt")) to be produced using [`rand`](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/index.html "mod rand")’s APIs. It also holds [`FromRng`](../../../prelude/trait.FromRng.html "trait bevy::prelude::FromRng"), an ergonomic extension to that functionality which permits the omission of type annotations.

For instance:

```rust
let mut rng = StdRng::seed_from_u64(7313429298);
// Random direction using thread-local rng
let random_direction1: Dir3 = random();

// Random direction using the rng constructed above
let random_direction2: Dir3 = rng.random();

// The same as the previous but with different syntax
let random_direction3 = Dir3::from_rng(&mut rng);

// Five random directions, using StandardUniform explicitly
let many_random_directions: Vec<Dir3> = rng.sample_iter(StandardUniform).take(5).collect();
```

## Traits

[FromRng](trait.FromRng.html "trait bevy::math::sampling::standard::FromRng")

Ergonomics trait for a type with a [`StandardUniform`](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/distr/struct.StandardUniform.html "struct rand::distr::StandardUniform") distribution, allowing values to be generated uniformly from an [`RngExt`](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/rng/trait.RngExt.html "trait rand::rng::RngExt") by a method in its own namespace.