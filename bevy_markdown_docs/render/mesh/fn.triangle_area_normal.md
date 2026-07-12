[bevy](../../index.html)::[render](../index.html)::[mesh](index.html)

# Function triangle\_area\_normal 

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/vertex.rs.html#222)

```rust
pub fn triangle_area_normal(a: [f32; 3], b: [f32; 3], c: [f32; 3]) -> [f32; 3]
```

Compute a vector whose direction is the normal of the triangle formed by points a, b, c, and whose magnitude is double the area of the triangle. This is useful for computing smooth normals where the contributing normals are proportionate to the areas of the triangles as [discussed here](https://iquilezles.org/articles/normals/).

Question: Why double the area? Because the area of a triangle _A_ is determined by this equation:

_A = |(b - a) x (c - a)| / 2_

By computing _2 A_ we avoid a division operation, and when calculating the the sum of these vectors which are then normalized, a constant multiple has no effect.