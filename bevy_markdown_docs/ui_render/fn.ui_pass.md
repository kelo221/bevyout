[bevy](../index.html)::[ui\_render](index.html)

# Function ui\_pass 

[Source](https://docs.rs/bevy_ui_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_render/render_pass.rs.html#24-31)

```rust
pub fn ui_pass(
    world: &World,
    view: ViewQuery<'_, '_, &UiCameraView>,
    ui_view_query: Query<'_, '_, (&ExtractedView, &UiViewTarget)>,
    ui_view_target_query: Query<'_, '_, (&ViewTarget, &ExtractedCamera)>,
    transparent_render_phases: Res<'_, ViewSortedRenderPhases<TransparentUi>>,
    ctx: RenderContext<'_, '_>,
)
```