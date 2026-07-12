[bevy](../../index.html)::[math](../index.html)

# Module sampling 

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/lib.rs.html#49)

Available on **crate feature `rand`** only.

This module contains tools related to random sampling.

To use this, the “rand” feature must be enabled.

## Modules

[mesh\_sampling](mesh_sampling/index.html "mod bevy::math::sampling::mesh_sampling")`alloc`

Functionality related to random sampling from triangle meshes.

[shape\_sampling](shape_sampling/index.html "mod bevy::math::sampling::shape_sampling")

The [`ShapeSample`](../../prelude/trait.ShapeSample.html "trait bevy::prelude::ShapeSample") trait, allowing random sampling from geometric shapes.

[standard](standard/index.html "mod bevy::math::sampling::standard")

This module holds local implementations of the [`Distribution`](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/distr/distribution/trait.Distribution.html "trait rand::distr::distribution::Distribution") trait for [`StandardUniform`](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/distr/struct.StandardUniform.html "struct rand::distr::StandardUniform"), which allow certain Bevy math types (those whose values can be randomly generated without additional input other than an [`RngExt`](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/rng/trait.RngExt.html "trait rand::rng::RngExt")) to be produced using [`rand`](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/index.html "mod rand")’s APIs. It also holds [`FromRng`](../../prelude/trait.FromRng.html "trait bevy::prelude::FromRng"), an ergonomic extension to that functionality which permits the omission of type annotations.

## Structs

[BoundaryOf](struct.BoundaryOf.html "struct bevy::math::sampling::BoundaryOf")

A wrapper struct that allows boundary sampling from a [`ShapeSample`](../../prelude/trait.ShapeSample.html "trait bevy::prelude::ShapeSample") type directly as a [`Distribution`](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/distr/distribution/trait.Distribution.html "trait rand::distr::distribution::Distribution").

[InteriorOf](struct.InteriorOf.html "struct bevy::math::sampling::InteriorOf")

A wrapper struct that allows interior sampling from a [`ShapeSample`](../../prelude/trait.ShapeSample.html "trait bevy::prelude::ShapeSample") type directly as a [`Distribution`](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/distr/distribution/trait.Distribution.html "trait rand::distr::distribution::Distribution").

[UniformMeshSampler](struct.UniformMeshSampler.html "struct bevy::math::sampling::UniformMeshSampler")

A [distribution](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/distr/distribution/trait.Distribution.html "trait rand::distr::distribution::Distribution") that caches data to allow fast sampling from a collection of triangles. Generally used through [`sample`](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/distr/distribution/trait.Distribution.html#tymethod.sample "method rand::distr::distribution::Distribution::sample") or [`sample_iter`](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/distr/distribution/trait.Distribution.html#method.sample_iter "method rand::distr::distribution::Distribution::sample_iter").

## Traits

[FromRng](trait.FromRng.html "trait bevy::math::sampling::FromRng")

Ergonomics trait for a type with a [`StandardUniform`](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/distr/struct.StandardUniform.html "struct rand::distr::StandardUniform") distribution, allowing values to be generated uniformly from an [`RngExt`](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/rng/trait.RngExt.html "trait rand::rng::RngExt") by a method in its own namespace.

[ShapeSample](trait.ShapeSample.html "trait bevy::math::sampling::ShapeSample")

Exposes methods to uniformly sample a variety of primitive shapes.