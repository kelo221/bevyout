[bevy](../../index.html)::[pbr](../index.html)::[deferred](index.html)

# Function prepare\_deferred\_lighting\_pipelines 

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/deferred/mod.rs.html#392-422)

```rust
pub fn prepare_deferred_lighting_pipelines(
    commands: Commands<'_, '_>,
    pipeline_cache: Res<'_, PipelineCache>,
    pipelines: ResMut<'_, SpecializedRenderPipelines<DeferredLightingLayout>>,
    deferred_lighting_layout: Res<'_, DeferredLightingLayout>,
    cameras: Query<'_, '_, (Entity, &ExtractedCamera, &ExtractedView, Option<&Tonemapping>, Option<&DebandDither>, Option<&ShadowFilteringMethod>, (Has<ScreenSpaceAmbientOcclusion>, Has<ScreenSpaceReflectionsUniform>, Has<DistanceFog>), (Has<NormalPrepass>, Has<DepthPrepass>, Has<MotionVectorPrepass>, Has<DeferredPrepass>), Has<RenderViewLightProbes<EnvironmentMapLight>>, Has<RenderViewLightProbes<IrradianceVolume>>, Option<&ScreenSpaceTransmission>, Has<OrderIndependentTransparencySettingsOffset>, Has<SkipDeferredLighting>, Has<ExtractedAtmosphere>)>,
)
```