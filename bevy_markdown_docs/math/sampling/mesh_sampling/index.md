[bevy](../../../index.html)::[math](../../index.html)::[sampling](../index.html)

# Module mesh\_sampling 

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/sampling/mod.rs.html#6)

Available on **crate features `alloc` and `rand`** only.

Functionality related to random sampling from triangle meshes.

## Structs

[UniformMeshSampler](struct.UniformMeshSampler.html "struct bevy::math::sampling::mesh_sampling::UniformMeshSampler")

A [distribution](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/distr/distribution/trait.Distribution.html "trait rand::distr::distribution::Distribution") that caches data to allow fast sampling from a collection of triangles. Generally used through [`sample`](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/distr/distribution/trait.Distribution.html#tymethod.sample "method rand::distr::distribution::Distribution::sample") or [`sample_iter`](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/distr/distribution/trait.Distribution.html#method.sample_iter "method rand::distr::distribution::Distribution::sample_iter").