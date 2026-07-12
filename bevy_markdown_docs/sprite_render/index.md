[bevy](../index.html)

# Crate sprite\_render 

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/lib.rs.html#1-125)

Provides 2D sprite rendering functionality.

## Modules

[prelude](prelude/index.html "mod bevy::sprite_render::prelude")

The sprite prelude.

## Structs

[ColorMaterial](struct.ColorMaterial.html "struct bevy::sprite_render::ColorMaterial")

A [2d material](trait.Material2d.html "trait bevy::sprite_render::Material2d") that renders [2d meshes](../prelude/struct.Mesh2d.html "struct bevy::prelude::Mesh2d") with a texture tinted by a uniform color

[ColorMaterialFlags](struct.ColorMaterialFlags.html "struct bevy::sprite_render::ColorMaterialFlags")

[ColorMaterialPlugin](struct.ColorMaterialPlugin.html "struct bevy::sprite_render::ColorMaterialPlugin")

[ColorMaterialUniform](struct.ColorMaterialUniform.html "struct bevy::sprite_render::ColorMaterialUniform")

The GPU representation of the uniform data of a [`ColorMaterial`](../prelude/struct.ColorMaterial.html "struct bevy::prelude::ColorMaterial").

[DrawMesh2d](struct.DrawMesh2d.html "struct bevy::sprite_render::DrawMesh2d")

[DrawSpriteBatch](struct.DrawSpriteBatch.html "struct bevy::sprite_render::DrawSpriteBatch")

[EntitiesNeedingSpecialization](struct.EntitiesNeedingSpecialization.html "struct bevy::sprite_render::EntitiesNeedingSpecialization")

Temporarily stores entities that were determined to either need their specialized pipelines updated or to have their specialized pipelines removed.

[ExtractedSlice](struct.ExtractedSlice.html "struct bevy::sprite_render::ExtractedSlice")

[ExtractedSlices](struct.ExtractedSlices.html "struct bevy::sprite_render::ExtractedSlices")

[ExtractedSprite](struct.ExtractedSprite.html "struct bevy::sprite_render::ExtractedSprite")

[ExtractedSprites](struct.ExtractedSprites.html "struct bevy::sprite_render::ExtractedSprites")

[ExtractedWireframeColor](struct.ExtractedWireframeColor.html "struct bevy::sprite_render::ExtractedWireframeColor")

[ImageBindGroups](struct.ImageBindGroups.html "struct bevy::sprite_render::ImageBindGroups")

[Material2dBindGroupId](struct.Material2dBindGroupId.html "struct bevy::sprite_render::Material2dBindGroupId")

[Material2dKey](struct.Material2dKey.html "struct bevy::sprite_render::Material2dKey")

[Material2dPipeline](struct.Material2dPipeline.html "struct bevy::sprite_render::Material2dPipeline")

Render pipeline data for a given [`Material2d`](trait.Material2d.html "trait bevy::sprite_render::Material2d")

[Material2dPlugin](struct.Material2dPlugin.html "struct bevy::sprite_render::Material2dPlugin")

Adds the necessary ECS resources and render logic to enable rendering entities using the given [`Material2d`](trait.Material2d.html "trait bevy::sprite_render::Material2d") asset type (which includes [`Material2d`](trait.Material2d.html "trait bevy::sprite_render::Material2d") types).

[Material2dProperties](struct.Material2dProperties.html "struct bevy::sprite_render::Material2dProperties")

Common [`Material2d`](trait.Material2d.html "trait bevy::sprite_render::Material2d") properties, calculated for a specific material instance.

[Mesh2dBindGroup](struct.Mesh2dBindGroup.html "struct bevy::sprite_render::Mesh2dBindGroup")

[Mesh2dMarker](struct.Mesh2dMarker.html "struct bevy::sprite_render::Mesh2dMarker")

[Mesh2dPipeline](struct.Mesh2dPipeline.html "struct bevy::sprite_render::Mesh2dPipeline")

