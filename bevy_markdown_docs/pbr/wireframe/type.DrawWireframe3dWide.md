[bevy](../../index.html)::[pbr](../index.html)::[wireframe](index.html)

# Type Alias DrawWireframe3dWide 

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/wireframe.rs.html#685)

```rust
pub type DrawWireframe3dWide = (SetItemPipeline, SetMeshViewBindGroup<0>, SetMeshViewBindingArrayBindGroup<1>, SetMeshBindGroup<2>, SetWireframe3dWideBindGroup, SetWireframe3dWideImmediates, DrawWireframeMeshPulled);
```

Draw wireframes using vertex pulling for wide lines or quad topology.