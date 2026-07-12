[bevy](../index.html)::[sprite\_render](index.html)

# Function extract\_entities\_needs\_specialization 

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/mesh2d/material.rs.html#586-590)

```rust
pub fn extract_entities_needs_specialization<M>(
    entities_needing_specialization: Extract<'_, '_, Res<'_, EntitiesNeedingSpecialization<M>>>,
    dirty_specializations: ResMut<'_, DirtySpecializations>,
)where
    M: Material2d,
```