[bevy](../../../index.html)::[material](../../index.html)::[bind\_group\_layout\_entries](../index.html)::[binding\_types](index.html)

# Function storage\_buffer\_sized 

[Source](https://docs.rs/bevy_material/0.19.0/x86_64-unknown-linux-gnu/src/bevy_material/bind_group_layout_entries.rs.html#380-383)

```rust
pub fn storage_buffer_sized(
    has_dynamic_offset: bool,
    min_binding_size: Option<NonZero<u64>>,
) -> BindGroupLayoutEntryBuilder
```