[bevy](../../../index.html)::[picking](../../index.html)::[mesh\_picking](../index.html)::[ray\_cast](index.html)

# Function ray\_aabb\_intersection\_3d 

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/mesh_picking/ray_cast/intersections.rs.html#288-292)

```rust
pub fn ray_aabb_intersection_3d(
    ray: Ray3d,
    aabb: &Aabb3d,
    model_to_world: &Affine3A,
) -> Option<f32>
```

Available on **crate feature `mesh_picking`** only.

Checks if the ray intersects with the AABB of a mesh, returning the distance to the point of intersection. The distance is zero if the ray starts inside the AABB.