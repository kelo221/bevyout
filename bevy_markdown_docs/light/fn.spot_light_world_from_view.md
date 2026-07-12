[bevy](../index.html)::[light](index.html)

# Function spot\_light\_world\_from\_view 

[Source](https://docs.rs/bevy_light/0.19.0/x86_64-unknown-linux-gnu/src/bevy_light/spot_light.rs.html#187)

```rust
pub fn spot_light_world_from_view(transform: &GlobalTransform) -> Affine3A
```

Constructs a right-handed orthonormal basis with translation, using only the forward direction and translation of a given [`GlobalTransform`](../prelude/struct.GlobalTransform.html "struct bevy::prelude::GlobalTransform").

This is a version of [`orthonormalize`](fn.orthonormalize.html "fn bevy::light::orthonormalize") which also includes translation.