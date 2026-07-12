[bevy](../index.html)::[render](index.html)

# Function get\_adreno\_model 

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/lib.rs.html#531)

```rust
pub fn get_adreno_model(adapter_info: &RenderAdapterInfo) -> Option<u32>
```

If the [`RenderAdapterInfo`](renderer/struct.RenderAdapterInfo.html "struct bevy::render::renderer::RenderAdapterInfo") is a Qualcomm Adreno, returns its model number.

This lets us work around hardware bugs.