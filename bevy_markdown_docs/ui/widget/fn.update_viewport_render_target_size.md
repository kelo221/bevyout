[bevy](../../index.html)::[ui](../index.html)::[widget](index.html)

# Function update\_viewport\_render\_target\_size 

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/widget/viewport.rs.html#166-173)

```rust
pub fn update_viewport_render_target_size(
    viewport_query: Query<'_, '_, (&mut ViewportNode, &ComputedNode), Or<(Changed<ComputedNode>, Changed<ViewportNode>)>>,
    camera_query: Query<'_, '_, &RenderTarget>,
    images: ResMut<'_, Assets<Image>>,
)
```

Updates the size of the associated render target for viewports when the node size changes.