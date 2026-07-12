[bevy](../index.html)::[asset](index.html)

# Trait Asset 

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/lib.rs.html#452)

```rust
pub trait Asset:
    VisitAssetDependencies
    + TypePath
    + Send
    + Sync
    + 'static { }
```

Declares that this type is an asset, which can be loaded and managed by the [`AssetServer`](../prelude/struct.AssetServer.html "struct bevy::prelude::AssetServer") and stored in [`Assets`](../prelude/struct.Assets.html "struct bevy::prelude::Assets") collections.

Generally, assets are large, complex, and/or expensive to load from disk, and are often authored by artists or designers.

[`TypePath`](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") is largely used for diagnostic purposes, and should almost always be implemented by deriving [`Reflect`](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") on your type. [`VisitAssetDependencies`](trait.VisitAssetDependencies.html "trait bevy::asset::VisitAssetDependencies") is used to track asset dependencies, and an implementation is automatically generated when deriving [`Asset`](../prelude/trait.Asset.html "trait bevy::prelude::Asset").

## Dyn Compatibility

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementations on Foreign Types

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/meta.rs.html#209)

### impl [Asset](../prelude/trait.Asset.html "trait bevy::prelude::Asset") for [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)

## Implementors

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/lib.rs.html#103)

### impl [Asset](../prelude/trait.Asset.html "trait bevy::prelude::Asset") for [AnimationClip](../prelude/struct.AnimationClip.html "struct bevy::prelude::AnimationClip")

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/graph.rs.html#112)

### impl [Asset](../prelude/trait.Asset.html "trait bevy::prelude::Asset") for [AnimationGraph](../prelude/struct.AnimationGraph.html "struct bevy::prelude::AnimationGraph")

