[bevy](../../index.html)::[pbr](../index.html)::[wireframe](index.html)

# Function wireframe\_3d 

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/wireframe.rs.html#796-806)

```rust
pub fn wireframe_3d(
    world: &World,
    view: ViewQuery<'_, '_, (&ExtractedCamera, &ExtractedView, &ViewTarget, &ViewDepthTexture)>,
    wireframe_phases: Res<'_, ViewBinnedRenderPhases<Wireframe3d>>,
    ctx: RenderContext<'_, '_>,
)
```