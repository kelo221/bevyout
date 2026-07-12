[bevy](../../index.html)::[camera](../index.html)

# Module primitives 

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/lib.rs.html#5)

## Structs

[Aabb](struct.Aabb.html "struct bevy::camera::primitives::Aabb")

An axis-aligned bounding box, defined by:

[CascadesFrusta](struct.CascadesFrusta.html "struct bevy::camera::primitives::CascadesFrusta")

[CubeMapFace](struct.CubeMapFace.html "struct bevy::camera::primitives::CubeMapFace")

[CubemapFrusta](struct.CubemapFrusta.html "struct bevy::camera::primitives::CubemapFrusta")

[Frustum](struct.Frustum.html "struct bevy::camera::primitives::Frustum")

A frustum component is used on an entity with a [`Camera`](../../prelude/struct.Camera.html "struct bevy::prelude::Camera") component to determine which entities will be considered for rendering by this camera. All entities with an [`Aabb`](struct.Aabb.html "struct bevy::camera::primitives::Aabb") component that are not contained by (or crossing the boundary of) the frustum will not be rendered, and not be used in rendering computations.

[Sphere](struct.Sphere.html "struct bevy::camera::primitives::Sphere")

A sphere, defined by a center and a radius.

## Enums

[CubemapLayout](enum.CubemapLayout.html "enum bevy::camera::primitives::CubemapLayout")

Cubemap layout defines the order of images in a packed cubemap image.

## Constants

[CUBE\_MAP\_FACES](constant.CUBE_MAP_FACES.html "constant bevy::camera::primitives::CUBE_MAP_FACES")

## Traits

[MeshAabb](trait.MeshAabb.html "trait bevy::camera::primitives::MeshAabb")

## Functions

[face\_index\_to\_name](fn.face_index_to_name.html "fn bevy::camera::primitives::face_index_to_name")