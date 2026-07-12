[bevy](../../index.html)::[pbr](../index.html)::[wireframe](index.html)

# Type Alias DrawWireframe3dThin 

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/wireframe.rs.html#675)

```rust
pub type DrawWireframe3dThin = (SetItemPipeline, SetMeshViewBindGroup<0>, SetMeshViewBindingArrayBindGroup<1>, SetMeshBindGroup<2>, SetWireframe3dThinImmediates, DrawMesh);
```

Draw wireframes with `PolygonMode::Line`, i.e. the fast path.