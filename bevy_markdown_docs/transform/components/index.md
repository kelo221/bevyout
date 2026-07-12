[bevy](../../index.html)::[transform](../index.html)

# Module components 

[Source](https://docs.rs/bevy_transform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_transform/lib.rs.html#18)

The basic components of the transform crate

## Structs

[GlobalTransform](struct.GlobalTransform.html "struct bevy::transform::components::GlobalTransform")

[`GlobalTransform`](../../prelude/struct.GlobalTransform.html "struct bevy::prelude::GlobalTransform") is an affine transformation from entity-local coordinates to worldspace coordinates.

[Transform](struct.Transform.html "struct bevy::transform::components::Transform")

Describe the position of an entity. If the entity has a parent, the position is relative to its parent position.

[TransformTreeChanged](struct.TransformTreeChanged.html "struct bevy::transform::components::TransformTreeChanged")

An optimization for transform propagation. This ZST marker component uses change detection to mark all entities of the hierarchy as “dirty” if any of their descendants have a changed `Transform`. If this component is _not_ marked `is_changed()`, propagation will halt.