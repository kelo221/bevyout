[bevy](../index.html)::[asset](index.html)

# Trait VisitAssetDependencies 

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/lib.rs.html#467)

```rust
pub trait VisitAssetDependencies {
    // Required method
    fn visit_dependencies(&self, visit: &mut impl FnMut(UntypedAssetId));
}
```

This trait defines how to visit the dependencies of an asset. For example, a 3D model might require both textures and meshes to be loaded.

Note that this trait is automatically implemented when deriving [`Asset`](../prelude/trait.Asset.html "trait bevy::prelude::Asset").

## Required Methods

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/lib.rs.html#468)

#### fn [visit\_dependencies](#tymethod.visit_dependencies)(&self, visit: &mut impl [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")([UntypedAssetId](enum.UntypedAssetId.html "enum bevy::asset::UntypedAssetId")))

## Dyn Compatibility

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementations on Foreign Types

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/meta.rs.html#211)

### impl [VisitAssetDependencies](trait.VisitAssetDependencies.html "trait bevy::asset::VisitAssetDependencies") for [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/meta.rs.html#212)

#### fn [visit\_dependencies](#tymethod.visit_dependencies)(&self, \_visit: &mut impl [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")([UntypedAssetId](enum.UntypedAssetId.html "enum bevy::asset::UntypedAssetId")))

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/lib.rs.html#491)

### impl [VisitAssetDependencies](trait.VisitAssetDependencies.html "trait bevy::asset::VisitAssetDependencies") for [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[UntypedHandle](../prelude/enum.UntypedHandle.html "enum bevy::prelude::UntypedHandle")\>

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/lib.rs.html#492)

#### fn [visit\_dependencies](#tymethod.visit_dependencies)(&self, visit: &mut impl [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")([UntypedAssetId](enum.UntypedAssetId.html "enum bevy::asset::UntypedAssetId")))

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/lib.rs.html#505)

### impl<A, const N: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\> [VisitAssetDependencies](trait.VisitAssetDependencies.html "trait bevy::asset::VisitAssetDependencies") for \[[Handle](../prelude/enum.Handle.html "enum bevy::prelude::Handle")<A>; [N](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]

where A: [Asset](../prelude/trait.Asset.html "trait bevy::prelude::Asset"),

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/lib.rs.html#506)

#### fn [visit\_dependencies](#tymethod.visit_dependencies)(&self, visit: &mut impl [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")([UntypedAssetId](enum.UntypedAssetId.html "enum bevy::asset::UntypedAssetId")))

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/lib.rs.html#477)

### impl<A> [VisitAssetDependencies](trait.VisitAssetDependencies.html "trait bevy::asset::VisitAssetDependencies") for [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[Handle](../prelude/enum.Handle.html "enum bevy::prelude::Handle")<A>>

where A: [Asset](../prelude/trait.Asset.html "trait bevy::prelude::Asset"),

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/lib.rs.html#478)

#### fn [visit\_dependencies](#tymethod.visit_dependencies)(&self, visit: &mut impl [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")([UntypedAssetId](enum.UntypedAssetId.html "enum bevy::asset::UntypedAssetId")))

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/lib.rs.html#513)

### impl<const N: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\> [VisitAssetDependencies](trait.VisitAssetDependencies.html "trait bevy::asset::VisitAssetDependencies") for \[[UntypedHandle](../prelude/enum.UntypedHandle.html "enum bevy::prelude::UntypedHandle"); [N](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/lib.rs.html#514)

#### fn [visit\_dependencies](#tymethod.visit_dependencies)(&self, visit: &mut impl [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")([UntypedAssetId](enum.UntypedAssetId.html "enum bevy::asset::UntypedAssetId")))

## Implementors

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/lib.rs.html#103)

### impl [VisitAssetDependencies](trait.VisitAssetDependencies.html "trait bevy::asset::VisitAssetDependencies") for [AnimationClip](../prelude/struct.AnimationClip.html "struct bevy::prelude::AnimationClip")

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/graph.rs.html#112)

### impl [VisitAssetDependencies](trait.VisitAssetDependencies.html "trait bevy::asset::VisitAssetDependencies") for [AnimationGraph](../prelude/struct.AnimationGraph.html "struct bevy::prelude::AnimationGraph")

