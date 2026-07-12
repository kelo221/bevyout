[bevy](../index.html)

# Crate gltf 

[Source](https://docs.rs/bevy_gltf/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gltf/lib.rs.html#1-307)

Plugin providing an [`AssetLoader`](../asset/trait.AssetLoader.html "trait bevy::asset::AssetLoader") and type definitions for loading glTF 2.0 (a standard 3D scene definition format) files in Bevy.

The [glTF 2.0 specification](https://registry.khronos.org/glTF/specs/2.0/glTF-2.0.html) defines the format of the glTF files.

## Quick Start

Here’s how to spawn a simple glTF scene

```rust
fn spawn_gltf(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        // This is equivalent to "models/FlightHelmet/FlightHelmet.gltf#Scene0"
        // The `#Scene0` label here is very important because it tells bevy to load the first scene in the glTF file.
        // If this isn't specified bevy doesn't know which part of the glTF file to load.
        WorldAssetRoot(asset_server.load(GltfAssetLabel::Scene(0).from_asset("models/FlightHelmet/FlightHelmet.gltf"))),
        // You can use the transform to give it a position
        Transform::from_xyz(2.0, 0.0, -5.0),
    ));
}
```

## Loading parts of a glTF asset

### Using `Gltf`

If you want to access part of the asset, you can load the entire `Gltf` using the `AssetServer`. Once the `Handle<Gltf>` is loaded you can then use it to access named parts of it.

```rust
// Holds the scene handle
#[derive(Resource)]
struct HelmetScene(Handle<Gltf>);

fn load_gltf(mut commands: Commands, asset_server: Res<AssetServer>) {
    let gltf = asset_server.load("models/FlightHelmet/FlightHelmet.gltf");
    commands.insert_resource(HelmetScene(gltf));
}

