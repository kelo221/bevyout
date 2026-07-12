[bevy](../../index.html)::[animation](../index.html)::[prelude](index.html)

# Type Alias AnimationMask 

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/graph.rs.html#426)

```rust
pub type AnimationMask = u64;
```

The type of an animation mask bitfield.

Bit N corresponds to mask group N.

Because this is a 64-bit value, there is currently a limitation of 64 mask groups per animation graph.