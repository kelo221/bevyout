[bevy](../index.html)::[prelude](index.html)

# Macro related 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/spawn.rs.html#487)

```rust
macro_rules! related {
    ($relationship_target:ty [$($child:expr),*$(,)?]) => { ... };
}
```

Returns a [`SpawnRelatedBundle`](../ecs/spawn/struct.SpawnRelatedBundle.html "struct bevy::ecs::spawn::SpawnRelatedBundle") that will insert the given [`RelationshipTarget`](trait.RelationshipTarget.html "trait bevy::prelude::RelationshipTarget"), spawn a [`SpawnableList`](../ecs/spawn/trait.SpawnableList.html "trait bevy::ecs::spawn::SpawnableList") of entities with given bundles that relate to the [`RelationshipTarget`](trait.RelationshipTarget.html "trait bevy::prelude::RelationshipTarget") entity via the [`RelationshipTarget::Relationship`](trait.RelationshipTarget.html#associatedtype.Relationship "associated type bevy::prelude::RelationshipTarget::Relationship") component, and reserve space in the [`RelationshipTarget`](trait.RelationshipTarget.html "trait bevy::prelude::RelationshipTarget") for each spawned entity.

The first argument is the [`RelationshipTarget`](trait.RelationshipTarget.html "trait bevy::prelude::RelationshipTarget") type. Any additional arguments will be interpreted as bundles to be spawned.

Also see [`children`](macro.children.html "macro bevy::prelude::children") for a [`Children`](struct.Children.html "struct bevy::prelude::Children")\-specific equivalent.

```rust
let mut world = World::new();
world.spawn((
    Name::new("Root"),
    related!(Children[
        Name::new("Child1"),
        (
            Name::new("Child2"),
            related!(Children[
                Name::new("Grandchild"),
            ])
        )
    ])
));
```