[Mesh2dPipelineKey](struct.Mesh2dPipelineKey.html "struct bevy::sprite_render::Mesh2dPipelineKey")

[Mesh2dRenderPlugin](struct.Mesh2dRenderPlugin.html "struct bevy::sprite_render::Mesh2dRenderPlugin")

[Mesh2dTransforms](struct.Mesh2dTransforms.html "struct bevy::sprite_render::Mesh2dTransforms")

[Mesh2dUniform](struct.Mesh2dUniform.html "struct bevy::sprite_render::Mesh2dUniform")

[Mesh2dViewBindGroup](struct.Mesh2dViewBindGroup.html "struct bevy::sprite_render::Mesh2dViewBindGroup")

[Mesh2dWireframe](struct.Mesh2dWireframe.html "struct bevy::sprite_render::Mesh2dWireframe")

[Mesh2dWireframeTemplate](struct.Mesh2dWireframeTemplate.html "struct bevy::sprite_render::Mesh2dWireframeTemplate")

[MeshFlags](struct.MeshFlags.html "struct bevy::sprite_render::MeshFlags")

[MeshMaterial2d](struct.MeshMaterial2d.html "struct bevy::sprite_render::MeshMaterial2d")

A [material](trait.Material2d.html "trait bevy::sprite_render::Material2d") used for rendering a [`Mesh2d`](../prelude/struct.Mesh2d.html "struct bevy::prelude::Mesh2d").

[MeshMaterial2dTemplate](struct.MeshMaterial2dTemplate.html "struct bevy::sprite_render::MeshMaterial2dTemplate")

[NoWireframe2d](struct.NoWireframe2d.html "struct bevy::sprite_render::NoWireframe2d")

Disables wireframe rendering for any entity it is attached to. It will ignore the [`Wireframe2dConfig`](struct.Wireframe2dConfig.html "struct bevy::sprite_render::Wireframe2dConfig") global setting.

[PackedTileData](struct.PackedTileData.html "struct bevy::sprite_render::PackedTileData")

Packed per-tile data for use in the `Rgba16Uint` tile data texture in `TilemapChunkMaterial`.

[PendingMeshMaterial2dQueues](struct.PendingMeshMaterial2dQueues.html "struct bevy::sprite_render::PendingMeshMaterial2dQueues")

Holds all entities with 2D mesh materials that couldn’t be specialized and/or queued because their materials hadn’t loaded yet.

[PendingWireframe2dQueues](struct.PendingWireframe2dQueues.html "struct bevy::sprite_render::PendingWireframe2dQueues")

[PreparedMaterial2d](struct.PreparedMaterial2d.html "struct bevy::sprite_render::PreparedMaterial2d")

Data prepared for a [`Material2d`](trait.Material2d.html "trait bevy::sprite_render::Material2d") instance.

[RenderMaterial2dBindGroupIds](struct.RenderMaterial2dBindGroupIds.html "struct bevy::sprite_render::RenderMaterial2dBindGroupIds")

[RenderMaterial2dIds](struct.RenderMaterial2dIds.html "struct bevy::sprite_render::RenderMaterial2dIds")

[RenderMaterial2dInstances](struct.RenderMaterial2dInstances.html "struct bevy::sprite_render::RenderMaterial2dInstances")

[RenderMesh2dInstance](struct.RenderMesh2dInstance.html "struct bevy::sprite_render::RenderMesh2dInstance")

[RenderMesh2dInstances](struct.RenderMesh2dInstances.html "struct bevy::sprite_render::RenderMesh2dInstances")

[RenderWireframeInstances](struct.RenderWireframeInstances.html "struct bevy::sprite_render::RenderWireframeInstances")

[RenderWireframeMaterial](struct.RenderWireframeMaterial.html "struct bevy::sprite_render::RenderWireframeMaterial")

[SetMaterial2dBindGroup](struct.SetMaterial2dBindGroup.html "struct bevy::sprite_render::SetMaterial2dBindGroup")

[SetMesh2dBindGroup](struct.SetMesh2dBindGroup.html "struct bevy::sprite_render::SetMesh2dBindGroup")

