[bevy](../../index.html)::[pbr](../index.html)

# Module wireframe 

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/lib.rs.html#13)

## Structs

[DrawWireframeMeshPulled](struct.DrawWireframeMeshPulled.html "struct bevy::pbr::wireframe::DrawWireframeMeshPulled")

[Mesh3dWireframe](struct.Mesh3dWireframe.html "struct bevy::pbr::wireframe::Mesh3dWireframe")

[Mesh3dWireframeTemplate](struct.Mesh3dWireframeTemplate.html "struct bevy::pbr::wireframe::Mesh3dWireframeTemplate")

[NoWireframe](struct.NoWireframe.html "struct bevy::pbr::wireframe::NoWireframe")

Disables wireframe rendering for any entity it is attached to. It will ignore the [`WireframeConfig`](struct.WireframeConfig.html "struct bevy::pbr::wireframe::WireframeConfig") global setting.

[PendingWireframeQueues](struct.PendingWireframeQueues.html "struct bevy::pbr::wireframe::PendingWireframeQueues")

[RenderWireframeInstances](struct.RenderWireframeInstances.html "struct bevy::pbr::wireframe::RenderWireframeInstances")

[RenderWireframeMaterial](struct.RenderWireframeMaterial.html "struct bevy::pbr::wireframe::RenderWireframeMaterial")

[SetWireframe3dThinImmediates](struct.SetWireframe3dThinImmediates.html "struct bevy::pbr::wireframe::SetWireframe3dThinImmediates")

[SetWireframe3dWideBindGroup](struct.SetWireframe3dWideBindGroup.html "struct bevy::pbr::wireframe::SetWireframe3dWideBindGroup")

[SetWireframe3dWideImmediates](struct.SetWireframe3dWideImmediates.html "struct bevy::pbr::wireframe::SetWireframe3dWideImmediates")

[SpecializedWireframePipelineCache](struct.SpecializedWireframePipelineCache.html "struct bevy::pbr::wireframe::SpecializedWireframePipelineCache")

[SpecializedWireframeViewPipelineCache](struct.SpecializedWireframeViewPipelineCache.html "struct bevy::pbr::wireframe::SpecializedWireframeViewPipelineCache")

[Wireframe](struct.Wireframe.html "struct bevy::pbr::wireframe::Wireframe")

Enables wireframe rendering for any entity it is attached to. It will ignore the [`WireframeConfig`](struct.WireframeConfig.html "struct bevy::pbr::wireframe::WireframeConfig") global setting.

[Wireframe3d](struct.Wireframe3d.html "struct bevy::pbr::wireframe::Wireframe3d")

[Wireframe3dBatchSetKey](struct.Wireframe3dBatchSetKey.html "struct bevy::pbr::wireframe::Wireframe3dBatchSetKey")

[Wireframe3dBinKey](struct.Wireframe3dBinKey.html "struct bevy::pbr::wireframe::Wireframe3dBinKey")

Data that must be identical in order to _batch_ phase items together.

[Wireframe3dPipeline](struct.Wireframe3dPipeline.html "struct bevy::pbr::wireframe::Wireframe3dPipeline")

[WireframeColor](struct.WireframeColor.html "struct bevy::pbr::wireframe::WireframeColor")

Sets the color of the [`Wireframe`](struct.Wireframe.html "struct bevy::pbr::wireframe::Wireframe") of the entity it is attached to.

[WireframeConfig](struct.WireframeConfig.html "struct bevy::pbr::wireframe::WireframeConfig")

[WireframeEntitiesNeedingSpecialization](struct.WireframeEntitiesNeedingSpecialization.html "struct bevy::pbr::wireframe::WireframeEntitiesNeedingSpecialization")

Temporarily stores entities that were determined to either need their specialized pipelines for wireframes updated or to have their specialized pipelines for wireframes removed.

[WireframeLineWidth](struct.WireframeLineWidth.html "struct bevy::pbr::wireframe::WireframeLineWidth")

Sets the line width (in screen-space pixels) of the wireframe.

[WireframeMaterial](struct.WireframeMaterial.html "struct bevy::pbr::wireframe::WireframeMaterial")

[WireframePipelineKey](struct.WireframePipelineKey.html "struct bevy::pbr::wireframe::WireframePipelineKey")

[WireframePlugin](struct.WireframePlugin.html "struct bevy::pbr::wireframe::WireframePlugin")

A [`Plugin`](../../prelude/trait.Plugin.html "trait bevy::prelude::Plugin") that draws wireframes.

[WireframeVertexPullParams](struct.WireframeVertexPullParams.html "struct bevy::pbr::wireframe::WireframeVertexPullParams")

[WireframeWideBindGroups](struct.WireframeWideBindGroups.html "struct bevy::pbr::wireframe::WireframeWideBindGroups")

## Enums

[WireframeTopology](enum.WireframeTopology.html "enum bevy::pbr::wireframe::WireframeTopology")

Controls whether wireframe edges follow triangle or quad topology.

## Functions

[check\_wireframe\_entities\_needing\_specialization](fn.check_wireframe_entities_needing_specialization.html "fn bevy::pbr::wireframe::check_wireframe_entities_needing_specialization")

Finds 3D wireframe entities that have changed in such a way as to potentially require specialization and adds them to the [`WireframeEntitiesNeedingSpecialization`](struct.WireframeEntitiesNeedingSpecialization.html "struct bevy::pbr::wireframe::WireframeEntitiesNeedingSpecialization") list.

[extract\_wireframe\_entities\_needing\_specialization](fn.extract_wireframe_entities_needing_specialization.html "fn bevy::pbr::wireframe::extract_wireframe_entities_needing_specialization")

[extract\_wireframe\_entities\_that\_need\_specializations\_removed](fn.extract_wireframe_entities_that_need_specializations_removed.html "fn bevy::pbr::wireframe::extract_wireframe_entities_that_need_specializations_removed")

A system that adds entities that were judged to need their wireframe specializations removed to the appropriate table in [`DirtyWireframeSpecializations`](../../render/camera/struct.DirtyWireframeSpecializations.html "struct bevy::render::camera::DirtyWireframeSpecializations").

[extract\_wireframe\_materials](fn.extract_wireframe_materials.html "fn bevy::pbr::wireframe::extract_wireframe_materials")

[init\_wireframe\_3d\_pipeline](fn.init_wireframe_3d_pipeline.html "fn bevy::pbr::wireframe::init_wireframe_3d_pipeline")

[prepare\_wireframe\_wide\_bind\_groups](fn.prepare_wireframe_wide_bind_groups.html "fn bevy::pbr::wireframe::prepare_wireframe_wide_bind_groups")

[specialize\_wireframes](fn.specialize_wireframes.html "fn bevy::pbr::wireframe::specialize_wireframes")

[wireframe\_3d](fn.wireframe_3d.html "fn bevy::pbr::wireframe::wireframe_3d")

## Type Aliases

[DrawWireframe3dThin](type.DrawWireframe3dThin.html "type bevy::pbr::wireframe::DrawWireframe3dThin")

Draw wireframes with `PolygonMode::Line`, i.e. the fast path.

[DrawWireframe3dWide](type.DrawWireframe3dWide.html "type bevy::pbr::wireframe::DrawWireframe3dWide")

Draw wireframes using vertex pulling for wide lines or quad topology.