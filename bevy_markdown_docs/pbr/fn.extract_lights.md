[bevy](../index.html)::[pbr](index.html)

# Function extract\_lights 

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/light.rs.html#319-424)

```rust
pub fn extract_lights(
    commands: Commands<'_, '_>,
    point_light_shadow_map: Extract<'_, '_, Res<'_, PointLightShadowMap>>,
    directional_light_shadow_map: Extract<'_, '_, Res<'_, DirectionalLightShadowMap>>,
    point_lights: Extract<'_, '_, Query<'_, '_, (Entity, RenderEntity, &PointLight, &CubemapVisibleEntities, &GlobalTransform, &ViewVisibility, &CubemapFrusta, Option<&VolumetricLight>), Or<(Changed<PointLight>, Changed<CubemapVisibleEntities>, Changed<GlobalTransform>, Changed<ViewVisibility>, Changed<CubemapFrusta>, Changed<VolumetricLight>)>>>,
    spot_lights: Extract<'_, '_, Query<'_, '_, (Entity, RenderEntity, &SpotLight, &VisibleMeshEntities, &GlobalTransform, &ViewVisibility, &Frustum, Option<&VolumetricLight>), Or<(Changed<SpotLight>, Changed<VisibleMeshEntities>, Changed<GlobalTransform>, Changed<ViewVisibility>, Changed<Frustum>, Changed<VolumetricLight>)>>>,
    directional_lights: Extract<'_, '_, Query<'_, '_, (Entity, RenderEntity, &DirectionalLight, &CascadesVisibleEntities, &Cascades, &CascadeShadowConfig, &CascadesFrusta, &GlobalTransform, &ViewVisibility, Option<&RenderLayers>, Option<&VolumetricLight>, Has<OcclusionCulling>, Option<&SunDisk>), (Without<SpotLight>, Or<(Changed<DirectionalLight>, Changed<CascadesVisibleEntities>, Changed<Cascades>, Changed<CascadeShadowConfig>, Changed<CascadesFrusta>, Changed<GlobalTransform>, Changed<ViewVisibility>, Changed<RenderLayers>, Changed<VolumetricLight>, Changed<OcclusionCulling>, Changed<SunDisk>)>)>>,
    rect_lights: Extract<'_, '_, Query<'_, '_, (Entity, RenderEntity, &RectLight, &GlobalTransform, &ViewVisibility), Or<(Changed<RectLight>, Changed<GlobalTransform>, Changed<ViewVisibility>)>>>,
    visibility_extraction_system_param: VisibilityExtractionSystemParam<'_, '_>,
    existing_render_shadow_map_visible_entities: Query<'_, '_, (&mut RenderExtractedShadowMapVisibleEntities, &mut RenderShadowMapVisibleEntities)>,
    rect_light_missing_luts_warning_emitted: Local<'_, bool>,
)
```