[SetMesh2dViewBindGroup](struct.SetMesh2dViewBindGroup.html "struct bevy::sprite_render::SetMesh2dViewBindGroup")

[SetSpriteTextureBindGroup](struct.SetSpriteTextureBindGroup.html "struct bevy::sprite_render::SetSpriteTextureBindGroup")

[SetSpriteViewBindGroup](struct.SetSpriteViewBindGroup.html "struct bevy::sprite_render::SetSpriteViewBindGroup")

[SetWireframe2dImmediates](struct.SetWireframe2dImmediates.html "struct bevy::sprite_render::SetWireframe2dImmediates")

[SpecializedMaterial2dPipelineCache](struct.SpecializedMaterial2dPipelineCache.html "struct bevy::sprite_render::SpecializedMaterial2dPipelineCache")

Stores the [`SpecializedMaterial2dViewPipelineCache`](struct.SpecializedMaterial2dViewPipelineCache.html "struct bevy::sprite_render::SpecializedMaterial2dViewPipelineCache") for each view.

[SpecializedMaterial2dViewPipelineCache](struct.SpecializedMaterial2dViewPipelineCache.html "struct bevy::sprite_render::SpecializedMaterial2dViewPipelineCache")

Stores the cached render pipeline ID for each entity in a single view, as well as the last time it was changed.

[SpecializedWireframePipelineCache](struct.SpecializedWireframePipelineCache.html "struct bevy::sprite_render::SpecializedWireframePipelineCache")

Stores the [`SpecializedWireframeViewPipelineCache`](struct.SpecializedWireframeViewPipelineCache.html "struct bevy::sprite_render::SpecializedWireframeViewPipelineCache") for each view.

[SpecializedWireframeViewPipelineCache](struct.SpecializedWireframeViewPipelineCache.html "struct bevy::sprite_render::SpecializedWireframeViewPipelineCache")

Stores the cached render pipeline ID for each entity in a single view, as well as the last time it was changed.

[SpriteAssetEvents](struct.SpriteAssetEvents.html "struct bevy::sprite_render::SpriteAssetEvents")

[SpriteBatch](struct.SpriteBatch.html "struct bevy::sprite_render::SpriteBatch")

[SpriteBatches](struct.SpriteBatches.html "struct bevy::sprite_render::SpriteBatches")

[SpriteMaterial](struct.SpriteMaterial.html "struct bevy::sprite_render::SpriteMaterial")

[SpriteMaterialFlags](struct.SpriteMaterialFlags.html "struct bevy::sprite_render::SpriteMaterialFlags")

[SpriteMaterialPlugin](struct.SpriteMaterialPlugin.html "struct bevy::sprite_render::SpriteMaterialPlugin")

[SpriteMaterialUniform](struct.SpriteMaterialUniform.html "struct bevy::sprite_render::SpriteMaterialUniform")

[SpriteMeshPlugin](struct.SpriteMeshPlugin.html "struct bevy::sprite_render::SpriteMeshPlugin")

[SpriteMeta](struct.SpriteMeta.html "struct bevy::sprite_render::SpriteMeta")

[SpritePipeline](struct.SpritePipeline.html "struct bevy::sprite_render::SpritePipeline")

[SpritePipelineKey](struct.SpritePipelineKey.html "struct bevy::sprite_render::SpritePipelineKey")

[SpriteRenderPlugin](struct.SpriteRenderPlugin.html "struct bevy::sprite_render::SpriteRenderPlugin")

Adds support for 2D sprite rendering.

[SpriteViewBindGroup](struct.SpriteViewBindGroup.html "struct bevy::sprite_render::SpriteViewBindGroup")

[TileData](struct.TileData.html "struct bevy::sprite_render::TileData")

Data for a single tile in the tilemap chunk.

[TilemapChunk](struct.TilemapChunk.html "struct bevy::sprite_render::TilemapChunk")

