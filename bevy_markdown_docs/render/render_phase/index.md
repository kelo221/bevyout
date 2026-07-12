[bevy](../../index.html)::[render](../index.html)

# Module render\_phase 

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/lib.rs.html#57)

The modular rendering abstraction responsible for queuing, preparing, sorting and drawing entities as part of separate render phases.

In Bevy each view (camera, or shadow-casting light, etc.) has one or multiple render phases (e.g. opaque, transparent, shadow, etc). They are used to queue entities for rendering. Multiple phases might be required due to different sorting/batching behaviors (e.g. opaque: front to back, transparent: back to front) or because one phase depends on the rendered texture of the previous phase (e.g. for screen-space reflections).

To draw an entity, a corresponding [`PhaseItem`](trait.PhaseItem.html "trait bevy::render::render_phase::PhaseItem") has to be added to one or multiple of these render phases for each view that it is visible in. This must be done in the [`RenderSystems::Queue`](../enum.RenderSystems.html#variant.Queue "variant bevy::render::RenderSystems::Queue"). After that the render phase sorts them in the [`RenderSystems::PhaseSort`](../enum.RenderSystems.html#variant.PhaseSort "variant bevy::render::RenderSystems::PhaseSort"). Finally the items are rendered using a single [`TrackedRenderPass`](struct.TrackedRenderPass.html "struct bevy::render::render_phase::TrackedRenderPass"), during the [`RenderSystems::Render`](../enum.RenderSystems.html#variant.Render "variant bevy::render::RenderSystems::Render").

Therefore each phase item is assigned a [`Draw`](trait.Draw.html "trait bevy::render::render_phase::Draw") function. These set up the state of the [`TrackedRenderPass`](struct.TrackedRenderPass.html "struct bevy::render::render_phase::TrackedRenderPass") (i.e. select the [`RenderPipeline`](../render_resource/struct.RenderPipeline.html "struct bevy::render::render_resource::RenderPipeline"), configure the [`BindGroup`](../render_resource/struct.BindGroup.html "struct bevy::render::render_resource::BindGroup")s, etc.) and then issue a draw call, for the corresponding item.

The [`Draw`](trait.Draw.html "trait bevy::render::render_phase::Draw") function trait can either be implemented directly or such a function can be created by composing multiple [`RenderCommand`](trait.RenderCommand.html "trait bevy::render::render_phase::RenderCommand")s.

## Structs

[BinnedRenderPhase](struct.BinnedRenderPhase.html "struct bevy::render::render_phase::BinnedRenderPhase")

A collection of all rendering instructions, that will be executed by the GPU, for a single render phase for a single view.

[BinnedRenderPhaseBatch](struct.BinnedRenderPhaseBatch.html "struct bevy::render::render_phase::BinnedRenderPhaseBatch")

Information about a single batch of entities rendered using binned phase items.

[BinnedRenderPhaseBatchSet](struct.BinnedRenderPhaseBatchSet.html "struct bevy::render::render_phase::BinnedRenderPhaseBatchSet")

A group of entities that will be batched together into a single multi-draw call.

[BinnedRenderPhasePlugin](struct.BinnedRenderPhasePlugin.html "struct bevy::render::render_phase::BinnedRenderPhasePlugin")

A convenient abstraction for adding all the systems necessary for a binned render phase to the render app.

[CachedBinKey](struct.CachedBinKey.html "struct bevy::render::render_phase::CachedBinKey")

Information that we use to identify a cached entity in a bin.

[CachedBinnedEntity](struct.CachedBinnedEntity.html "struct bevy::render::render_phase::CachedBinnedEntity")

Information that we keep about an entity currently within a bin.

[DrawFunctionId](struct.DrawFunctionId.html "struct bevy::render::render_phase::DrawFunctionId")

An identifier for a [`Draw`](https://docs.rs/bevy/latest/bevy/render/render_phase/trait.Draw.html) function stored in [`DrawFunctions`](https://docs.rs/bevy/latest/bevy/render/render_phase/struct.DrawFunctions.html).

[DrawFunctions](struct.DrawFunctions.html "struct bevy::render::render_phase::DrawFunctions")

Stores all draw functions for the [`PhaseItem`](trait.PhaseItem.html "trait bevy::render::render_phase::PhaseItem") type hidden behind a reader-writer lock.

[DrawFunctionsInternal](struct.DrawFunctionsInternal.html "struct bevy::render::render_phase::DrawFunctionsInternal")

Stores all [`Draw`](trait.Draw.html "trait bevy::render::render_phase::Draw") functions for the [`PhaseItem`](trait.PhaseItem.html "trait bevy::render::render_phase::PhaseItem") type.

[GpuRenderBinnedMeshInstance](struct.GpuRenderBinnedMeshInstance.html "struct bevy::render::render_phase::GpuRenderBinnedMeshInstance")

A single mesh instance in a bin.

[InputUniformIndex](struct.InputUniformIndex.html "struct bevy::render::render_phase::InputUniformIndex")

The index of the uniform describing this object in the GPU buffer, when GPU preprocessing is enabled.

[NonMeshEntities](struct.NonMeshEntities.html "struct bevy::render::render_phase::NonMeshEntities")

Information about [`BinnedRenderPhaseType::NonMesh`](enum.BinnedRenderPhaseType.html#variant.NonMesh "variant bevy::render::render_phase::BinnedRenderPhaseType::NonMesh") entities.

[RenderBin](struct.RenderBin.html "struct bevy::render::render_phase::RenderBin")

All entities that share a mesh and a material and can be batched as part of a [`BinnedRenderPhase`](struct.BinnedRenderPhase.html "struct bevy::render::render_phase::BinnedRenderPhase").

[RenderCommandState](struct.RenderCommandState.html "struct bevy::render::render_phase::RenderCommandState")

Wraps a [`RenderCommand`](trait.RenderCommand.html "trait bevy::render::render_phase::RenderCommand") into a state so that it can be used as a [`Draw`](trait.Draw.html "trait bevy::render::render_phase::Draw") function.

[RenderMultidrawableBatchSet](struct.RenderMultidrawableBatchSet.html "struct bevy::render::render_phase::RenderMultidrawableBatchSet")

A collection of mesh instances that can be drawn together, sorted into bins.

[RenderMultidrawableBatchSetGpuBuffers](struct.RenderMultidrawableBatchSetGpuBuffers.html "struct bevy::render::render_phase::RenderMultidrawableBatchSetGpuBuffers")

The GPU buffers that go along with [`RenderMultidrawableBatchSet`](struct.RenderMultidrawableBatchSet.html "struct bevy::render::render_phase::RenderMultidrawableBatchSet").

[RenderMultidrawableBin](struct.RenderMultidrawableBin.html "struct bevy::render::render_phase::RenderMultidrawableBin")

Information about each bin that the [`RenderMultidrawableBatchSet`](struct.RenderMultidrawableBatchSet.html "struct bevy::render::render_phase::RenderMultidrawableBatchSet") maintains on the CPU.

[SetItemPipeline](struct.SetItemPipeline.html "struct bevy::render::render_phase::SetItemPipeline")

A [`RenderCommand`](trait.RenderCommand.html "trait bevy::render::render_phase::RenderCommand") that sets the pipeline for the [`CachedRenderPipelinePhaseItem`](trait.CachedRenderPipelinePhaseItem.html "trait bevy::render::render_phase::CachedRenderPipelinePhaseItem").

[SortedRenderPhase](struct.SortedRenderPhase.html "struct bevy::render::render_phase::SortedRenderPhase")

A collection of all items to be rendered that will be encoded to GPU commands for a single render phase for a single view.

[SortedRenderPhasePlugin](struct.SortedRenderPhasePlugin.html "struct bevy::render::render_phase::SortedRenderPhasePlugin")

A convenient abstraction for adding all the systems necessary for a sorted render phase to the render app.

[TrackedRenderPass](struct.TrackedRenderPass.html "struct bevy::render::render_phase::TrackedRenderPass")

A [`RenderPass`](https://docs.rs/wgpu/29.0.3/x86_64-unknown-linux-gnu/wgpu/api/render_pass/struct.RenderPass.html "struct wgpu::api::render_pass::RenderPass"), which tracks the current pipeline state to skip redundant operations.

[UnbatchableBinnedEntities](struct.UnbatchableBinnedEntities.html "struct bevy::render::render_phase::UnbatchableBinnedEntities")

Information about the unbatchable entities in a bin.

[ViewBinnedRenderPhases](struct.ViewBinnedRenderPhases.html "struct bevy::render::render_phase::ViewBinnedRenderPhases")

Stores the rendering instructions for a single phase that uses bins in all views.

[ViewRangefinder3d](struct.ViewRangefinder3d.html "struct bevy::render::render_phase::ViewRangefinder3d")

A distance calculator for the draw order of [`PhaseItem`](trait.PhaseItem.html "trait bevy::render::render_phase::PhaseItem")s.

[ViewSortedRenderPhases](struct.ViewSortedRenderPhases.html "struct bevy::render::render_phase::ViewSortedRenderPhases")

Stores the rendering instructions for a single phase that sorts items in all views.

## Enums

[BinnedRenderPhaseBatchSets](enum.BinnedRenderPhaseBatchSets.html "enum bevy::render::render_phase::BinnedRenderPhaseBatchSets")

How we store and render the batch sets.

[BinnedRenderPhaseType](enum.BinnedRenderPhaseType.html "enum bevy::render::render_phase::BinnedRenderPhaseType")

Identifies the list within [`BinnedRenderPhase`](struct.BinnedRenderPhase.html "struct bevy::render::render_phase::BinnedRenderPhase") that a phase item is to be placed in.

[DrawError](enum.DrawError.html "enum bevy::render::render_phase::DrawError")

[PhaseItemExtraIndex](enum.PhaseItemExtraIndex.html "enum bevy::render::render_phase::PhaseItemExtraIndex")

The “extra index” associated with some [`PhaseItem`](trait.PhaseItem.html "trait bevy::render::render_phase::PhaseItem")s, alongside the indirect instance index.

[RenderCommandResult](enum.RenderCommandResult.html "enum bevy::render::render_phase::RenderCommandResult")

The result of a [`RenderCommand`](trait.RenderCommand.html "trait bevy::render::render_phase::RenderCommand").

## Traits

[AddRenderCommand](trait.AddRenderCommand.html "trait bevy::render::render_phase::AddRenderCommand")

Registers a [`RenderCommand`](trait.RenderCommand.html "trait bevy::render::render_phase::RenderCommand") as a [`Draw`](trait.Draw.html "trait bevy::render::render_phase::Draw") function. They are stored inside the [`DrawFunctions`](struct.DrawFunctions.html "struct bevy::render::render_phase::DrawFunctions") resource of the app.

[BinnedPhaseItem](trait.BinnedPhaseItem.html "trait bevy::render::render_phase::BinnedPhaseItem")

Represents phase items that are placed into bins. The `BinKey` specifies which bin they’re to be placed in. Bin keys are sorted, and items within the same bin are eligible to be batched together. The elements within the bins aren’t themselves sorted.

[CachedRenderPipelinePhaseItem](trait.CachedRenderPipelinePhaseItem.html "trait bevy::render::render_phase::CachedRenderPipelinePhaseItem")

A [`PhaseItem`](trait.PhaseItem.html "trait bevy::render::render_phase::PhaseItem") item, that automatically sets the appropriate render pipeline, cached in the [`PipelineCache`](../render_resource/struct.PipelineCache.html "struct bevy::render::render_resource::PipelineCache").

[Draw](trait.Draw.html "trait bevy::render::render_phase::Draw")

A draw function used to draw [`PhaseItem`](trait.PhaseItem.html "trait bevy::render::render_phase::PhaseItem")s.

[PhaseItem](trait.PhaseItem.html "trait bevy::render::render_phase::PhaseItem")

An item (entity of the render world) which will be drawn to a texture or the screen, as part of a render phase.

[PhaseItemBatchSetKey](trait.PhaseItemBatchSetKey.html "trait bevy::render::render_phase::PhaseItemBatchSetKey")

A key used to combine batches into batch sets.

[RenderCommand](trait.RenderCommand.html "trait bevy::render::render_phase::RenderCommand")

[`RenderCommand`](trait.RenderCommand.html "trait bevy::render::render_phase::RenderCommand")s are modular standardized pieces of render logic that can be composed into [`Draw`](trait.Draw.html "trait bevy::render::render_phase::Draw") functions.

[SortedPhaseItem](trait.SortedPhaseItem.html "trait bevy::render::render_phase::SortedPhaseItem")

Represents phase items that must be sorted. The `SortKey` specifies the order that these items are drawn in. These are placed into a single array, and the array as a whole is then sorted.

## Functions

[sort\_phase\_system](fn.sort_phase_system.html "fn bevy::render::render_phase::sort_phase_system")

This system sorts the [`PhaseItem`](trait.PhaseItem.html "trait bevy::render::render_phase::PhaseItem")s of all [`SortedRenderPhase`](struct.SortedRenderPhase.html "struct bevy::render::render_phase::SortedRenderPhase")s of this type.

## Derive Macros

[DrawFunctionLabel](derive.DrawFunctionLabel.html "derive bevy::render::render_phase::DrawFunctionLabel")

[ShaderLabel](derive.ShaderLabel.html "derive bevy::render::render_phase::ShaderLabel")