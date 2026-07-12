[bevy](../index.html)::[pbr](index.html)

# Type Alias DrawPrepass 

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/prepass/mod.rs.html#1477)

```rust
pub type DrawPrepass = (SetItemPipeline, SetPrepassViewBindGroup<0>, SetPrepassViewEmptyBindGroup<1>, SetMeshBindGroup<2>, SetMaterialBindGroup<3>, DrawMesh);
```