A component representing a chunk of a tilemap. Each chunk is a rectangular section of tiles that is rendered as a single mesh.

[TilemapChunkMaterial](struct.TilemapChunkMaterial.html "struct bevy::sprite_render::TilemapChunkMaterial")

Material used for rendering tilemap chunks.

[TilemapChunkMaterialPlugin](struct.TilemapChunkMaterialPlugin.html "struct bevy::sprite_render::TilemapChunkMaterialPlugin")

Plugin that adds support for tilemap chunk materials.

[TilemapChunkMeshCache](struct.TilemapChunkMeshCache.html "struct bevy::sprite_render::TilemapChunkMeshCache")

A resource storing the meshes for each tilemap chunk size.

[TilemapChunkPlugin](struct.TilemapChunkPlugin.html "struct bevy::sprite_render::TilemapChunkPlugin")

Plugin that handles the initialization and updating of tilemap chunks. Adds systems for processing newly added tilemap chunks and updating their indices.

[TilemapChunkTemplate](struct.TilemapChunkTemplate.html "struct bevy::sprite_render::TilemapChunkTemplate")

[TilemapChunkTileData](struct.TilemapChunkTileData.html "struct bevy::sprite_render::TilemapChunkTileData")

Component storing the data of tiles within a chunk. Each index corresponds to a specific tile in the tileset. `None` indicates an empty tile.

[ViewKeyCache](struct.ViewKeyCache.html "struct bevy::sprite_render::ViewKeyCache")

[Wireframe2d](struct.Wireframe2d.html "struct bevy::sprite_render::Wireframe2d")

Enables wireframe rendering for any entity it is attached to. It will ignore the [`Wireframe2dConfig`](struct.Wireframe2dConfig.html "struct bevy::sprite_render::Wireframe2dConfig") global setting.

[Wireframe2dBatchSetKey](struct.Wireframe2dBatchSetKey.html "struct bevy::sprite_render::Wireframe2dBatchSetKey")

[Wireframe2dBinKey](struct.Wireframe2dBinKey.html "struct bevy::sprite_render::Wireframe2dBinKey")

Data that must be identical in order to _batch_ phase items together.

[Wireframe2dColor](struct.Wireframe2dColor.html "struct bevy::sprite_render::Wireframe2dColor")

Sets the color of the [`Wireframe2d`](struct.Wireframe2d.html "struct bevy::sprite_render::Wireframe2d") of the entity it is attached to.

[Wireframe2dConfig](struct.Wireframe2dConfig.html "struct bevy::sprite_render::Wireframe2dConfig")

[Wireframe2dMaterial](struct.Wireframe2dMaterial.html "struct bevy::sprite_render::Wireframe2dMaterial")

[Wireframe2dPhaseItem](struct.Wireframe2dPhaseItem.html "struct bevy::sprite_render::Wireframe2dPhaseItem")

[Wireframe2dPipeline](struct.Wireframe2dPipeline.html "struct bevy::sprite_render::Wireframe2dPipeline")

[Wireframe2dPlugin](struct.Wireframe2dPlugin.html "struct bevy::sprite_render::Wireframe2dPlugin")

A [`Plugin`](../prelude/trait.Plugin.html "trait bevy::prelude::Plugin") that draws wireframes for 2D meshes.

[WireframeEntitiesNeedingSpecialization](struct.WireframeEntitiesNeedingSpecialization.html "struct bevy::sprite_render::WireframeEntitiesNeedingSpecialization")

Temporarily stores entities that were determined to either need their specialized pipelines for wireframes updated or to have their specialized pipelines for wireframes removed.

## Enums

[AlphaMode2d](enum.AlphaMode2d.html "enum bevy::sprite_render::AlphaMode2d")

Sets how a 2d material’s base color alpha channel is used for transparency. Currently, this only works with [`Mesh2d`](../prelude/struct.Mesh2d.html "struct bevy::prelude::Mesh2d"). Sprites are always transparent.

[ExtractedSpriteKind](enum.ExtractedSpriteKind.html "enum bevy::sprite_render::ExtractedSpriteKind")

