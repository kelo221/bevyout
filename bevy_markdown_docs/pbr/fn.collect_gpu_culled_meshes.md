[bevy](../index.html)::[pbr](index.html)

# Function collect\_gpu\_culled\_meshes 

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/mesh.rs.html#2231-2235)

```rust
pub fn collect_gpu_culled_meshes(
    cameras: Query<'_, '_, (Option<&RenderLayers>, &mut RenderVisibleEntities), With<ExtractedView>>,
    lights: Query<'_, '_, (Option<&RenderLayers>, &mut RenderShadowMapVisibleEntities)>,
    render_gpu_culled_entities: ResMut<'_, RenderGpuCulledEntities>,
)
```

Transfers entities from [`RenderGpuCulledEntities`](struct.RenderGpuCulledEntities.html "struct bevy::pbr::RenderGpuCulledEntities") to the [`RenderVisibleEntities`](../render/view/struct.RenderVisibleEntities.html "struct bevy::render::view::RenderVisibleEntities") and [`RenderShadowMapVisibleEntities`](../render/view/struct.RenderShadowMapVisibleEntities.html "struct bevy::render::view::RenderShadowMapVisibleEntities") components on each view.

Each view must maintain a separate list of GPU-culled entities because the views and entities might belong to different render layers.