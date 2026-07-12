[bevy](../../index.html)::[pbr](../index.html)::[wireframe](index.html)

# Function extract\_wireframe\_entities\_needing\_specialization 

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/wireframe.rs.html#1280-1283)

```rust
pub fn extract_wireframe_entities_needing_specialization(
    entities_needing_specialization: Extract<'_, '_, Res<'_, WireframeEntitiesNeedingSpecialization>>,
    dirty_wireframe_specializations: ResMut<'_, DirtyWireframeSpecializations>,
)
```