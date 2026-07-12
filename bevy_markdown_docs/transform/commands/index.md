[bevy](../../index.html)::[transform](../index.html)

# Module commands 

[Source](https://docs.rs/bevy_transform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_transform/lib.rs.html#16)

Available on **crate feature `bevy-support`** only.

Extension to [`EntityCommands`](../../prelude/struct.EntityCommands.html "struct bevy::prelude::EntityCommands") to modify [`bevy_ecs::hierarchy`](../../ecs/hierarchy/index.html "mod bevy::ecs::hierarchy") hierarchies. while preserving [`GlobalTransform`](../../prelude/struct.GlobalTransform.html "struct bevy::prelude::GlobalTransform").

## Traits

[BuildChildrenTransformExt](trait.BuildChildrenTransformExt.html "trait bevy::transform::commands::BuildChildrenTransformExt")

Collection of methods similar to the built-in parenting methods on [`EntityWorldMut`](../../prelude/struct.EntityWorldMut.html "struct bevy::prelude::EntityWorldMut") and [`EntityCommands`](../../prelude/struct.EntityCommands.html "struct bevy::prelude::EntityCommands"), but preserving each entity’s [`GlobalTransform`](../../prelude/struct.GlobalTransform.html "struct bevy::prelude::GlobalTransform").