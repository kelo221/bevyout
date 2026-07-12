[bevy](../../../index.html)::[picking](../../index.html)::[mesh\_picking](../index.html)

# Module ray\_cast 

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/mesh_picking/mod.rs.html#17)

Available on **crate feature `mesh_picking`** only.

Ray casting for meshes.

See the [`MeshRayCast`](../../../prelude/struct.MeshRayCast.html "struct bevy::prelude::MeshRayCast") system parameter for more information.

## Structs

[MeshRayCast](struct.MeshRayCast.html "struct bevy::picking::mesh_picking::ray_cast::MeshRayCast")

Add this ray casting [`SystemParam`](../../../ecs/system/trait.SystemParam.html "trait bevy::ecs::system::SystemParam") to your system to cast rays into the world with an immediate-mode API. Call `cast_ray` to immediately perform a ray cast and get a result.

[MeshRayCastSettings](struct.MeshRayCastSettings.html "struct bevy::picking::mesh_picking::ray_cast::MeshRayCastSettings")

Settings for a ray cast.

[RayCastBackfaces](struct.RayCastBackfaces.html "struct bevy::picking::mesh_picking::ray_cast::RayCastBackfaces")

Disables backface culling for [ray casts](../../../prelude/struct.MeshRayCast.html "struct bevy::prelude::MeshRayCast") on this entity.

[RayMeshHit](struct.RayMeshHit.html "struct bevy::picking::mesh_picking::ray_cast::RayMeshHit")

Hit data for an intersection between a ray and a mesh.

[SimplifiedMesh](struct.SimplifiedMesh.html "struct bevy::picking::mesh_picking::ray_cast::SimplifiedMesh")

A simplified mesh component that can be used for [ray casting](../../../prelude/struct.MeshRayCast.html "struct bevy::prelude::MeshRayCast").

[SimplifiedMeshTemplate](struct.SimplifiedMeshTemplate.html "struct bevy::picking::mesh_picking::ray_cast::SimplifiedMeshTemplate")

## Enums

[Backfaces](enum.Backfaces.html "enum bevy::picking::mesh_picking::ray_cast::Backfaces")

Determines whether backfaces should be culled or included in ray intersection tests.

[RayCastVisibility](enum.RayCastVisibility.html "enum bevy::picking::mesh_picking::ray_cast::RayCastVisibility")

How a ray cast should handle [`Visibility`](../../../prelude/enum.Visibility.html "enum bevy::prelude::Visibility").

## Functions

[ray\_aabb\_intersection\_3d](fn.ray_aabb_intersection_3d.html "fn bevy::picking::mesh_picking::ray_cast::ray_aabb_intersection_3d")

Checks if the ray intersects with the AABB of a mesh, returning the distance to the point of intersection. The distance is zero if the ray starts inside the AABB.

[ray\_mesh\_intersection](fn.ray_mesh_intersection.html "fn bevy::picking::mesh_picking::ray_cast::ray_mesh_intersection")

Checks if a ray intersects a mesh, and returns the nearest intersection if one exists.