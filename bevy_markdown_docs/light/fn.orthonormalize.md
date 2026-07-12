[bevy](../index.html)::[light](index.html)

# Function orthonormalize 

[Source](https://docs.rs/bevy_light/0.19.0/x86_64-unknown-linux-gnu/src/bevy_light/spot_light.rs.html#172)

```rust
pub fn orthonormalize(z_basis: Dir3) -> Mat3
```

Constructs a right-handed orthonormal basis from a given unit Z vector.

This method of constructing a basis from a [`Vec3`](../prelude/struct.Vec3.html "struct bevy::prelude::Vec3") is used by [`bevy_math::Vec3::any_orthonormal_pair`](../prelude/struct.Vec3.html#method.any_orthonormal_pair "method bevy::prelude::Vec3::any_orthonormal_pair")