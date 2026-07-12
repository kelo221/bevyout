[bevy](../../index.html)::[render](../index.html)::[render\_resource](index.html)

# Function create\_bindless\_bind\_group\_layout\_entries 

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/render_resource/bindless.rs.html#225-230)

```rust
pub fn create_bindless_bind_group_layout_entries(
    bindless_index_table_length: u32,
    bindless_slab_resource_limit: u32,
    bindless_index_table_binding_number: BindingNumber,
    used_resource_types: &[BindlessResourceType],
) -> Vec<BindGroupLayoutEntry>
```

Creates the bind group layout entries common to all shaders that use bindless bind groups.

`used_resource_types` limits which binding arrays are created, reducing argument buffer slot usage on constrained platforms.