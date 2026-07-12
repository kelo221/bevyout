[bevy](../index.html)::[pbr](index.html)

# Function extract\_entities\_needs\_specialization 

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/material.rs.html#761-765)

```rust
pub fn extract_entities_needs_specialization<M>(
    entities_needing_specialization: Extract<'_, '_, Res<'_, EntitiesNeedingSpecialization<M>>>,
    dirty_specializations: ResMut<'_, DirtySpecializations>,
)where
    M: Material,
```