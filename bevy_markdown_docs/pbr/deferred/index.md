[bevy](../../index.html)::[pbr](../index.html)

# Module deferred 

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/lib.rs.html#45)

## Structs

[DeferredLightingLayout](struct.DeferredLightingLayout.html "struct bevy::pbr::deferred::DeferredLightingLayout")

[DeferredLightingPipeline](struct.DeferredLightingPipeline.html "struct bevy::pbr::deferred::DeferredLightingPipeline")

[DeferredPbrLightingPlugin](struct.DeferredPbrLightingPlugin.html "struct bevy::pbr::deferred::DeferredPbrLightingPlugin")

[PbrDeferredLightingDepthId](struct.PbrDeferredLightingDepthId.html "struct bevy::pbr::deferred::PbrDeferredLightingDepthId")

Component with a `depth_id` for specifying which corresponding materials should be rendered by this specific PBR deferred lighting pass.

[SkipDeferredLighting](struct.SkipDeferredLighting.html "struct bevy::pbr::deferred::SkipDeferredLighting")

Component to skip running the deferred lighting pass in [`deferred_lighting`](fn.deferred_lighting.html "fn bevy::pbr::deferred::deferred_lighting") for a specific view.

## Constants

[DEFAULT\_PBR\_DEFERRED\_LIGHTING\_PASS\_ID](constant.DEFAULT_PBR_DEFERRED_LIGHTING_PASS_ID.html "constant bevy::pbr::deferred::DEFAULT_PBR_DEFERRED_LIGHTING_PASS_ID")

## Functions

[deferred\_lighting](fn.deferred_lighting.html "fn bevy::pbr::deferred::deferred_lighting")

[init\_deferred\_lighting\_layout](fn.init_deferred_lighting_layout.html "fn bevy::pbr::deferred::init_deferred_lighting_layout")

[insert\_deferred\_lighting\_pass\_id\_component](fn.insert_deferred_lighting_pass_id_component.html "fn bevy::pbr::deferred::insert_deferred_lighting_pass_id_component")

[prepare\_deferred\_lighting\_pipelines](fn.prepare_deferred_lighting_pipelines.html "fn bevy::pbr::deferred::prepare_deferred_lighting_pipelines")