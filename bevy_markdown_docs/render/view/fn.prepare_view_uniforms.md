[bevy](../../index.html)::[render](../index.html)::[view](index.html)

# Function prepare\_view\_uniforms 

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/view/mod.rs.html#998-1014)

```rust
pub fn prepare_view_uniforms(
    commands: Commands<'_, '_>,
    render_device: Res<'_, RenderDevice>,
    render_queue: Res<'_, RenderQueue>,
    view_uniforms: ResMut<'_, ViewUniforms>,
    views: Query<'_, '_, (Entity, Option<&ExtractedCamera>, &ExtractedView, Option<&Frustum>, Option<&TemporalJitter>, Option<&MipBias>, Option<&MainPassResolutionOverride>)>,
    frame_count: Res<'_, FrameCount>,
    shadow_lod_origin: Option<Res<'_, RenderShadowLodOrigin>>,
)
```