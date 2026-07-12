[bevy](../../index.html)::[pbr](../index.html)::[decal](index.html)

# Type Alias ForwardDecalMaterial 

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/decal/forward.rs.html#73)

```rust
pub type ForwardDecalMaterial<B>where
    B: Material, = ExtendedMaterial<B, ForwardDecalMaterialExt>;
```

Type alias for an extended material with a [`ForwardDecalMaterialExt`](struct.ForwardDecalMaterialExt.html "struct bevy::pbr::decal::ForwardDecalMaterialExt") extension.

Make sure to register the [`MaterialPlugin`](../../prelude/struct.MaterialPlugin.html "struct bevy::prelude::MaterialPlugin") for this material in your app setup.

[`StandardMaterial`](../../prelude/struct.StandardMaterial.html "struct bevy::prelude::StandardMaterial") comes with out of the box support for forward decals.

## Aliased Type

```rust
pub struct ForwardDecalMaterial<B>where
    B: Material,{
    pub base: B,
    pub extension: ForwardDecalMaterialExt,
}
```

## Fields

`base: B``extension: [ForwardDecalMaterialExt](struct.ForwardDecalMaterialExt.html "struct bevy::pbr::decal::ForwardDecalMaterialExt")`