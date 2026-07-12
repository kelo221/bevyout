[bevy](../../index.html)::[core\_pipeline](../index.html)

# Module core\_2d 

[Source](https://docs.rs/bevy_core_pipeline/0.19.0/x86_64-unknown-linux-gnu/src/bevy_core_pipeline/lib.rs.html#10)

## Structs

[AlphaMask2d](struct.AlphaMask2d.html "struct bevy::core_pipeline::core_2d::AlphaMask2d")

Alpha mask 2D [`BinnedPhaseItem`](../../render/render_phase/trait.BinnedPhaseItem.html "trait bevy::render::render_phase::BinnedPhaseItem")s.

[AlphaMask2dBinKey](struct.AlphaMask2dBinKey.html "struct bevy::core_pipeline::core_2d::AlphaMask2dBinKey")

Data that must be identical in order to batch phase items together.

[BatchSetKey2d](struct.BatchSetKey2d.html "struct bevy::core_pipeline::core_2d::BatchSetKey2d")

2D meshes aren’t currently multi-drawn together, so this batch set key only stores whether the mesh is indexed.

[Core2dPlugin](struct.Core2dPlugin.html "struct bevy::core_pipeline::core_2d::Core2dPlugin")

[Opaque2d](struct.Opaque2d.html "struct bevy::core_pipeline::core_2d::Opaque2d")

Opaque 2D [`BinnedPhaseItem`](../../render/render_phase/trait.BinnedPhaseItem.html "trait bevy::render::render_phase::BinnedPhaseItem")s.

[Opaque2dBinKey](struct.Opaque2dBinKey.html "struct bevy::core_pipeline::core_2d::Opaque2dBinKey")

Data that must be identical in order to batch phase items together.

[Transparent2d](struct.Transparent2d.html "struct bevy::core_pipeline::core_2d::Transparent2d")

Transparent 2D [`SortedPhaseItem`](../../render/render_phase/trait.SortedPhaseItem.html "trait bevy::render::render_phase::SortedPhaseItem")s.

## Constants

[CORE\_2D\_DEPTH\_FORMAT](constant.CORE_2D_DEPTH_FORMAT.html "constant bevy::core_pipeline::core_2d::CORE_2D_DEPTH_FORMAT")

## Functions

[extract\_core\_2d\_camera\_phases](fn.extract_core_2d_camera_phases.html "fn bevy::core_pipeline::core_2d::extract_core_2d_camera_phases")

[main\_opaque\_pass\_2d](fn.main_opaque_pass_2d.html "fn bevy::core_pipeline::core_2d::main_opaque_pass_2d")

[main\_transparent\_pass\_2d](fn.main_transparent_pass_2d.html "fn bevy::core_pipeline::core_2d::main_transparent_pass_2d")

[prepare\_core\_2d\_depth\_textures](fn.prepare_core_2d_depth_textures.html "fn bevy::core_pipeline::core_2d::prepare_core_2d_depth_textures")