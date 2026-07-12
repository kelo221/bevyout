[bevy](../../index.html)::[ecs](../index.html)

# Module spawn 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/lib.rs.html#52)

Entity spawning abstractions, largely focused on spawning related hierarchies of entities. See [`related`](../../prelude/macro.related.html "macro bevy::prelude::related") and [`SpawnRelated`](../../prelude/trait.SpawnRelated.html "trait bevy::prelude::SpawnRelated") for the best entry points into these APIs and examples of how to use them.

## Structs

[Spawn](struct.Spawn.html "struct bevy::ecs::spawn::Spawn")

A wrapper over a [`Bundle`](../../prelude/trait.Bundle.html "trait bevy::prelude::Bundle") indicating that an entity should be spawned with that [`Bundle`](../../prelude/trait.Bundle.html "trait bevy::prelude::Bundle"). This is intended to be used for hierarchical spawning via traits like [`SpawnableList`](trait.SpawnableList.html "trait bevy::ecs::spawn::SpawnableList") and [`SpawnRelated`](../../prelude/trait.SpawnRelated.html "trait bevy::prelude::SpawnRelated").

[SpawnIter](struct.SpawnIter.html "struct bevy::ecs::spawn::SpawnIter")

A [`SpawnableList`](trait.SpawnableList.html "trait bevy::ecs::spawn::SpawnableList") that spawns entities using an iterator of a given [`Bundle`](../../prelude/trait.Bundle.html "trait bevy::prelude::Bundle"):

[SpawnOneRelated](struct.SpawnOneRelated.html "struct bevy::ecs::spawn::SpawnOneRelated")

A [`Bundle`](../../prelude/trait.Bundle.html "trait bevy::prelude::Bundle") that:

[SpawnRelatedBundle](struct.SpawnRelatedBundle.html "struct bevy::ecs::spawn::SpawnRelatedBundle")

A [`Bundle`](../../prelude/trait.Bundle.html "trait bevy::prelude::Bundle") that:

[SpawnWith](struct.SpawnWith.html "struct bevy::ecs::spawn::SpawnWith")

A [`SpawnableList`](trait.SpawnableList.html "trait bevy::ecs::spawn::SpawnableList") that spawns entities using a [`FnOnce`](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce") with a [`RelatedSpawner`](../relationship/struct.RelatedSpawner.html "struct bevy::ecs::relationship::RelatedSpawner") as an argument:

[WithOneRelated](struct.WithOneRelated.html "struct bevy::ecs::spawn::WithOneRelated")

A wrapper over an [`Entity`](../../prelude/struct.Entity.html "struct bevy::prelude::Entity") indicating that an entity should be added. This is intended to be used for hierarchical spawning via traits like [`SpawnableList`](trait.SpawnableList.html "trait bevy::ecs::spawn::SpawnableList") and [`SpawnRelated`](../../prelude/trait.SpawnRelated.html "trait bevy::prelude::SpawnRelated").

[WithRelated](struct.WithRelated.html "struct bevy::ecs::spawn::WithRelated")

A [`SpawnableList`](trait.SpawnableList.html "trait bevy::ecs::spawn::SpawnableList") that links already spawned entities to the root entity via relations of type `I`.

## Traits

[SpawnRelated](trait.SpawnRelated.html "trait bevy::ecs::spawn::SpawnRelated")

[`RelationshipTarget`](../../prelude/trait.RelationshipTarget.html "trait bevy::prelude::RelationshipTarget") methods that create a [`Bundle`](../../prelude/trait.Bundle.html "trait bevy::prelude::Bundle") with a [`DynamicBundle::Effect`](../bundle/trait.DynamicBundle.html#associatedtype.Effect "associated type bevy::ecs::bundle::DynamicBundle::Effect") that:

[SpawnableList](trait.SpawnableList.html "trait bevy::ecs::spawn::SpawnableList")

A spawn-able list of changes to a given [`World`](../../prelude/struct.World.html "struct bevy::prelude::World") and relative to a given [`Entity`](../../prelude/struct.Entity.html "struct bevy::prelude::Entity"). This is generally used for spawning “related” entities, such as children.