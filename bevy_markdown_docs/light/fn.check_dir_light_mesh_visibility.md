[bevy](../index.html)::[light](index.html)

# Function check\_dir\_light\_mesh\_visibility 

[Source](https://docs.rs/bevy_light/0.19.0/x86_64-unknown-linux-gnu/src/bevy_light/lib.rs.html#336-368)

```rust
pub fn check_dir_light_mesh_visibility(
    commands: Commands<'_, '_>,
    directional_lights: Query<'_, '_, (&DirectionalLight, &CascadesFrusta, &mut CascadesVisibleEntities, Option<&RenderLayers>, &ViewVisibility), Without<SpotLight>>,
    visible_entity_query: Query<'_, '_, (Entity, &InheritedVisibility, Option<&RenderLayers>, Option<&Aabb>, Option<&GlobalTransform>, Has<VisibilityRange>, Has<NoFrustumCulling>), (Without<NotShadowCaster>, Without<DirectionalLight>, Without<NoCpuCulling>, With<Mesh3d>)>,
    visible_entity_ranges: Option<Res<'_, VisibleEntityRanges>>,
    defer_visible_entities_queue: Local<'_, Parallel<Vec<Entity>>>,
    view_visible_entities_queue: Local<'_, Parallel<Vec<Vec<Entity>>>>,
)
```

Updates the visibility for [`DirectionalLight`](../prelude/struct.DirectionalLight.html "struct bevy::prelude::DirectionalLight")s so that shadow map rendering can work.

This only processes entities without [`NoCpuCulling`](../camera/visibility/struct.NoCpuCulling.html "struct bevy::camera::visibility::NoCpuCulling"). Entities with [`NoCpuCulling`](../camera/visibility/struct.NoCpuCulling.html "struct bevy::camera::visibility::NoCpuCulling") receive no view-specific processing in the main world.