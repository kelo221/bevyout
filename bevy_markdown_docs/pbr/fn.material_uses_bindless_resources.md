[bevy](../index.html)::[pbr](index.html)

# Function material\_uses\_bindless\_resources 

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/material_bind_groups.rs.html#1716-1718)

```rust
pub fn material_uses_bindless_resources<M>(render_device: &RenderDevice) -> boolwhere
    M: Material,
```

Returns true if the material will _actually_ use bindless resources or false if it won’t.

This takes the platform support (or lack thereof) for bindless resources into account.