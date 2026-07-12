[bevy](../../../../index.html)::[core\_pipeline](../../../index.html)::[mip\_generation](../../index.html)::[experimental](../index.html)::[depth](index.html)

# Function create\_depth\_pyramid\_dummy\_texture 

[Source](https://docs.rs/bevy_core_pipeline/0.19.0/x86_64-unknown-linux-gnu/src/bevy_core_pipeline/mip_generation/experimental/depth.rs.html#500-504)

```rust
pub fn create_depth_pyramid_dummy_texture(
    render_device: &RenderDevice,
    texture_label: &'static str,
    texture_view_label: &'static str,
) -> TextureView
```

Creates a placeholder texture that can be bound to a depth pyramid binding if no depth pyramid is needed.