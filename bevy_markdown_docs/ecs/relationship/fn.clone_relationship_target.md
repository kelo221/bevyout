[bevy](../../index.html)::[ecs](../index.html)::[relationship](index.html)

# Function clone\_relationship\_target 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/relationship/mod.rs.html#372-376)

```rust
pub fn clone_relationship_target<T>(
    component: &T,
    cloned: &mut T,
    context: &mut ComponentCloneCtx<'_, '_>,
)where
    T: RelationshipTarget,
```

The “clone behavior” for [`RelationshipTarget`](../../prelude/trait.RelationshipTarget.html "trait bevy::prelude::RelationshipTarget"). The [`RelationshipTarget`](../../prelude/trait.RelationshipTarget.html "trait bevy::prelude::RelationshipTarget") will be populated with the proper components when the corresponding [`Relationship`](trait.Relationship.html "trait bevy::ecs::relationship::Relationship") sources of truth are inserted. Cloning the actual entities in the original [`RelationshipTarget`](../../prelude/trait.RelationshipTarget.html "trait bevy::prelude::RelationshipTarget") would result in duplicates, so we don’t do that!

This will also queue up clones of the relationship sources if the [`EntityCloner`](../entity/struct.EntityCloner.html "struct bevy::ecs::entity::EntityCloner") is configured to spawn recursively.