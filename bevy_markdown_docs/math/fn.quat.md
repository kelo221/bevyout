[bevy](../index.html)::[math](index.html)

# Function quat 

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/quat.rs.html#36)

```rust
pub const fn quat(x: f32, y: f32, z: f32, w: f32) -> Quat
```

Creates a quaternion from `x`, `y`, `z` and `w` values.

This should generally not be called manually unless you know what you are doing. Use one of the other constructors instead such as `identity` or `from_axis_angle`.