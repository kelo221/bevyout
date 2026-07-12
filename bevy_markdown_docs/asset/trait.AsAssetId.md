[bevy](../index.html)::[asset](index.html)

# Trait AsAssetId 

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/lib.rs.html#455)

```rust
pub trait AsAssetId: Component {
    type Asset: Asset;

    // Required method
    fn as_asset_id(&self) -> AssetId<Self::Asset>;
}
```

A trait for components that can be used as asset identifiers, e.g. handle wrappers.

## Required Associated Types

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/lib.rs.html#457)

#### type [Asset](#associatedtype.Asset): [Asset](../prelude/trait.Asset.html "trait bevy::prelude::Asset")

The underlying asset type.

## Required Methods

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/lib.rs.html#460)

#### fn [as\_asset\_id](#tymethod.as_asset_id)(&self) -> [AssetId](../prelude/enum.AssetId.html "enum bevy::prelude::AssetId")<Self::[Asset](trait.AsAssetId.html#associatedtype.Asset "type bevy::asset::AsAssetId::Asset")\>

Retrieves the asset id from this component.

## Dyn Compatibility

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors

[Source](https://docs.rs/bevy_world_serialization/0.19.0/x86_64-unknown-linux-gnu/src/bevy_world_serialization/components.rs.html#43)

### impl [AsAssetId](trait.AsAssetId.html "trait bevy::asset::AsAssetId") for [DynamicWorldRoot](../prelude/struct.DynamicWorldRoot.html "struct bevy::prelude::DynamicWorldRoot")

[Source](https://docs.rs/bevy_world_serialization/0.19.0/x86_64-unknown-linux-gnu/src/bevy_world_serialization/components.rs.html#44)

#### type [Asset](#associatedtype.Asset) = [DynamicWorld](../prelude/struct.DynamicWorld.html "struct bevy::prelude::DynamicWorld")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/widget/image.rs.html#147)

### impl [AsAssetId](trait.AsAssetId.html "trait bevy::asset::AsAssetId") for [ImageNode](../prelude/struct.ImageNode.html "struct bevy::prelude::ImageNode")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/widget/image.rs.html#148)

#### type [Asset](#associatedtype.Asset) = [Image](../prelude/struct.Image.html "struct bevy::prelude::Image")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/components.rs.html#59)

### impl [AsAssetId](trait.AsAssetId.html "trait bevy::asset::AsAssetId") for [Mesh2d](../prelude/struct.Mesh2d.html "struct bevy::prelude::Mesh2d")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/components.rs.html#60)

#### type [Asset](#associatedtype.Asset) = [Mesh](../prelude/struct.Mesh.html "struct bevy::prelude::Mesh")

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/mesh2d/wireframe2d.rs.html#450)

### impl [AsAssetId](trait.AsAssetId.html "trait bevy::asset::AsAssetId") for [Mesh2dWireframe](../sprite_render/struct.Mesh2dWireframe.html "struct bevy::sprite_render::Mesh2dWireframe")

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/mesh2d/wireframe2d.rs.html#451)

#### type [Asset](#associatedtype.Asset) = [Wireframe2dMaterial](../sprite_render/struct.Wireframe2dMaterial.html "struct bevy::sprite_render::Wireframe2dMaterial")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/components.rs.html#116)

### impl [AsAssetId](trait.AsAssetId.html "trait bevy::asset::AsAssetId") for [Mesh3d](../prelude/struct.Mesh3d.html "struct bevy::prelude::Mesh3d")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/components.rs.html#117)

#### type [Asset](#associatedtype.Asset) = [Mesh](../prelude/struct.Mesh.html "struct bevy::prelude::Mesh")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/wireframe.rs.html#940)

### impl [AsAssetId](trait.AsAssetId.html "trait bevy::asset::AsAssetId") for [Mesh3dWireframe](../pbr/wireframe/struct.Mesh3dWireframe.html "struct bevy::pbr::wireframe::Mesh3dWireframe")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/wireframe.rs.html#941)

#### type [Asset](#associatedtype.Asset) = [WireframeMaterial](../pbr/wireframe/struct.WireframeMaterial.html "struct bevy::pbr::wireframe::WireframeMaterial")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/skinning.rs.html#24)

### impl [AsAssetId](trait.AsAssetId.html "trait bevy::asset::AsAssetId") for [SkinnedMesh](../mesh/skinning/struct.SkinnedMesh.html "struct bevy::mesh::skinning::SkinnedMesh")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/skinning.rs.html#25)

#### type [Asset](#associatedtype.Asset) = [SkinnedMeshInverseBindposes](../mesh/skinning/struct.SkinnedMeshInverseBindposes.html "struct bevy::mesh::skinning::SkinnedMeshInverseBindposes")

[Source](https://docs.rs/bevy_sprite/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite/sprite.rs.html#157)

### impl [AsAssetId](trait.AsAssetId.html "trait bevy::asset::AsAssetId") for [Sprite](../prelude/struct.Sprite.html "struct bevy::prelude::Sprite")

[Source](https://docs.rs/bevy_sprite/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite/sprite.rs.html#158)

#### type [Asset](#associatedtype.Asset) = [Image](../prelude/struct.Image.html "struct bevy::prelude::Image")

[Source](https://docs.rs/bevy_world_serialization/0.19.0/x86_64-unknown-linux-gnu/src/bevy_world_serialization/components.rs.html#25)

### impl [AsAssetId](trait.AsAssetId.html "trait bevy::asset::AsAssetId") for [WorldAssetRoot](../prelude/struct.WorldAssetRoot.html "struct bevy::prelude::WorldAssetRoot")

[Source](https://docs.rs/bevy_world_serialization/0.19.0/x86_64-unknown-linux-gnu/src/bevy_world_serialization/components.rs.html#26)

#### type [Asset](#associatedtype.Asset) = [WorldAsset](../prelude/struct.WorldAsset.html "struct bevy::prelude::WorldAsset")

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/mesh2d/material.rs.html#232)

### impl<M> [AsAssetId](trait.AsAssetId.html "trait bevy::asset::AsAssetId") for [MeshMaterial2d](../prelude/struct.MeshMaterial2d.html "struct bevy::prelude::MeshMaterial2d")<M>

where M: [Material2d](../sprite_render/trait.Material2d.html "trait bevy::sprite_render::Material2d"),

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/mesh2d/material.rs.html#233)

#### type [Asset](#associatedtype.Asset) = M

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/mesh_material.rs.html#69)

### impl<M> [AsAssetId](trait.AsAssetId.html "trait bevy::asset::AsAssetId") for [MeshMaterial3d](../prelude/struct.MeshMaterial3d.html "struct bevy::prelude::MeshMaterial3d")<M>

where M: [Material](../prelude/trait.Material.html "trait bevy::prelude::Material"),

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/mesh_material.rs.html#70)

#### type [Asset](#associatedtype.Asset) = M