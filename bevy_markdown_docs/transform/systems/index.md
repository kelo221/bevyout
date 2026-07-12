[bevy](../../index.html)::[transform](../index.html)

# Module systems 

[Source](https://docs.rs/bevy_transform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_transform/lib.rs.html#33)

Available on **crate feature `bevy-support`** only.

Systems responsible for transform propagation

## Enums

[StaticTransformOptimizations](enum.StaticTransformOptimizations.html "enum bevy::transform::systems::StaticTransformOptimizations")

Configure the behavior of static scene optimizations for [`Transform`](../../prelude/struct.Transform.html "struct bevy::prelude::Transform") propagation.

## Functions

[mark\_dirty\_trees](fn.mark_dirty_trees.html "fn bevy::transform::systems::mark_dirty_trees")

Optimization for static scenes.

[propagate\_parent\_transforms](fn.propagate_parent_transforms.html "fn bevy::transform::systems::propagate_parent_transforms")

Update [`GlobalTransform`](../../prelude/struct.GlobalTransform.html "struct bevy::prelude::GlobalTransform") component of entities based on entity hierarchy and [`Transform`](../../prelude/struct.Transform.html "struct bevy::prelude::Transform") component.

[propagate\_transforms\_for](fn.propagate_transforms_for.html "fn bevy::transform::systems::propagate_transforms_for")

Generic system that propagates transforms, using [`TransformHelper`](../../prelude/struct.TransformHelper.html "struct bevy::prelude::TransformHelper") for any entity matching the filter `F`. Useful for moving and rendering in the same frame.

[sync\_simple\_transforms](fn.sync_simple_transforms.html "fn bevy::transform::systems::sync_simple_transforms")

Update [`GlobalTransform`](../../prelude/struct.GlobalTransform.html "struct bevy::prelude::GlobalTransform") component of entities that aren’t in the hierarchy