fn spawn_gltf_objects(
    mut commands: Commands,
    helmet_scene: Res<HelmetScene>,
    gltf_assets: Res<Assets<Gltf>>,
    mut loaded: Local<bool>,
) {
    // Only do this once
    if *loaded {
        return;
    }
    // Wait until the scene is loaded
    let Some(gltf) = gltf_assets.get(&helmet_scene.0) else {
        return;
    };
    *loaded = true;

    // Spawns the first scene in the file
    commands.spawn(WorldAssetRoot(gltf.scenes[0].clone()));

    // Spawns the scene named "Lenses_low"
    commands.spawn((
        WorldAssetRoot(gltf.named_scenes["Lenses_low"].clone()),
        Transform::from_xyz(1.0, 2.0, 3.0),
    ));
}
```

### Asset Labels

The glTF loader let’s you specify labels that let you target specific parts of the glTF.

Be careful when using this feature, if you misspell a label it will simply ignore it without warning.

You can use [`GltfAssetLabel`](../prelude/enum.GltfAssetLabel.html "enum bevy::prelude::GltfAssetLabel") to ensure you are using the correct label.

## Supported KHR Extensions

glTF files may use functionality beyond the base glTF specification, specified as a list of required extensions. The table below shows which of the ratified Khronos extensions are supported by Bevy.

| Extension | Supported | Requires feature |
| --- | --- | --- |
| `KHR_animation_pointer` | ❌ |  |
| `KHR_draco_mesh_compression` | ❌ |  |
| `KHR_lights_punctual` | ✅ |  |
| `KHR_materials_anisotropy` | ✅ | `pbr_anisotropy_texture` |
| `KHR_materials_clearcoat` | ✅ | `pbr_multi_layer_material_textures` |
| `KHR_materials_dispersion` | ❌ |  |
| `KHR_materials_emissive_strength` | ✅ |  |
| `KHR_materials_ior` | ✅ |  |
| `KHR_materials_iridescence` | ❌ |  |
| `KHR_materials_sheen` | ❌ |  |
| `KHR_materials_specular` | ✅ | `pbr_specular_textures` |
| `KHR_materials_transmission` | ✅ | `pbr_transmission_textures` |
| `KHR_materials_unlit` | ✅ |  |
| `KHR_materials_variants` | ❌ |  |
| `KHR_materials_volume` | ✅ |  |
| `KHR_mesh_quantization` | ❌ |  |
| `KHR_texture_basisu` | ❌\* |  |
| `KHR_texture_transform` | ✅\*\* |  |
| `KHR_xmp_json_ld` | ❌ |  |
| `EXT_mesh_gpu_instancing` | ❌ |  |
| `EXT_meshopt_compression` | ❌ |  |
| `EXT_texture_webp` | ❌\* |  |

\*Bevy supports ktx2 and webp formats but doesn’t support the extension’s syntax, see [#19104](https://github.com/bevyengine/bevy/issues/19104).

\*\*`KHR_texture_transform` is only supported on `base_color_texture`, see [#15310](https://github.com/bevyengine/bevy/issues/15310).

See the [glTF Extension Registry](https://github.com/KhronosGroup/glTF/blob/main/extensions/README.md) for more information on extensions.

## Modules

[convert\_coordinates](convert_coordinates/index.html "mod bevy::gltf::convert_coordinates")

Utilities for converting from glTF’s [standard coordinate system](https://registry.khronos.org/glTF/specs/2.0/glTF-2.0.html#coordinate-system-and-units) to Bevy’s.

[extensions](extensions/index.html "mod bevy::gltf::extensions")

glTF extensions defined by the Khronos Group and other vendors

[gltf](gltf/index.html "mod bevy::gltf::gltf")

Re-exports for GLTF

[gltf\_ext](gltf_ext/index.html "mod bevy::gltf::gltf_ext")

Methods to access information from [`gltf`](https://docs.rs/gltf/1.4.1/x86_64-unknown-linux-gnu/gltf/index.html "mod gltf") types

[prelude](prelude/index.html "mod bevy::gltf::prelude")

The glTF prelude.

[vertex\_attributes](vertex_attributes/index.html "mod bevy::gltf::vertex_attributes")

A set of utilities for accessing and converting vertex attribute data

## Structs

[DefaultGltfImageSampler](struct.DefaultGltfImageSampler.html "struct bevy::gltf::DefaultGltfImageSampler")

Stores default [`ImageSamplerDescriptor`](../image/struct.ImageSamplerDescriptor.html "struct bevy::image::ImageSamplerDescriptor") in main world.

[Gltf](struct.Gltf.html "struct bevy::gltf::Gltf")

Representation of a loaded glTF file.

[GltfExtras](struct.GltfExtras.html "struct bevy::gltf::GltfExtras")

Additional untyped data that can be present on most glTF types at the primitive level.

[GltfLoader](struct.GltfLoader.html "struct bevy::gltf::GltfLoader")

Loads glTF files with all of their data as their corresponding bevy representations.

[GltfLoaderSettings](struct.GltfLoaderSettings.html "struct bevy::gltf::GltfLoaderSettings")

Specifies optional settings for processing gltfs at load time. By default, all recognized contents of the gltf will be loaded.

[GltfMaterial](struct.GltfMaterial.html "struct bevy::gltf::GltfMaterial")

Data to build a Gltf Material

[GltfMaterialExtras](struct.GltfMaterialExtras.html "struct bevy::gltf::GltfMaterialExtras")

Additional untyped data that can be present on most glTF types at the material level.

[GltfMaterialName](struct.GltfMaterialName.html "struct bevy::gltf::GltfMaterialName")

The material name of a glTF primitive.

[GltfMesh](struct.GltfMesh.html "struct bevy::gltf::GltfMesh")

A glTF mesh, which may consist of multiple [`GltfPrimitives`](struct.GltfPrimitive.html "struct bevy::gltf::GltfPrimitive") and an optional [`GltfExtras`](../prelude/struct.GltfExtras.html "struct bevy::prelude::GltfExtras").

[GltfMeshExtras](struct.GltfMeshExtras.html "struct bevy::gltf::GltfMeshExtras")

Additional untyped data that can be present on most glTF types at the mesh level.

[GltfMeshName](struct.GltfMeshName.html "struct bevy::gltf::GltfMeshName")

The mesh name of a glTF primitive.

[GltfNode](struct.GltfNode.html "struct bevy::gltf::GltfNode")

A glTF node with all of its child nodes, its [`GltfMesh`](struct.GltfMesh.html "struct bevy::gltf::GltfMesh"), [`Transform`](../prelude/struct.Transform.html "struct bevy::prelude::Transform"), its optional [`GltfSkin`](struct.GltfSkin.html "struct bevy::gltf::GltfSkin") and an optional [`GltfExtras`](../prelude/struct.GltfExtras.html "struct bevy::prelude::GltfExtras").

[GltfPlugin](struct.GltfPlugin.html "struct bevy::gltf::GltfPlugin")

Adds support for glTF file loading to the app.

[GltfPrimitive](struct.GltfPrimitive.html "struct bevy::gltf::GltfPrimitive")

Part of a [`GltfMesh`](struct.GltfMesh.html "struct bevy::gltf::GltfMesh") that consists of a [`Mesh`](../prelude/struct.Mesh.html "struct bevy::prelude::Mesh"), an optional [`GltfMaterial`](struct.GltfMaterial.html "struct bevy::gltf::GltfMaterial") and [`GltfExtras`](../prelude/struct.GltfExtras.html "struct bevy::prelude::GltfExtras").

[GltfSceneExtras](struct.GltfSceneExtras.html "struct bevy::gltf::GltfSceneExtras")

Additional untyped data that can be present on most glTF types at the scene level.

[GltfSceneName](struct.GltfSceneName.html "struct bevy::gltf::GltfSceneName")

The name of a glTF scene.

[GltfSkin](struct.GltfSkin.html "struct bevy::gltf::GltfSkin")

A glTF skin with all of its joint nodes, [`SkinnedMeshInversiveBindposes`](../mesh/skinning/struct.SkinnedMeshInverseBindposes.html "struct bevy::mesh::skinning::SkinnedMeshInverseBindposes") and an optional [`GltfExtras`](../prelude/struct.GltfExtras.html "struct bevy::prelude::GltfExtras").

[MorphTargetNames](struct.MorphTargetNames.html "struct bevy::gltf::MorphTargetNames")

Applications like Blender place shape key names in the glTF extras as a list of target names.

[PrimitiveMorphAttributesIter](struct.PrimitiveMorphAttributesIter.html "struct bevy::gltf::PrimitiveMorphAttributesIter")

An Iterator that iterates over morph target positions, normals, and tangents while optionally handling coordinate conversions. Used when setting morph targets on a `Mesh` while reading them from a primitive.

## Enums

[GltfAssetLabel](enum.GltfAssetLabel.html "enum bevy::gltf::GltfAssetLabel")

Labels that can be used to load part of a glTF

[GltfError](enum.GltfError.html "enum bevy::gltf::GltfError")

An error that occurs when loading a glTF file.

[GltfSkinnedMeshBoundsPolicy](enum.GltfSkinnedMeshBoundsPolicy.html "enum bevy::gltf::GltfSkinnedMeshBoundsPolicy")

Controls the bounds related components that are assigned to skinned mesh entities. These components are used by systems like frustum culling.

## Constants

[MAX\_JOINTS](constant.MAX_JOINTS.html "constant bevy::gltf::MAX_JOINTS")

Must match [`MAX_JOINTS`](https://docs.rs/bevy/latest/bevy/pbr/constant.MAX_JOINTS.html)