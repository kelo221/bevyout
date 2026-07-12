[bevy](../../index.html)::[pbr](../index.html)::[wireframe](index.html)

# Function extract\_wireframe\_entities\_that\_need\_specializations\_removed 

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/wireframe.rs.html#1296-1299)

```rust
pub fn extract_wireframe_entities_that_need_specializations_removed(
    entities_needing_specialization: Extract<'_, '_, Res<'_, WireframeEntitiesNeedingSpecialization>>,
    dirty_wireframe_specializations: ResMut<'_, DirtyWireframeSpecializations>,
)
```

A system that adds entities that were judged to need their wireframe specializations removed to the appropriate table in [`DirtyWireframeSpecializations`](../../render/camera/struct.DirtyWireframeSpecializations.html "struct bevy::render::camera::DirtyWireframeSpecializations").