[SpriteSystems](enum.SpriteSystems.html "enum bevy::sprite_render::SpriteSystems")

System set for sprite rendering.

[TileOrientation](enum.TileOrientation.html "enum bevy::sprite_render::TileOrientation")

The set of possible tile orientations. These represent all possible results of mirroring the tile horizontally and/or vertically, and/or rotation by 90 degree increments.

## Constants

[MATERIAL\_2D\_BIND\_GROUP\_INDEX](constant.MATERIAL_2D_BIND_GROUP_INDEX.html "constant bevy::sprite_render::MATERIAL_2D_BIND_GROUP_INDEX")

## Traits

[Material2d](trait.Material2d.html "trait bevy::sprite_render::Material2d")

Materials are used alongside [`Material2dPlugin`](struct.Material2dPlugin.html "struct bevy::sprite_render::Material2dPlugin"), [`Mesh2d`](../prelude/struct.Mesh2d.html "struct bevy::prelude::Mesh2d"), and [`MeshMaterial2d`](../prelude/struct.MeshMaterial2d.html "struct bevy::prelude::MeshMaterial2d") to spawn entities that are rendered with a specific [`Material2d`](trait.Material2d.html "trait bevy::sprite_render::Material2d") type. They serve as an easy to use high level way to render [`Mesh2d`](../prelude/struct.Mesh2d.html "struct bevy::prelude::Mesh2d") entities with custom shader logic.

## Functions

[alpha\_mode\_pipeline\_key](fn.alpha_mode_pipeline_key.html "fn bevy::sprite_render::alpha_mode_pipeline_key")

[check\_entities\_needing\_specialization](fn.check_entities_needing_specialization.html "fn bevy::sprite_render::check_entities_needing_specialization")

Finds 2D entities that have changed in such a way as to potentially require specialization and adds them to the [`EntitiesNeedingSpecialization`](struct.EntitiesNeedingSpecialization.html "struct bevy::sprite_render::EntitiesNeedingSpecialization") list.

[check\_views\_need\_specialization](fn.check_views_need_specialization.html "fn bevy::sprite_render::check_views_need_specialization")

[check\_wireframe\_entities\_needing\_specialization](fn.check_wireframe_entities_needing_specialization.html "fn bevy::sprite_render::check_wireframe_entities_needing_specialization")

Finds 2D wireframe entities that have changed in such a way as to potentially require specialization and adds them to the [`WireframeEntitiesNeedingSpecialization`](struct.WireframeEntitiesNeedingSpecialization.html "struct bevy::sprite_render::WireframeEntitiesNeedingSpecialization") list.

[extract\_entities\_needs\_specialization](fn.extract_entities_needs_specialization.html "fn bevy::sprite_render::extract_entities_needs_specialization")

[extract\_entities\_that\_need\_specializations\_removed](fn.extract_entities_that_need_specializations_removed.html "fn bevy::sprite_render::extract_entities_that_need_specializations_removed")

A system that adds entities that were judged to need their specializations removed to the appropriate table in [`DirtySpecializations`](../render/camera/struct.DirtySpecializations.html "struct bevy::render::camera::DirtySpecializations").

[extract\_mesh2d](fn.extract_mesh2d.html "fn bevy::sprite_render::extract_mesh2d")

[extract\_mesh\_materials\_2d](fn.extract_mesh_materials_2d.html "fn bevy::sprite_render::extract_mesh_materials_2d")

[extract\_sprite\_events](fn.extract_sprite_events.html "fn bevy::sprite_render::extract_sprite_events")

[extract\_sprites](fn.extract_sprites.html "fn bevy::sprite_render::extract_sprites")

[extract\_text2d\_sprite](fn.extract_text2d_sprite.html "fn bevy::sprite_render::extract_text2d_sprite")

This system extracts the sprites from the 2D text components and adds them to the “render world”.

[extract\_wireframe\_2d\_entities\_needing\_specialization](fn.extract_wireframe_2d_entities_needing_specialization.html "fn bevy::sprite_render::extract_wireframe_2d_entities_needing_specialization")

