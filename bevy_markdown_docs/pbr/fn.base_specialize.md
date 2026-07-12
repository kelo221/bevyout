[bevy](../index.html)::[pbr](index.html)

# Function base\_specialize 

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/material.rs.html#1447-1452)

```rust
pub fn base_specialize(
    world: &mut World,
    key: ErasedMaterialPipelineKey,
    layout: &MeshVertexBufferLayoutRef,
    properties: &Arc<MaterialProperties>,
) -> Result<CachedRenderPipelineId, SpecializedMeshPipelineError>
```