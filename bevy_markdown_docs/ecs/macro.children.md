[bevy](../index.html)::[ecs](index.html)

# Macro children 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/hierarchy.rs.html#519)

```rust
macro_rules! children {
    [$($child:expr),*$(,)?] => { ... };
}
```

Returns a [`SpawnRelatedBundle`](spawn/struct.SpawnRelatedBundle.html "struct bevy::ecs::spawn::SpawnRelatedBundle") that will insert the [`Children`](../prelude/struct.Children.html "struct bevy::prelude::Children") component, spawn a [`SpawnableList`](spawn/trait.SpawnableList.html "trait bevy::ecs::spawn::SpawnableList") of entities with given bundles that relate to the [`Children`](../prelude/struct.Children.html "struct bevy::prelude::Children") entity via the [`ChildOf`](../prelude/struct.ChildOf.html "struct bevy::prelude::ChildOf") component, and reserve space in the [`Children`](../prelude/struct.Children.html "struct bevy::prelude::Children") for each spawned entity.

Any additional arguments will be interpreted as bundles to be spawned.

Also see [`related`](../prelude/macro.related.html "macro bevy::prelude::related") for a version of this that works with any [`RelationshipTarget`](../prelude/trait.RelationshipTarget.html "trait bevy::prelude::RelationshipTarget") type.

```rust
let mut world = World::new();
world.spawn((
    Name::new("Root"),
    children![
        Name::new("Child1"),
        (
            Name::new("Child2"),
            children![Name::new("Grandchild")]
        )
    ]
));
```