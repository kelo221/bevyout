[bevy](../index.html)::[pbr](index.html)

# Function check\_views\_need\_specialization 

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/mesh.rs.html#358-389)

```rust
pub fn check_views_need_specialization(
    view_key_cache: ResMut<'_, ViewKeyCache>,
    dirty_specializations: ResMut<'_, DirtySpecializations>,
    views: Query<'_, '_, (&ExtractedView, Option<&ExtractedCamera>, &Msaa, (Option<&Tonemapping>, Option<&DebandDither>), Option<&ShadowFilteringMethod>, Has<ScreenSpaceAmbientOcclusion>, (Has<NormalPrepass>, Has<DepthPrepass>, Has<MotionVectorPrepass>, Has<DeferredPrepass>), Option<&ScreenSpaceTransmission>, Has<TemporalJitter>, Option<&Projection>, Has<DistanceFog>, (Has<RenderViewLightProbes<EnvironmentMapLight>>, Has<RenderViewLightProbes<IrradianceVolume>>), (Has<OrderIndependentTransparencySettings>, Has<ExtractedAtmosphere>, Has<ScreenSpaceReflectionsUniform>, Has<ViewContactShadowsUniformOffset>))>,
)
```