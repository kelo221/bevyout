[bevy](../../index.html)::[ecs](../index.html)::[spawn](index.html)

# Trait SpawnRelated 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/spawn.rs.html#416)

```rust
pub trait SpawnRelated: RelationshipTarget {
    // Required methods
    fn spawn<L>(list: L) -> SpawnRelatedBundle<Self::Relationship, L>
       where L: SpawnableList<Self::Relationship>;
    fn spawn_one<B>(bundle: B) -> SpawnOneRelated<Self::Relationship, B>
       where B: Bundle;
}
```

[`RelationshipTarget`](../../prelude/trait.RelationshipTarget.html "trait bevy::prelude::RelationshipTarget") methods that create a [`Bundle`](../../prelude/trait.Bundle.html "trait bevy::prelude::Bundle") with a [`DynamicBundle::Effect`](../bundle/trait.DynamicBundle.html#associatedtype.Effect "associated type bevy::ecs::bundle::DynamicBundle::Effect") that:

1.  Contains the [`RelationshipTarget`](../../prelude/trait.RelationshipTarget.html "trait bevy::prelude::RelationshipTarget") component, pre-allocated with the necessary space for spawned entities.
2.  Spawns an entity (or a list of entities) that relate to the entity the [`Bundle`](../../prelude/trait.Bundle.html "trait bevy::prelude::Bundle") is added to via the [`RelationshipTarget::Relationship`](../../prelude/trait.RelationshipTarget.html#associatedtype.Relationship "associated type bevy::prelude::RelationshipTarget::Relationship").

## Required Methods

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/spawn.rs.html#421-423)

#### fn [spawn](#tymethod.spawn)<L>(list: L) -> [SpawnRelatedBundle](struct.SpawnRelatedBundle.html "struct bevy::ecs::spawn::SpawnRelatedBundle")<Self::[Relationship](../../prelude/trait.RelationshipTarget.html#associatedtype.Relationship "type bevy::prelude::RelationshipTarget::Relationship"), L>

where L: [SpawnableList](trait.SpawnableList.html "trait bevy::ecs::spawn::SpawnableList")<Self::[Relationship](../../prelude/trait.RelationshipTarget.html#associatedtype.Relationship "type bevy::prelude::RelationshipTarget::Relationship")\>,

Returns a [`Bundle`](../../prelude/trait.Bundle.html "trait bevy::prelude::Bundle") containing this [`RelationshipTarget`](../../prelude/trait.RelationshipTarget.html "trait bevy::prelude::RelationshipTarget") component. It also spawns a [`SpawnableList`](trait.SpawnableList.html "trait bevy::ecs::spawn::SpawnableList") of entities, each related to the bundle’s entity via [`RelationshipTarget::Relationship`](../../prelude/trait.RelationshipTarget.html#associatedtype.Relationship "associated type bevy::prelude::RelationshipTarget::Relationship"). The [`RelationshipTarget`](../../prelude/trait.RelationshipTarget.html "trait bevy::prelude::RelationshipTarget") (when possible) will pre-allocate space for the related entities.

See [`Spawn`](../../prelude/struct.Spawn.html "struct bevy::prelude::Spawn"), [`SpawnIter`](../../prelude/struct.SpawnIter.html "struct bevy::prelude::SpawnIter"), [`SpawnWith`](../../prelude/struct.SpawnWith.html "struct bevy::prelude::SpawnWith"), [`WithRelated`](../../prelude/struct.WithRelated.html "struct bevy::prelude::WithRelated") and [`WithOneRelated`](../../prelude/struct.WithOneRelated.html "struct bevy::prelude::WithOneRelated") for usage examples.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/spawn.rs.html#439)

#### fn [spawn\_one](#tymethod.spawn_one)<B>(bundle: B) -> [SpawnOneRelated](struct.SpawnOneRelated.html "struct bevy::ecs::spawn::SpawnOneRelated")<Self::[Relationship](../../prelude/trait.RelationshipTarget.html#associatedtype.Relationship "type bevy::prelude::RelationshipTarget::Relationship"), B>

where B: [Bundle](../../prelude/trait.Bundle.html "trait bevy::prelude::Bundle"),

Returns a [`Bundle`](../../prelude/trait.Bundle.html "trait bevy::prelude::Bundle") containing this [`RelationshipTarget`](../../prelude/trait.RelationshipTarget.html "trait bevy::prelude::RelationshipTarget") component. It also spawns a single entity containing [`Bundle`](../../prelude/trait.Bundle.html "trait bevy::prelude::Bundle") that is related to the bundle’s entity via [`RelationshipTarget::Relationship`](../../prelude/trait.RelationshipTarget.html#associatedtype.Relationship "associated type bevy::prelude::RelationshipTarget::Relationship").

```rust
let mut world = World::new();
world.spawn((
    Name::new("Root"),
    Children::spawn_one(Name::new("Child")),
));
```

## Dyn Compatibility

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/spawn.rs.html#442)

### impl<T> [SpawnRelated](../../prelude/trait.SpawnRelated.html "trait bevy::prelude::SpawnRelated") for T

where T: [RelationshipTarget](../../prelude/trait.RelationshipTarget.html "trait bevy::prelude::RelationshipTarget"),