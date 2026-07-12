[bevy](../index.html)::[pbr](index.html)

# Function prepare\_material\_bind\_groups 

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/material.rs.html#1776-1782)

```rust
pub fn prepare_material_bind_groups(
    allocators: ResMut<'_, MaterialBindGroupAllocators>,
    render_device: Res<'_, RenderDevice>,
    pipeline_cache: Res<'_, PipelineCache>,
    fallback_image: Res<'_, FallbackImage>,
    fallback_resources: Res<'_, FallbackBindlessResources>,
)
```

Creates and/or recreates any bind groups that contain materials that were modified this frame.