[bevy](../index.html)::[ui\_render](index.html)

# Function extract\_ui\_camera\_view 

[Source](https://docs.rs/bevy_ui_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_render/lib.rs.html#776-793)

```rust
pub fn extract_ui_camera_view(
    commands: Commands<'_, '_>,
    transparent_render_phases: ResMut<'_, ViewSortedRenderPhases<TransparentUi>>,
    query: Extract<'_, '_, Query<'_, '_, (Entity, RenderEntity, &Camera, Option<&UiAntiAlias>, Option<&BoxShadowSamples>), Or<(With<Camera2d>, With<Camera3d>)>>>,
    main_pass_formats: Res<'_, CameraMainPassTextureFormats>,
    live_entities: Local<'_, HashSet<RetainedViewEntity>>,
)
```

Extracts all UI elements associated with a camera into the render world.