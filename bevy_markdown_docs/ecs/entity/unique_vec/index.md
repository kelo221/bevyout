[bevy](../../../index.html)::[ecs](../../index.html)::[entity](../index.html)

# Module unique\_vec 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/mod.rs.html#112)

A wrapper around entity [`Vec`](../../../prelude/struct.Vec.html "struct bevy::prelude::Vec")s with a uniqueness invariant.

## Structs

[UniqueEntityEquivalentVec](struct.UniqueEntityEquivalentVec.html "struct bevy::ecs::entity::unique_vec::UniqueEntityEquivalentVec")

A `Vec` that contains only unique entities.

## Type Aliases

[Drain](type.Drain.html "type bevy::ecs::entity::unique_vec::Drain")

A draining iterator for [`UniqueEntityEquivalentVec<T>`](../struct.UniqueEntityEquivalentVec.html "struct bevy::ecs::entity::UniqueEntityEquivalentVec").

[IntoIter](type.IntoIter.html "type bevy::ecs::entity::unique_vec::IntoIter")

An iterator that moves out of a vector.

[Splice](type.Splice.html "type bevy::ecs::entity::unique_vec::Splice")

A splicing iterator for [`UniqueEntityEquivalentVec`](../struct.UniqueEntityEquivalentVec.html "struct bevy::ecs::entity::UniqueEntityEquivalentVec").

[UniqueEntityVec](type.UniqueEntityVec.html "type bevy::ecs::entity::unique_vec::UniqueEntityVec")

A `Vec` that contains only unique [`Entity`](../../../prelude/struct.Entity.html "struct bevy::prelude::Entity").