[bevy](../index.html)::[sprite\_render](index.html)

# Function extract\_entities\_that\_need\_specializations\_removed 

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/mesh2d/material.rs.html#603-607)

```rust
pub fn extract_entities_that_need_specializations_removed<M>(
    entities_needing_specialization: Extract<'_, '_, Res<'_, EntitiesNeedingSpecialization<M>>>,
    dirty_specializations: ResMut<'_, DirtySpecializations>,
)where
    M: Material2d,
```

A system that adds entities that were judged to need their specializations removed to the appropriate table in [`DirtySpecializations`](../render/camera/struct.DirtySpecializations.html "struct bevy::render::camera::DirtySpecializations").