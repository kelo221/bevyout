[bevy](../../../index.html)::[gltf](../../index.html)::[gltf\_ext](../index.html)::[mesh](index.html)

# Function primitive\_topology 

[Source](https://docs.rs/bevy_gltf/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gltf/loader/gltf_ext/mesh.rs.html#28)

```rust
pub fn primitive_topology(mode: Mode) -> Result<PrimitiveTopology, GltfError>
```

Maps the `primitive_topology` from glTF to `wgpu`.