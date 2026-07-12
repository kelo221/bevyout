[bevy](../../index.html)::[pbr](../index.html)

# Module prelude 

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/lib.rs.html#93)

The PBR prelude.

This includes the most common types in this crate, re-exported for your convenience.

## Structs

[ContactShadowsPlugin](struct.ContactShadowsPlugin.html "struct bevy::pbr::prelude::ContactShadowsPlugin")

Enables contact shadows for a camera.

[DistanceFog](struct.DistanceFog.html "struct bevy::pbr::prelude::DistanceFog")

Configures the “classic” computer graphics [distance fog](https://en.wikipedia.org/wiki/Distance_fog) effect, in which objects appear progressively more covered in atmospheric haze the further away they are from the camera. Affects meshes rendered via the PBR [`StandardMaterial`](../../prelude/struct.StandardMaterial.html "struct bevy::prelude::StandardMaterial").

[MaterialPlugin](struct.MaterialPlugin.html "struct bevy::pbr::prelude::MaterialPlugin")

Adds the necessary ECS resources and render logic to enable rendering entities using the given [`Material`](../../prelude/trait.Material.html "trait bevy::prelude::Material") asset type.

[MeshMaterial3d](struct.MeshMaterial3d.html "struct bevy::pbr::prelude::MeshMaterial3d")

A [material](../../prelude/trait.Material.html "trait bevy::prelude::Material") used for rendering a [`Mesh3d`](../../prelude/struct.Mesh3d.html "struct bevy::prelude::Mesh3d").

[ScreenSpaceAmbientOcclusionPlugin](struct.ScreenSpaceAmbientOcclusionPlugin.html "struct bevy::pbr::prelude::ScreenSpaceAmbientOcclusionPlugin")

Plugin for screen space ambient occlusion.

[StandardMaterial](struct.StandardMaterial.html "struct bevy::pbr::prelude::StandardMaterial")

A material with “standard” properties used in PBR lighting. Standard property values with pictures here: [https://google.github.io/filament/notes/material\_properties.html](https://google.github.io/filament/notes/material_properties.html).

## Enums

[FogFalloff](enum.FogFalloff.html "enum bevy::pbr::prelude::FogFalloff")

Allows switching between different fog falloff modes, and configuring their parameters.

[ParallaxMappingMethod](enum.ParallaxMappingMethod.html "enum bevy::pbr::prelude::ParallaxMappingMethod")

The [parallax mapping](https://en.wikipedia.org/wiki/Parallax_mapping) method to use to compute depth based on the material’s [`depth_map`](../../prelude/struct.StandardMaterial.html#structfield.depth_map "field bevy::prelude::StandardMaterial::depth_map").

## Traits

[Material](trait.Material.html "trait bevy::pbr::prelude::Material")

Materials are used alongside [`MaterialPlugin`](../../prelude/struct.MaterialPlugin.html "struct bevy::prelude::MaterialPlugin"), [`Mesh3d`](../../prelude/struct.Mesh3d.html "struct bevy::prelude::Mesh3d"), and [`MeshMaterial3d`](../../prelude/struct.MeshMaterial3d.html "struct bevy::prelude::MeshMaterial3d") to spawn entities that are rendered with a specific [`Material`](../../prelude/trait.Material.html "trait bevy::prelude::Material") type. They serve as an easy to use high level way to render [`Mesh3d`](../../prelude/struct.Mesh3d.html "struct bevy::prelude::Mesh3d") entities with custom shader logic.