[Source](https://docs.rs/bevy_audio/0.19.0/x86_64-unknown-linux-gnu/src/bevy_audio/audio_source.rs.html#7)

### impl [VisitAssetDependencies](trait.VisitAssetDependencies.html "trait bevy::asset::VisitAssetDependencies") for [AudioSource](../prelude/struct.AudioSource.html "struct bevy::prelude::AudioSource")

[Source](https://docs.rs/bevy_post_process/0.19.0/x86_64-unknown-linux-gnu/src/bevy_post_process/auto_exposure/compensation_curve.rs.html#20)

### impl [VisitAssetDependencies](trait.VisitAssetDependencies.html "trait bevy::asset::VisitAssetDependencies") for [AutoExposureCompensationCurve](../post_process/auto_exposure/struct.AutoExposureCompensationCurve.html "struct bevy::post_process::auto_exposure::AutoExposureCompensationCurve")

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/mesh2d/color_material.rs.html#36)

### impl [VisitAssetDependencies](trait.VisitAssetDependencies.html "trait bevy::asset::VisitAssetDependencies") for [ColorMaterial](../prelude/struct.ColorMaterial.html "struct bevy::prelude::ColorMaterial")

[Source](https://docs.rs/bevy_world_serialization/0.19.0/x86_64-unknown-linux-gnu/src/bevy_world_serialization/dynamic_world.rs.html#24)

### impl [VisitAssetDependencies](trait.VisitAssetDependencies.html "trait bevy::asset::VisitAssetDependencies") for [DynamicWorld](../prelude/struct.DynamicWorld.html "struct bevy::prelude::DynamicWorld")

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/font.rs.html#28)

### impl [VisitAssetDependencies](trait.VisitAssetDependencies.html "trait bevy::asset::VisitAssetDependencies") for [Font](../prelude/struct.Font.html "struct bevy::prelude::Font")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/decal/forward.rs.html#82)

### impl [VisitAssetDependencies](trait.VisitAssetDependencies.html "trait bevy::asset::VisitAssetDependencies") for [ForwardDecalMaterialExt](../pbr/decal/struct.ForwardDecalMaterialExt.html "struct bevy::pbr::decal::ForwardDecalMaterialExt")

[Source](https://docs.rs/bevy_dev_tools/0.19.0/x86_64-unknown-linux-gnu/src/bevy_dev_tools/frame_time_graph/mod.rs.html#80)

### impl [VisitAssetDependencies](trait.VisitAssetDependencies.html "trait bevy::asset::VisitAssetDependencies") for [FrametimeGraphMaterial](../dev_tools/frame_time_graph/struct.FrametimeGraphMaterial.html "struct bevy::dev_tools::frame_time_graph::FrametimeGraphMaterial")

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/lib.rs.html#325)

### impl [VisitAssetDependencies](trait.VisitAssetDependencies.html "trait bevy::asset::VisitAssetDependencies") for [GizmoAsset](../prelude/struct.GizmoAsset.html "struct bevy::prelude::GizmoAsset")

[Source](https://docs.rs/bevy_gltf/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gltf/assets.rs.html#17)

### impl [VisitAssetDependencies](trait.VisitAssetDependencies.html "trait bevy::asset::VisitAssetDependencies") for [Gltf](../prelude/struct.Gltf.html "struct bevy::prelude::Gltf")

[Source](https://docs.rs/bevy_gltf/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gltf/material.rs.html#13)

### impl [VisitAssetDependencies](trait.VisitAssetDependencies.html "trait bevy::asset::VisitAssetDependencies") for [GltfMaterial](../gltf/struct.GltfMaterial.html "struct bevy::gltf::GltfMaterial")

[Source](https://docs.rs/bevy_gltf/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gltf/assets.rs.html#55)

### impl [VisitAssetDependencies](trait.VisitAssetDependencies.html "trait bevy::asset::VisitAssetDependencies") for [GltfMesh](../gltf/struct.GltfMesh.html "struct bevy::gltf::GltfMesh")

[Source](https://docs.rs/bevy_gltf/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gltf/assets.rs.html#97)

### impl [VisitAssetDependencies](trait.VisitAssetDependencies.html "trait bevy::asset::VisitAssetDependencies") for [GltfNode](../gltf/struct.GltfNode.html "struct bevy::gltf::GltfNode")

[Source](https://docs.rs/bevy_gltf/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gltf/assets.rs.html#163)

### impl [VisitAssetDependencies](trait.VisitAssetDependencies.html "trait bevy::asset::VisitAssetDependencies") for [GltfPrimitive](../gltf/struct.GltfPrimitive.html "struct bevy::gltf::GltfPrimitive")

[Source](https://docs.rs/bevy_gltf/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gltf/assets.rs.html#222)

### impl [VisitAssetDependencies](trait.VisitAssetDependencies.html "trait bevy::asset::VisitAssetDependencies") for [GltfSkin](../gltf/struct.GltfSkin.html "struct bevy::gltf::GltfSkin")

[Source](https://docs.rs/bevy_image/0.19.0/x86_64-unknown-linux-gnu/src/bevy_image/image.rs.html#605)

### impl [VisitAssetDependencies](trait.VisitAssetDependencies.html "trait bevy::asset::VisitAssetDependencies") for [Image](../prelude/struct.Image.html "struct bevy::prelude::Image")

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/folder.rs.html#11)

### impl [VisitAssetDependencies](trait.VisitAssetDependencies.html "trait bevy::asset::VisitAssetDependencies") for [LoadedFolder](struct.LoadedFolder.html "struct bevy::asset::LoadedFolder")

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/assets.rs.html#97)

### impl [VisitAssetDependencies](trait.VisitAssetDependencies.html "trait bevy::asset::VisitAssetDependencies") for [LoadedUntypedAsset](struct.LoadedUntypedAsset.html "struct bevy::asset::LoadedUntypedAsset")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/mesh.rs.html#225)

### impl [VisitAssetDependencies](trait.VisitAssetDependencies.html "trait bevy::asset::VisitAssetDependencies") for [Mesh](../prelude/struct.Mesh.html "struct bevy::prelude::Mesh")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/meshlet/asset.rs.html#40)

### impl [VisitAssetDependencies](trait.VisitAssetDependencies.html "trait bevy::asset::VisitAssetDependencies") for [MeshletMesh](../pbr/experimental/meshlet/struct.MeshletMesh.html "struct bevy::pbr::experimental::meshlet::MeshletMesh")

[Source](https://docs.rs/bevy_audio/0.19.0/x86_64-unknown-linux-gnu/src/bevy_audio/pitch.rs.html#10)

### impl [VisitAssetDependencies](trait.VisitAssetDependencies.html "trait bevy::asset::VisitAssetDependencies") for [Pitch](../prelude/struct.Pitch.html "struct bevy::prelude::Pitch")

[Source](https://docs.rs/bevy_light/0.19.0/x86_64-unknown-linux-gnu/src/bevy_light/atmosphere.rs.html#132)

### impl [VisitAssetDependencies](trait.VisitAssetDependencies.html "trait bevy::asset::VisitAssetDependencies") for [ScatteringMedium](../light/atmosphere/struct.ScatteringMedium.html "struct bevy::light::atmosphere::ScatteringMedium")

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/scene_patch.rs.html#113)

### impl [VisitAssetDependencies](trait.VisitAssetDependencies.html "trait bevy::asset::VisitAssetDependencies") for [SceneListPatch](../scene/struct.SceneListPatch.html "struct bevy::scene::SceneListPatch")

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/scene_patch.rs.html#19)

### impl [VisitAssetDependencies](trait.VisitAssetDependencies.html "trait bevy::asset::VisitAssetDependencies") for [ScenePatch](../scene/struct.ScenePatch.html "struct bevy::scene::ScenePatch")

[Source](https://docs.rs/bevy_shader/0.19.0/x86_64-unknown-linux-gnu/src/bevy_shader/shader.rs.html#34)

### impl [VisitAssetDependencies](trait.VisitAssetDependencies.html "trait bevy::asset::VisitAssetDependencies") for [Shader](../prelude/struct.Shader.html "struct bevy::prelude::Shader")

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/storage.rs.html#27)

### impl [VisitAssetDependencies](trait.VisitAssetDependencies.html "trait bevy::asset::VisitAssetDependencies") for [ShaderBuffer](../render/storage/struct.ShaderBuffer.html "struct bevy::render::storage::ShaderBuffer")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/skinning.rs.html#34)

### impl [VisitAssetDependencies](trait.VisitAssetDependencies.html "trait bevy::asset::VisitAssetDependencies") for [SkinnedMeshInverseBindposes](../mesh/skinning/struct.SkinnedMeshInverseBindposes.html "struct bevy::mesh::skinning::SkinnedMeshInverseBindposes")

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/sprite_mesh/sprite_material.rs.html#34)

### impl [VisitAssetDependencies](trait.VisitAssetDependencies.html "trait bevy::asset::VisitAssetDependencies") for [SpriteMaterial](../prelude/struct.SpriteMaterial.html "struct bevy::prelude::SpriteMaterial")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/pbr_material.rs.html#21)

### impl [VisitAssetDependencies](trait.VisitAssetDependencies.html "trait bevy::asset::VisitAssetDependencies") for [StandardMaterial](../prelude/struct.StandardMaterial.html "struct bevy::prelude::StandardMaterial")

[Source](https://docs.rs/bevy_image/0.19.0/x86_64-unknown-linux-gnu/src/bevy_image/texture_atlas.rs.html#92)

### impl [VisitAssetDependencies](trait.VisitAssetDependencies.html "trait bevy::asset::VisitAssetDependencies") for [TextureAtlasLayout](../prelude/struct.TextureAtlasLayout.html "struct bevy::prelude::TextureAtlasLayout")

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/tilemap_chunk/tilemap_chunk_material.rs.html#27)

### impl [VisitAssetDependencies](trait.VisitAssetDependencies.html "trait bevy::asset::VisitAssetDependencies") for [TilemapChunkMaterial](../sprite_render/struct.TilemapChunkMaterial.html "struct bevy::sprite_render::TilemapChunkMaterial")

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/lib.rs.html#499)

### impl [VisitAssetDependencies](trait.VisitAssetDependencies.html "trait bevy::asset::VisitAssetDependencies") for [UntypedAssetId](enum.UntypedAssetId.html "enum bevy::asset::UntypedAssetId")

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/lib.rs.html#485)

### impl [VisitAssetDependencies](trait.VisitAssetDependencies.html "trait bevy::asset::VisitAssetDependencies") for [UntypedHandle](../prelude/enum.UntypedHandle.html "enum bevy::prelude::UntypedHandle")

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/mesh2d/wireframe2d.rs.html#434)

### impl [VisitAssetDependencies](trait.VisitAssetDependencies.html "trait bevy::asset::VisitAssetDependencies") for [Wireframe2dMaterial](../sprite_render/struct.Wireframe2dMaterial.html "struct bevy::sprite_render::Wireframe2dMaterial")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/wireframe.rs.html#910)

### impl [VisitAssetDependencies](trait.VisitAssetDependencies.html "trait bevy::asset::VisitAssetDependencies") for [WireframeMaterial](../pbr/wireframe/struct.WireframeMaterial.html "struct bevy::pbr::wireframe::WireframeMaterial")

[Source](https://docs.rs/bevy_world_serialization/0.19.0/x86_64-unknown-linux-gnu/src/bevy_world_serialization/world_asset.rs.html#22)

### impl [VisitAssetDependencies](trait.VisitAssetDependencies.html "trait bevy::asset::VisitAssetDependencies") for [WorldAsset](../prelude/struct.WorldAsset.html "struct bevy::prelude::WorldAsset")

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/lib.rs.html#537)

### impl<A, K> [VisitAssetDependencies](trait.VisitAssetDependencies.html "trait bevy::asset::VisitAssetDependencies") for [HashMap](../platform/collections/struct.HashMap.html "struct bevy::platform::collections::HashMap")<K, [Handle](../prelude/enum.Handle.html "enum bevy::prelude::Handle")<A>>

where A: [Asset](../prelude/trait.Asset.html "trait bevy::prelude::Asset"),

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/lib.rs.html#471)

### impl<A> [VisitAssetDependencies](trait.VisitAssetDependencies.html "trait bevy::asset::VisitAssetDependencies") for [Handle](../prelude/enum.Handle.html "enum bevy::prelude::Handle")<A>

where A: [Asset](../prelude/trait.Asset.html "trait bevy::prelude::Asset"),

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/extended_material.rs.html#142)

### impl<B, E> [VisitAssetDependencies](trait.VisitAssetDependencies.html "trait bevy::asset::VisitAssetDependencies") for [ExtendedMaterial](../pbr/struct.ExtendedMaterial.html "struct bevy::pbr::ExtendedMaterial")<B, E>

where B: [Material](../prelude/trait.Material.html "trait bevy::prelude::Material"), E: [MaterialExtension](../pbr/trait.MaterialExtension.html "trait bevy::pbr::MaterialExtension"),

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/lib.rs.html#545)

### impl<K> [VisitAssetDependencies](trait.VisitAssetDependencies.html "trait bevy::asset::VisitAssetDependencies") for [HashMap](../platform/collections/struct.HashMap.html "struct bevy::platform::collections::HashMap")<K, [UntypedHandle](../prelude/enum.UntypedHandle.html "enum bevy::prelude::UntypedHandle")\>

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/lib.rs.html#529)

### impl<V> [VisitAssetDependencies](trait.VisitAssetDependencies.html "trait bevy::asset::VisitAssetDependencies") for [HashSet](../platform/collections/struct.HashSet.html "struct bevy::platform::collections::HashSet")<V>

where V: [VisitAssetDependencies](trait.VisitAssetDependencies.html "trait bevy::asset::VisitAssetDependencies"),

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/lib.rs.html#521)

### impl<V> [VisitAssetDependencies](trait.VisitAssetDependencies.html "trait bevy::asset::VisitAssetDependencies") for [Vec](../prelude/struct.Vec.html "struct bevy::prelude::Vec")<V>

where V: [VisitAssetDependencies](trait.VisitAssetDependencies.html "trait bevy::asset::VisitAssetDependencies"),