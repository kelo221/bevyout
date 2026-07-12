[bevy](../../index.html)::[pbr](../index.html)

# Module decal 

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/lib.rs.html#44)

Decal rendering.

Decals are a material that render on top of the surface that they’re placed above. They can be used to render signs, paint, snow, impact craters, and other effects on top of surfaces.

## Modules

[clustered](clustered/index.html "mod bevy::pbr::decal::clustered")

Clustered decals, bounding regions that project textures onto surfaces.

## Structs

[ForwardDecal](struct.ForwardDecal.html "struct bevy::pbr::decal::ForwardDecal")

A decal that renders via a 1x1 transparent quad mesh, smoothly alpha-blending with the underlying geometry towards the edges.

[ForwardDecalMaterialExt](struct.ForwardDecalMaterialExt.html "struct bevy::pbr::decal::ForwardDecalMaterialExt")

Material extension for a [`ForwardDecal`](struct.ForwardDecal.html "struct bevy::pbr::decal::ForwardDecal").

[ForwardDecalMaterialExtUniform](struct.ForwardDecalMaterialExtUniform.html "struct bevy::pbr::decal::ForwardDecalMaterialExtUniform")

[ForwardDecalPlugin](struct.ForwardDecalPlugin.html "struct bevy::pbr::decal::ForwardDecalPlugin")

Plugin to render [`ForwardDecal`](struct.ForwardDecal.html "struct bevy::pbr::decal::ForwardDecal")s.

## Type Aliases

[ForwardDecalMaterial](type.ForwardDecalMaterial.html "type bevy::pbr::decal::ForwardDecalMaterial")

Type alias for an extended material with a [`ForwardDecalMaterialExt`](struct.ForwardDecalMaterialExt.html "struct bevy::pbr::decal::ForwardDecalMaterialExt") extension.