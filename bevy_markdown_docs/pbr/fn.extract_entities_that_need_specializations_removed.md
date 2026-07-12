[bevy](../index.html)::[pbr](index.html)

# Function extract\_entities\_that\_need\_specializations\_removed 

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/material.rs.html#778-782)

```rust
pub fn extract_entities_that_need_specializations_removed<M>(
    entities_needing_specialization: Extract<'_, '_, Res<'_, EntitiesNeedingSpecialization<M>>>,
    dirty_specializations: ResMut<'_, DirtySpecializations>,
)where
    M: Material,
```

A system that adds entities that were judged to need their specializations removed to the appropriate table in [`DirtySpecializations`](../render/camera/struct.DirtySpecializations.html "struct bevy::render::camera::DirtySpecializations").