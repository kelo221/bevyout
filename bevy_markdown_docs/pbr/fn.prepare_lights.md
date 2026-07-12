[bevy](../index.html)::[pbr](index.html)

# Function prepare\_lights 

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/light.rs.html#981-1050)

```rust
pub fn prepare_lights(
    commands: Commands<'_, '_>,
    texture_cache: ResMut<'_, TextureCache>,
    _: (Res<'_, RenderDevice>, Res<'_, RenderQueue>),
    global_clusterable_object_meta: ResMut<'_, GlobalClusterableObjectMeta>,
    light_meta: ResMut<'_, LightMeta>,
    views: Query<'_, '_, (Entity, MainEntity, &ExtractedView, &ExtractedClusterConfig, Option<&RenderLayers>, Has<NoIndirectDrawing>, Option<&AmbientLight>), With<Camera3d>>,
    ambient_light: Res<'_, GlobalAmbientLight>,
    point_light_shadow_map: Res<'_, PointLightShadowMap>,
    directional_light_shadow_map: Res<'_, DirectionalLightShadowMap>,
    shadow_render_phases: ResMut<'_, ViewBinnedRenderPhases<Shadow>>,
    _: (Local<'_, bool>, Local<'_, bool>, Local<'_, bool>, Local<'_, HashSet<RetainedViewEntity>>),
    _: (Query<'_, '_, (Entity, &MainEntity, &ExtractedPointLight, &mut PointAndSpotLightViewEntities, AnyOf<(&CubemapFrusta, &Frustum)>)>, Query<'_, '_, (), Or<(Changed<ExtractedPointLight>, Changed<CubemapFrusta>, Changed<Frustum>)>>, Query<'_, '_, (Entity, &MainEntity, &ExtractedDirectionalLight)>, Query<'_, '_, (Entity, &MainEntity, &ExtractedRectLight)>, Query<'_, '_, &mut DirectionalLightViewEntities>),
    sorted_cameras: Res<'_, SortedCameras>,
    _: (Res<'_, GpuPreprocessingSupport>, Option<Res<'_, RenderClusteredDecals>>),
    _: (Query<'_, '_, &ShadowView>, ResMut<'_, LightKeyCache>, ResMut<'_, SpecializedShadowMaterialPipelineCache>),
)
```