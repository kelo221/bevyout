[bevy](../../index.html)::[core\_pipeline](../index.html)

# Module core\_3d 

[Source](https://docs.rs/bevy_core_pipeline/0.19.0/x86_64-unknown-linux-gnu/src/bevy_core_pipeline/lib.rs.html#11)

## Structs

[AlphaMask3d](struct.AlphaMask3d.html "struct bevy::core_pipeline::core_3d::AlphaMask3d")

[Core3dPlugin](struct.Core3dPlugin.html "struct bevy::core_pipeline::core_3d::Core3dPlugin")

[Opaque3d](struct.Opaque3d.html "struct bevy::core_pipeline::core_3d::Opaque3d")

Opaque 3D [`BinnedPhaseItem`](../../render/render_phase/trait.BinnedPhaseItem.html "trait bevy::render::render_phase::BinnedPhaseItem")s.

[Opaque3dBatchSetKey](struct.Opaque3dBatchSetKey.html "struct bevy::core_pipeline::core_3d::Opaque3dBatchSetKey")

Information that must be identical in order to place opaque meshes in the same _batch set_.

[Opaque3dBinKey](struct.Opaque3dBinKey.html "struct bevy::core_pipeline::core_3d::Opaque3dBinKey")

Data that must be identical in order to _batch_ phase items together.

[Transparent3d](struct.Transparent3d.html "struct bevy::core_pipeline::core_3d::Transparent3d")

## Enums

[TransparentSortingInfo3d](enum.TransparentSortingInfo3d.html "enum bevy::core_pipeline::core_3d::TransparentSortingInfo3d")

Information needed to perform a depth sort.

## Constants

[CORE\_3D\_DEPTH\_FORMAT](constant.CORE_3D_DEPTH_FORMAT.html "constant bevy::core_pipeline::core_3d::CORE_3D_DEPTH_FORMAT")

[DEPTH\_PREPASS\_TEXTURE\_SUPPORTED](constant.DEPTH_PREPASS_TEXTURE_SUPPORTED.html "constant bevy::core_pipeline::core_3d::DEPTH_PREPASS_TEXTURE_SUPPORTED")`webgpu` or non-WebAssembly

True if multisampled depth textures are supported on this platform.

## Functions

[check\_msaa](fn.check_msaa.html "fn bevy::core_pipeline::core_3d::check_msaa")

[extract\_camera\_prepass\_phase](fn.extract_camera_prepass_phase.html "fn bevy::core_pipeline::core_3d::extract_camera_prepass_phase")

[extract\_core\_3d\_camera\_phases](fn.extract_core_3d_camera_phases.html "fn bevy::core_pipeline::core_3d::extract_core_3d_camera_phases")

[main\_opaque\_pass\_3d](fn.main_opaque_pass_3d.html "fn bevy::core_pipeline::core_3d::main_opaque_pass_3d")

[main\_transparent\_pass\_3d](fn.main_transparent_pass_3d.html "fn bevy::core_pipeline::core_3d::main_transparent_pass_3d")

[prepare\_core\_3d\_depth\_textures](fn.prepare_core_3d_depth_textures.html "fn bevy::core_pipeline::core_3d::prepare_core_3d_depth_textures")

[prepare\_prepass\_textures](fn.prepare_prepass_textures.html "fn bevy::core_pipeline::core_3d::prepare_prepass_textures")