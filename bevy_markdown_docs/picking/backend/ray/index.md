[bevy](../../../index.html)::[picking](../../index.html)::[backend](../index.html)

# Module ray 

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/backend.rs.html#231)

Types and systems for constructing rays from cameras and pointers.

## Structs

[RayId](struct.RayId.html "struct bevy::picking::backend::ray::RayId")

Identifies a ray constructed from some (pointer, camera) combination. A pointer can be over multiple cameras, which is why a single pointer may have multiple rays.

[RayMap](struct.RayMap.html "struct bevy::picking::backend::ray::RayMap")

A map from [`RayId`](struct.RayId.html "struct bevy::picking::backend::ray::RayId") to [`Ray3d`](../../../prelude/struct.Ray3d.html "struct bevy::prelude::Ray3d").