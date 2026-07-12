[bevy](../../index.html)::[transform](../index.html)::[commands](index.html)

# Trait BuildChildrenTransformExt 

[Source](https://docs.rs/bevy_transform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_transform/commands.rs.html#9)

```rust
pub trait BuildChildrenTransformExt {
    // Required methods
    fn set_parent_in_place(&mut self, parent: Entity) -> &mut Self;
    fn remove_parent_in_place(&mut self) -> &mut Self;
}
```

Available on **crate feature `bevy-support`** only.

Collection of methods similar to the built-in parenting methods on [`EntityWorldMut`](../../prelude/struct.EntityWorldMut.html "struct bevy::prelude::EntityWorldMut") and [`EntityCommands`](../../prelude/struct.EntityCommands.html "struct bevy::prelude::EntityCommands"), but preserving each entity’s [`GlobalTransform`](../../prelude/struct.GlobalTransform.html "struct bevy::prelude::GlobalTransform").

## Required Methods

[Source](https://docs.rs/bevy_transform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_transform/commands.rs.html#18)

#### fn [set\_parent\_in\_place](#tymethod.set_parent_in_place)(&mut self, parent: [Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity")) -> &mut Self

Change this entity’s parent while preserving this entity’s [`GlobalTransform`](../../prelude/struct.GlobalTransform.html "struct bevy::prelude::GlobalTransform") by updating its [`Transform`](../../prelude/struct.Transform.html "struct bevy::prelude::Transform").

Insert the [`ChildOf`](../../prelude/struct.ChildOf.html "struct bevy::prelude::ChildOf") component directly if you don’t want to also update the [`Transform`](../../prelude/struct.Transform.html "struct bevy::prelude::Transform").

Note that both the hierarchy and transform updates will only execute the next time commands are applied (during [`ApplyDeferred`](../../prelude/struct.ApplyDeferred.html "struct bevy::prelude::ApplyDeferred")).

[Source](https://docs.rs/bevy_transform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_transform/commands.rs.html#28)

#### fn [remove\_parent\_in\_place](#tymethod.remove_parent_in_place)(&mut self) -> &mut Self

Make this entity parentless while preserving this entity’s [`GlobalTransform`](../../prelude/struct.GlobalTransform.html "struct bevy::prelude::GlobalTransform") by updating its [`Transform`](../../prelude/struct.Transform.html "struct bevy::prelude::Transform") to be equal to its current [`GlobalTransform`](../../prelude/struct.GlobalTransform.html "struct bevy::prelude::GlobalTransform").

See [`EntityWorldMut::remove::<ChildOf>`](../../prelude/struct.EntityWorldMut.html#method.remove "method bevy::prelude::EntityWorldMut::remove") or [`EntityCommands::remove::<ChildOf>`](../../prelude/struct.EntityCommands.html#method.remove "method bevy::prelude::EntityCommands::remove") for a method that doesn’t update the [`Transform`](../../prelude/struct.Transform.html "struct bevy::prelude::Transform").

Note that both the hierarchy and transform updates will only execute the next time commands are applied (during [`ApplyDeferred`](../../prelude/struct.ApplyDeferred.html "struct bevy::prelude::ApplyDeferred")).

## Dyn Compatibility

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors

[Source](https://docs.rs/bevy_transform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_transform/commands.rs.html#31)

### impl [BuildChildrenTransformExt](../../prelude/trait.BuildChildrenTransformExt.html "trait bevy::prelude::BuildChildrenTransformExt") for [EntityCommands](../../prelude/struct.EntityCommands.html "struct bevy::prelude::EntityCommands")<'\_>

[Source](https://docs.rs/bevy_transform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_transform/commands.rs.html#45)

### impl [BuildChildrenTransformExt](../../prelude/trait.BuildChildrenTransformExt.html "trait bevy::prelude::BuildChildrenTransformExt") for [EntityWorldMut](../../prelude/struct.EntityWorldMut.html "struct bevy::prelude::EntityWorldMut")<'\_>