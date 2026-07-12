[bevy](../../index.html)::[render](../index.html)::[view](index.html)

# Function collect\_visible\_cpu\_culled\_entities 

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/view/visibility/mod.rs.html#331-341)

```rust
pub fn collect_visible_cpu_culled_entities(
    cameras: Query<'_, '_, (&mut RenderVisibleEntities, Option<&mut RenderExtractedVisibleEntities>)>,
    lights: Query<'_, '_, (&mut RenderShadowMapVisibleEntities, Option<&mut RenderExtractedShadowMapVisibleEntities>)>,
    visibility_classes: Local<'_, HashSet<TypeId>>,
)
```

Updates the [`RenderVisibleEntities`](struct.RenderVisibleEntities.html "struct bevy::render::view::RenderVisibleEntities") and [`RenderShadowMapVisibleEntities`](struct.RenderShadowMapVisibleEntities.html "struct bevy::render::view::RenderShadowMapVisibleEntities") components with the contents of the [`RenderExtractedVisibleEntities`](struct.RenderExtractedVisibleEntities.html "struct bevy::render::view::RenderExtractedVisibleEntities") and the [`RenderExtractedShadowMapVisibleEntities`](struct.RenderExtractedShadowMapVisibleEntities.html "struct bevy::render::view::RenderExtractedShadowMapVisibleEntities") components respectively.

This system only handles CPU-culled entities (i.e. those without [`NoCpuCulling`](../../camera/visibility/struct.NoCpuCulling.html "struct bevy::camera::visibility::NoCpuCulling") components). The `collect_gpu_culled_meshes` system in `bevy_pbr` handles GPU-culled entities.