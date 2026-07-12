[bevy](../index.html)::[sprite\_render](index.html)

# Function extract\_wireframe\_2d\_entities\_that\_need\_specializations\_removed 

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/mesh2d/wireframe2d.rs.html#687-690)

```rust
pub fn extract_wireframe_2d_entities_that_need_specializations_removed(
    entities_needing_specialization: Extract<'_, '_, Res<'_, WireframeEntitiesNeedingSpecialization>>,
    dirty_specializations: ResMut<'_, DirtyWireframeSpecializations>,
)
```

A system that adds entities that were judged to need their specializations removed to the appropriate table in [`DirtyWireframeSpecializations`](../render/camera/struct.DirtyWireframeSpecializations.html "struct bevy::render::camera::DirtyWireframeSpecializations").