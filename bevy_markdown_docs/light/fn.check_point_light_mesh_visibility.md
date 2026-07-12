[bevy](../index.html)::[light](index.html)

# Function check\_point\_light\_mesh\_visibility 

[Source](https://docs.rs/bevy_light/0.19.0/x86_64-unknown-linux-gnu/src/bevy_light/lib.rs.html#504-545)

```rust
pub fn check_point_light_mesh_visibility(
    visible_point_lights: Query<'_, '_, &VisibleEntities>,
    point_lights: Query<'_, '_, (&PointLight, &GlobalTransform, &CubemapFrusta, &mut CubemapVisibleEntities, Option<&RenderLayers>)>,
    spot_lights: Query<'_, '_, (&SpotLight, &GlobalTransform, &Frustum, &mut VisibleMeshEntities, Option<&RenderLayers>)>,
    visible_entity_query: Query<'_, '_, (Entity, &InheritedVisibility, &mut ViewVisibility, Option<&RenderLayers>, Option<&Aabb>, Option<&GlobalTransform>, Has<VisibilityRange>, Has<NoFrustumCulling>), (Without<NotShadowCaster>, Without<DirectionalLight>, Without<NoCpuCulling>, With<Mesh3d>)>,
    camera_query: Query<'_, '_, (Entity, &RenderTarget), With<Camera>>,
    shadow_lod_origin_query: Query<'_, '_, Entity, With<ShadowLodOrigin>>,
    point_and_spot_light_query: Query<'_, '_, Entity, Or<(With<PointLight>, With<SpotLight>)>>,
    visible_entity_ranges: Option<Res<'_, VisibleEntityRanges>>,
    cubemap_visible_entities_queue: Local<'_, Parallel<[Vec<Entity>; 6]>>,
    spot_visible_entities_queue: Local<'_, Parallel<Vec<Entity>>>,
    checked_lights: Local<'_, EntityHashSet>,
)
```

Updates the visibility for [`PointLight`](../prelude/struct.PointLight.html "struct bevy::prelude::PointLight")s and [`SpotLight`](../prelude/struct.SpotLight.html "struct bevy::prelude::SpotLight")s so that shadow map rendering can work.

This only processes entities without [`NoCpuCulling`](../camera/visibility/struct.NoCpuCulling.html "struct bevy::camera::visibility::NoCpuCulling"). Entities with [`NoCpuCulling`](../camera/visibility/struct.NoCpuCulling.html "struct bevy::camera::visibility::NoCpuCulling") receive no view-specific processing in the main world.