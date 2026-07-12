[bevy](../index.html)::[pbr](index.html)

# Function skins\_use\_uniform\_buffers 

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/skin.rs.html#171)

```rust
pub fn skins_use_uniform_buffers(limits: &Limits) -> bool
```

Returns true if skinning must use uniforms (and dynamic offsets) because storage buffers aren’t supported on the current platform.