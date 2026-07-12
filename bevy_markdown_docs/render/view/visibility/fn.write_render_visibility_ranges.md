[bevy](../../../index.html)::[render](../../index.html)::[view](../index.html)::[visibility](index.html)

# Function write\_render\_visibility\_ranges 

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/view/visibility/range.rs.html#221-225)

```rust
pub fn write_render_visibility_ranges(
    render_device: Res<'_, RenderDevice>,
    render_queue: Res<'_, RenderQueue>,
    render_visibility_ranges: ResMut<'_, RenderVisibilityRanges>,
)
```

Writes the [`RenderVisibilityRanges`](../struct.RenderVisibilityRanges.html "struct bevy::render::view::RenderVisibilityRanges") table to the GPU.