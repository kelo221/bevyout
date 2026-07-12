[bevy](../../index.html)::[core\_pipeline](../index.html)::[mip\_generation](index.html)

# Function can\_combine\_downsampling\_bind\_groups 

[Source](https://docs.rs/bevy_core_pipeline/0.19.0/x86_64-unknown-linux-gnu/src/bevy_core_pipeline/mip_generation/mod.rs.html#942-945)

```rust
pub fn can_combine_downsampling_bind_groups(
    render_adapter: &RenderAdapter,
    render_device: &RenderDevice,
) -> bool
```

Returns true if the current platform can use a single bind group for single-pass downsampling.

If this platform must use two separate bind groups, one for each pass, this function returns false.