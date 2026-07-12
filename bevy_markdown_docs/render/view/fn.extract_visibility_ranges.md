[bevy](../../index.html)::[render](../index.html)::[view](index.html)

# Function extract\_visibility\_ranges 

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/view/visibility/range.rs.html#203-208)

```rust
pub fn extract_visibility_ranges(
    render_visibility_ranges: ResMut<'_, RenderVisibilityRanges>,
    visibility_ranges_query: Extract<'_, '_, Query<'_, '_, (Entity, &VisibilityRange)>>,
    changed_ranges_query: Extract<'_, '_, Query<'_, '_, Entity, Changed<VisibilityRange>>>,
    removed_visibility_ranges: Extract<'_, '_, RemovedComponents<'_, '_, VisibilityRange>>,
)
```

Extracts all [`VisibilityRange`](../../camera/visibility/struct.VisibilityRange.html "struct bevy::camera::visibility::VisibilityRange") components from the main world to the render world and inserts them into [`RenderVisibilityRanges`](struct.RenderVisibilityRanges.html "struct bevy::render::view::RenderVisibilityRanges").