[Source](https://docs.rs/bevy_audio/0.19.0/x86_64-unknown-linux-gnu/src/bevy_audio/audio_source.rs.html#7)

### impl [Asset](../prelude/trait.Asset.html "trait bevy::prelude::Asset") for [AudioSource](../prelude/struct.AudioSource.html "struct bevy::prelude::AudioSource")

[Source](https://docs.rs/bevy_post_process/0.19.0/x86_64-unknown-linux-gnu/src/bevy_post_process/auto_exposure/compensation_curve.rs.html#20)

### impl [Asset](../prelude/trait.Asset.html "trait bevy::prelude::Asset") for [AutoExposureCompensationCurve](../post_process/auto_exposure/struct.AutoExposureCompensationCurve.html "struct bevy::post_process::auto_exposure::AutoExposureCompensationCurve")

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/mesh2d/color_material.rs.html#36)

### impl [Asset](../prelude/trait.Asset.html "trait bevy::prelude::Asset") for [ColorMaterial](../prelude/struct.ColorMaterial.html "struct bevy::prelude::ColorMaterial")

[Source](https://docs.rs/bevy_world_serialization/0.19.0/x86_64-unknown-linux-gnu/src/bevy_world_serialization/dynamic_world.rs.html#24)

### impl [Asset](../prelude/trait.Asset.html "trait bevy::prelude::Asset") for [DynamicWorld](../prelude/struct.DynamicWorld.html "struct bevy::prelude::DynamicWorld")

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/font.rs.html#28)

### impl [Asset](../prelude/trait.Asset.html "trait bevy::prelude::Asset") for [Font](../prelude/struct.Font.html "struct bevy::prelude::Font")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/decal/forward.rs.html#82)

### impl [Asset](../prelude/trait.Asset.html "trait bevy::prelude::Asset") for [ForwardDecalMaterialExt](../pbr/decal/struct.ForwardDecalMaterialExt.html "struct bevy::pbr::decal::ForwardDecalMaterialExt")

[Source](https://docs.rs/bevy_dev_tools/0.19.0/x86_64-unknown-linux-gnu/src/bevy_dev_tools/frame_time_graph/mod.rs.html#80)

### impl [Asset](../prelude/trait.Asset.html "trait bevy::prelude::Asset") for [FrametimeGraphMaterial](../dev_tools/frame_time_graph/struct.FrametimeGraphMaterial.html "struct bevy::dev_tools::frame_time_graph::FrametimeGraphMaterial")

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/lib.rs.html#325)

### impl [Asset](../prelude/trait.Asset.html "trait bevy::prelude::Asset") for [GizmoAsset](../prelude/struct.GizmoAsset.html "struct bevy::prelude::GizmoAsset")

[Source](https://docs.rs/bevy_gltf/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gltf/assets.rs.html#17)

### impl [Asset](../prelude/trait.Asset.html "trait bevy::prelude::Asset") for [Gltf](../prelude/struct.Gltf.html "struct bevy::prelude::Gltf")

[Source](https://docs.rs/bevy_gltf/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gltf/material.rs.html#13)

### impl [Asset](../prelude/trait.Asset.html "trait bevy::prelude::Asset") for [GltfMaterial](../gltf/struct.GltfMaterial.html "struct bevy::gltf::GltfMaterial")

[Source](https://docs.rs/bevy_gltf/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gltf/assets.rs.html#55)

### impl [Asset](../prelude/trait.Asset.html "trait bevy::prelude::Asset") for [GltfMesh](../gltf/struct.GltfMesh.html "struct bevy::gltf::GltfMesh")

[Source](https://docs.rs/bevy_gltf/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gltf/assets.rs.html#97)

### impl [Asset](../prelude/trait.Asset.html "trait bevy::prelude::Asset") for [GltfNode](../gltf/struct.GltfNode.html "struct bevy::gltf::GltfNode")

[Source](https://docs.rs/bevy_gltf/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gltf/assets.rs.html#163)

### impl [Asset](../prelude/trait.Asset.html "trait bevy::prelude::Asset") for [GltfPrimitive](../gltf/struct.GltfPrimitive.html "struct bevy::gltf::GltfPrimitive")

[Source](https://docs.rs/bevy_gltf/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gltf/assets.rs.html#222)

### impl [Asset](../prelude/trait.Asset.html "trait bevy::prelude::Asset") for [GltfSkin](../gltf/struct.GltfSkin.html "struct bevy::gltf::GltfSkin")

[Source](https://docs.rs/bevy_image/0.19.0/x86_64-unknown-linux-gnu/src/bevy_image/image.rs.html#605)

### impl [Asset](../prelude/trait.Asset.html "trait bevy::prelude::Asset") for [Image](../prelude/struct.Image.html "struct bevy::prelude::Image")

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/folder.rs.html#11)

### impl [Asset](../prelude/trait.Asset.html "trait bevy::prelude::Asset") for [LoadedFolder](struct.LoadedFolder.html "struct bevy::asset::LoadedFolder")

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/assets.rs.html#97)

### impl [Asset](../prelude/trait.Asset.html "trait bevy::prelude::Asset") for [LoadedUntypedAsset](struct.LoadedUntypedAsset.html "struct bevy::asset::LoadedUntypedAsset")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/mesh.rs.html#225)

### impl [Asset](../prelude/trait.Asset.html "trait bevy::prelude::Asset") for [Mesh](../prelude/struct.Mesh.html "struct bevy::prelude::Mesh")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/meshlet/asset.rs.html#40)

### impl [Asset](../prelude/trait.Asset.html "trait bevy::prelude::Asset") for [MeshletMesh](../pbr/experimental/meshlet/struct.MeshletMesh.html "struct bevy::pbr::experimental::meshlet::MeshletMesh")

[Source](https://docs.rs/bevy_audio/0.19.0/x86_64-unknown-linux-gnu/src/bevy_audio/pitch.rs.html#10)

### impl [Asset](../prelude/trait.Asset.html "trait bevy::prelude::Asset") for [Pitch](../prelude/struct.Pitch.html "struct bevy::prelude::Pitch")

[Source](https://docs.rs/bevy_light/0.19.0/x86_64-unknown-linux-gnu/src/bevy_light/atmosphere.rs.html#132)

### impl [Asset](../prelude/trait.Asset.html "trait bevy::prelude::Asset") for [ScatteringMedium](../light/atmosphere/struct.ScatteringMedium.html "struct bevy::light::atmosphere::ScatteringMedium")

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/scene_patch.rs.html#113)

### impl [Asset](../prelude/trait.Asset.html "trait bevy::prelude::Asset") for [SceneListPatch](../scene/struct.SceneListPatch.html "struct bevy::scene::SceneListPatch")

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/scene_patch.rs.html#19)

### impl [Asset](../prelude/trait.Asset.html "trait bevy::prelude::Asset") for [ScenePatch](../scene/struct.ScenePatch.html "struct bevy::scene::ScenePatch")

[Source](https://docs.rs/bevy_shader/0.19.0/x86_64-unknown-linux-gnu/src/bevy_shader/shader.rs.html#34)

### impl [Asset](../prelude/trait.Asset.html "trait bevy::prelude::Asset") for [Shader](../prelude/struct.Shader.html "struct bevy::prelude::Shader")

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/storage.rs.html#27)

### impl [Asset](../prelude/trait.Asset.html "trait bevy::prelude::Asset") for [ShaderBuffer](../render/storage/struct.ShaderBuffer.html "struct bevy::render::storage::ShaderBuffer")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/skinning.rs.html#34)

### impl [Asset](../prelude/trait.Asset.html "trait bevy::prelude::Asset") for [SkinnedMeshInverseBindposes](../mesh/skinning/struct.SkinnedMeshInverseBindposes.html "struct bevy::mesh::skinning::SkinnedMeshInverseBindposes")

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/sprite_mesh/sprite_material.rs.html#34)

### impl [Asset](../prelude/trait.Asset.html "trait bevy::prelude::Asset") for [SpriteMaterial](../prelude/struct.SpriteMaterial.html "struct bevy::prelude::SpriteMaterial")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/pbr_material.rs.html#21)

### impl [Asset](../prelude/trait.Asset.html "trait bevy::prelude::Asset") for [StandardMaterial](../prelude/struct.StandardMaterial.html "struct bevy::prelude::StandardMaterial")

[Source](https://docs.rs/bevy_image/0.19.0/x86_64-unknown-linux-gnu/src/bevy_image/texture_atlas.rs.html#92)

### impl [Asset](../prelude/trait.Asset.html "trait bevy::prelude::Asset") for [TextureAtlasLayout](../prelude/struct.TextureAtlasLayout.html "struct bevy::prelude::TextureAtlasLayout")

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/tilemap_chunk/tilemap_chunk_material.rs.html#27)

### impl [Asset](../prelude/trait.Asset.html "trait bevy::prelude::Asset") for [TilemapChunkMaterial](../sprite_render/struct.TilemapChunkMaterial.html "struct bevy::sprite_render::TilemapChunkMaterial")

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/mesh2d/wireframe2d.rs.html#434)

### impl [Asset](../prelude/trait.Asset.html "trait bevy::prelude::Asset") for [Wireframe2dMaterial](../sprite_render/struct.Wireframe2dMaterial.html "struct bevy::sprite_render::Wireframe2dMaterial")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/wireframe.rs.html#910)

### impl [Asset](../prelude/trait.Asset.html "trait bevy::prelude::Asset") for [WireframeMaterial](../pbr/wireframe/struct.WireframeMaterial.html "struct bevy::pbr::wireframe::WireframeMaterial")

[Source](https://docs.rs/bevy_world_serialization/0.19.0/x86_64-unknown-linux-gnu/src/bevy_world_serialization/world_asset.rs.html#22)

### impl [Asset](../prelude/trait.Asset.html "trait bevy::prelude::Asset") for [WorldAsset](../prelude/struct.WorldAsset.html "struct bevy::prelude::WorldAsset")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/extended_material.rs.html#142)

### impl<B, E> [Asset](../prelude/trait.Asset.html "trait bevy::prelude::Asset") for [ExtendedMaterial](../pbr/struct.ExtendedMaterial.html "struct bevy::pbr::ExtendedMaterial")<B, E>

where B: [Material](../prelude/trait.Material.html "trait bevy::prelude::Material"), E: [MaterialExtension](../pbr/trait.MaterialExtension.html "trait bevy::pbr::MaterialExtension"),