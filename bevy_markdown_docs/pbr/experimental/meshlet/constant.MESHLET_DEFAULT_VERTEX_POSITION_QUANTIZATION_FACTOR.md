[bevy](../../../index.html)::[pbr](../../index.html)::[experimental](../index.html)::[meshlet](index.html)

# Constant MESHLET\_DEFAULT\_VERTEX\_POSITION\_QUANTIZATION\_FACTOR 

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/meshlet/from_mesh.rs.html#35)

```rust
pub const MESHLET_DEFAULT_VERTEX_POSITION_QUANTIZATION_FACTOR: u8 = 4; // 4u8
```

Available on **crate feature `meshlet`** only.

Default vertex position quantization factor for use with [`MeshletMesh::from_mesh`](struct.MeshletMesh.html#method.from_mesh "associated function bevy::pbr::experimental::meshlet::MeshletMesh::from_mesh").

Snaps vertices to the nearest 1/16th of a centimeter (1/2^4).