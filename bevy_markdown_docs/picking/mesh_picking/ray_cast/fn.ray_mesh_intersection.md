[bevy](../../../index.html)::[picking](../../index.html)::[mesh\_picking](../index.html)::[ray\_cast](index.html)

# Function ray\_mesh\_intersection 

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/mesh_picking/ray_cast/intersections.rs.html#80-90)

```rust
pub fn ray_mesh_intersection<I>(
    ray: Ray3d,
    mesh_transform: &Affine3A,
    positions: &[[f32; 3]],
    vertex_normals: Option<&[[f32; 3]]>,
    indices: Option<&[I]>,
    uvs: Option<&[[f32; 2]]>,
    backface_culling: Backfaces,
) -> Option<RayMeshHit>where
    I: TryInto<usize> + Clone + Copy,
```

Available on **crate feature `mesh_picking`** only.

Checks if a ray intersects a mesh, and returns the nearest intersection if one exists.