[extract\_wireframe\_2d\_entities\_that\_need\_specializations\_removed](fn.extract_wireframe_2d_entities_that_need_specializations_removed.html "fn bevy::sprite_render::extract_wireframe_2d_entities_that_need_specializations_removed")

A system that adds entities that were judged to need their specializations removed to the appropriate table in [`DirtyWireframeSpecializations`](../render/camera/struct.DirtyWireframeSpecializations.html "struct bevy::render::camera::DirtyWireframeSpecializations").

[extract\_wireframe\_materials](fn.extract_wireframe_materials.html "fn bevy::sprite_render::extract_wireframe_materials")

[init\_batched\_instance\_buffer](fn.init_batched_instance_buffer.html "fn bevy::sprite_render::init_batched_instance_buffer")

[init\_material\_2d\_pipeline](fn.init_material_2d_pipeline.html "fn bevy::sprite_render::init_material_2d_pipeline")

[init\_mesh\_2d\_pipeline](fn.init_mesh_2d_pipeline.html "fn bevy::sprite_render::init_mesh_2d_pipeline")

[init\_sprite\_pipeline](fn.init_sprite_pipeline.html "fn bevy::sprite_render::init_sprite_pipeline")

[init\_wireframe\_2d\_pipeline](fn.init_wireframe_2d_pipeline.html "fn bevy::sprite_render::init_wireframe_2d_pipeline")

[make\_chunk\_tile\_data\_image](fn.make_chunk_tile_data_image.html "fn bevy::sprite_render::make_chunk_tile_data_image")

[prepare\_mesh2d\_bind\_group](fn.prepare_mesh2d_bind_group.html "fn bevy::sprite_render::prepare_mesh2d_bind_group")

[prepare\_mesh2d\_view\_bind\_groups](fn.prepare_mesh2d_view_bind_groups.html "fn bevy::sprite_render::prepare_mesh2d_view_bind_groups")

[prepare\_pending\_mesh\_material2d\_queues](fn.prepare_pending_mesh_material2d_queues.html "fn bevy::sprite_render::prepare_pending_mesh_material2d_queues")

Prepares the [`PendingMeshMaterial2dQueues`](struct.PendingMeshMaterial2dQueues.html "struct bevy::sprite_render::PendingMeshMaterial2dQueues") for a new frame by swapping the current and previous frame queues for each view.

[prepare\_sprite\_image\_bind\_groups](fn.prepare_sprite_image_bind_groups.html "fn bevy::sprite_render::prepare_sprite_image_bind_groups")

[prepare\_sprite\_view\_bind\_groups](fn.prepare_sprite_view_bind_groups.html "fn bevy::sprite_render::prepare_sprite_view_bind_groups")

[queue\_material2d\_meshes](fn.queue_material2d_meshes.html "fn bevy::sprite_render::queue_material2d_meshes")

[queue\_sprites](fn.queue_sprites.html "fn bevy::sprite_render::queue_sprites")

[specialize\_material2d\_meshes](fn.specialize_material2d_meshes.html "fn bevy::sprite_render::specialize_material2d_meshes")

[specialize\_wireframes](fn.specialize_wireframes.html "fn bevy::sprite_render::specialize_wireframes")

[tonemapping\_pipeline\_key](fn.tonemapping_pipeline_key.html "fn bevy::sprite_render::tonemapping_pipeline_key")

[update\_tilemap\_chunk\_indices](fn.update_tilemap_chunk_indices.html "fn bevy::sprite_render::update_tilemap_chunk_indices")

## Type Aliases

[DrawSprite](type.DrawSprite.html "type bevy::sprite_render::DrawSprite")

[`RenderCommand`](../render/render_phase/trait.RenderCommand.html "trait bevy::render::render_phase::RenderCommand") for sprite rendering.

[DrawWireframe2d](type.DrawWireframe2d.html "type bevy::sprite_render::DrawWireframe2d")