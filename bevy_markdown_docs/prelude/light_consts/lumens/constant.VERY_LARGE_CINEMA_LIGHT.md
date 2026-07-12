[bevy](../../../index.html)::[prelude](../../index.html)::[light\_consts](../index.html)::[lumens](index.html)

# Constant VERY\_LARGE\_CINEMA\_LIGHT 

[Source](https://docs.rs/bevy_light/0.19.0/x86_64-unknown-linux-gnu/src/bevy_light/lib.rs.html#117)

```rust
pub const VERY_LARGE_CINEMA_LIGHT: f32 = 1_000_000.0; // 1.0E+6f32
```

1,000,000 lumens is a very large “cinema light” capable of registering brightly at Bevy’s default [`bevy_camera::Exposure::BLENDER`](../../../camera/struct.Exposure.html#associatedconstant.BLENDER "associated constant bevy::camera::Exposure::BLENDER") exposure level. For “indoor lighting” with a lower exposure, this would be way too bright.