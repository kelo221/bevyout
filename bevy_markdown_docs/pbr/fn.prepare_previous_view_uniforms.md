[bevy](../index.html)::[pbr](index.html)

# Function prepare\_previous\_view\_uniforms 

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/prepass/mod.rs.html#667-676)

```rust
pub fn prepare_previous_view_uniforms(
    commands: Commands<'_, '_>,
    render_device: Res<'_, RenderDevice>,
    render_queue: Res<'_, RenderQueue>,
    previous_view_uniforms: ResMut<'_, PreviousViewUniforms>,
    views: Query<'_, '_, (Entity, &ExtractedView, Option<&PreviousViewData>), Or<(With<Camera3d>, With<ShadowView>)>>,
)
```