[bevy](../../index.html)::[material](../index.html)::[specialize](index.html)

# Type Alias BaseSpecializeFn 

[Source](https://docs.rs/bevy_material/0.19.0/x86_64-unknown-linux-gnu/src/bevy_material/specialize.rs.html#18)

```rust
pub type BaseSpecializeFn = fn(&mut World, ErasedMaterialPipelineKey, &MeshVertexBufferLayoutRef, &Arc<MaterialProperties>) -> Result<CachedRenderPipelineId, SpecializedMeshPipelineError>;
```

A type erased function pointer for specializing a material pipeline. The implementation is expected to:

*   Look up the appropriate specializer from the world
*   Downcast the erased key to the concrete key type
*   Call `SpecializedMeshPipelines::specialize` with the specializer and return the resulting pipeline id