[bevy](../index.html)::[sprite\_render](index.html)

# Function extract\_wireframe\_2d\_entities\_needing\_specialization 

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/mesh2d/wireframe2d.rs.html#674-677)

```rust
pub fn extract_wireframe_2d_entities_needing_specialization(
    entities_needing_specialization: Extract<'_, '_, Res<'_, WireframeEntitiesNeedingSpecialization>>,
    dirty_specializations: ResMut<'_, DirtyWireframeSpecializations>,
)
```