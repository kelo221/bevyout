[bevy](../index.html)::[math](index.html)

# Function dquat 

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dquat.rs.html#22)

```rust
pub const fn dquat(x: f64, y: f64, z: f64, w: f64) -> DQuat
```

Creates a quaternion from `x`, `y`, `z` and `w` values.

This should generally not be called manually unless you know what you are doing. Use one of the other constructors instead such as `identity` or `from_axis_angle`.