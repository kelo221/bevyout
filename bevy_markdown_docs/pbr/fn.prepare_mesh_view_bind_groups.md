[bevy](../index.html)::[pbr](index.html)

# Function prepare\_mesh\_view\_bind\_groups 

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/mesh_view_bindings.rs.html#626-704)

```rust
pub fn prepare_mesh_view_bind_groups(
    commands: Commands<'_, '_>,
    _: (Res<'_, RenderDevice>, Res<'_, PipelineCache>, Res<'_, RenderAdapter>),
    mesh_pipeline: Res<'_, MeshPipeline>,
    shadow_samplers: Res<'_, ShadowSamplers>,
    _: (Res<'_, LightMeta>, Res<'_, GlobalClusterableObjectMeta>, Res<'_, FogMeta>, Res<'_, ViewUniforms>),
    views: Query<'_, '_, (Entity, Option<&ExtractedCamera>, &ViewShadowBindings, &ViewClusterBindings, &Msaa, Option<&ScreenSpaceAmbientOcclusionResources>, Option<&ViewPrepassTextures>, Option<&ViewTransmissionTexture>, Option<&AtmosphereTextures>, &Tonemapping, (Option<&RenderViewLightProbes<EnvironmentMapLight>>, Option<&RenderViewLightProbes<IrradianceVolume>>), Has<ExtractedAtmosphere>, (&ViewUniformOffset, &ViewLightsUniformOffset, &ViewLightProbesUniformOffset, Option<&ViewFogUniformOffset>, Option<&ViewScreenSpaceReflectionsUniformOffset>, Option<&ViewContactShadowsUniformOffset>, Option<&OrderIndependentTransparencySettingsOffset>))>,
    _: (Res<'_, RenderAssets<GpuImage>>, Res<'_, FallbackImage>, Res<'_, FallbackImageZero>),
    globals_buffer: Res<'_, GlobalsBuffer>,
    tonemapping_luts: Res<'_, TonemappingLuts>,
    light_probes_buffer: Res<'_, LightProbesBuffer>,
    visibility_ranges: Res<'_, RenderVisibilityRanges>,
    _: (Res<'_, ScreenSpaceReflectionsBuffer>, Res<'_, ContactShadowsBuffer>, Res<'_, OitBuffers>),
    _: (Res<'_, DecalsBuffer>, Res<'_, RenderClusteredDecals>, Option<Res<'_, AtmosphereBuffer>>, Option<Res<'_, AtmosphereSampler>>, Res<'_, Bluenoise>, Res<'_, AreaLightLuts>, Res<'_, DfgLut>),
    entries_cache: Local<'_, Vec<BindGroupEntry<'_>>>,
    entries_binding_array_cache: Local<'_, Vec<BindGroupEntry<'_>>>,
)
```